//! Single-instance process lock management.

use crate::config::AppPaths;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

/// Lifetime guard that holds the OS file lock for the current process.
#[derive(Debug)]
pub struct SingleInstanceLock {
    file: File,
    lock_file: PathBuf,
}

impl Drop for SingleInstanceLock {
    fn drop(&mut self) {
        let _ = self.file.sync_all();
        let _ = std::fs::remove_file(&self.lock_file);
    }
}

/// Startup error for single-instance protection.
#[derive(Debug, thiserror::Error)]
pub enum SingleInstanceError {
    #[error("another game instance is already running (lock file: {lock_file:?})")]
    AlreadyRunning { lock_file: PathBuf },
    #[error("failed to acquire startup lock: {0}")]
    Io(#[from] std::io::Error),
}

/// Acquire a global lock for the process unless multi-instance mode is enabled.
pub fn acquire_single_instance_lock(
    paths: &AppPaths,
    allow_multiple_instances: bool,
) -> Result<Option<SingleInstanceLock>, SingleInstanceError> {
    if allow_multiple_instances {
        return Ok(None);
    }

    let file = match OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(&paths.instance_lock_file)
    {
        Ok(file) => file,
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            if try_clear_stale_lock(&paths.instance_lock_file).map_err(SingleInstanceError::Io)? {
                OpenOptions::new()
                    .create_new(true)
                    .read(true)
                    .write(true)
                    .open(&paths.instance_lock_file)
                    .map_err(|retry_err| {
                        if retry_err.kind() == std::io::ErrorKind::AlreadyExists {
                            SingleInstanceError::AlreadyRunning {
                                lock_file: paths.instance_lock_file.clone(),
                            }
                        } else {
                            SingleInstanceError::Io(retry_err)
                        }
                    })?
            } else {
                return Err(SingleInstanceError::AlreadyRunning {
                    lock_file: paths.instance_lock_file.clone(),
                });
            }
        }
        Err(e) => return Err(SingleInstanceError::Io(e)),
    };

    write_lock_metadata(file, &paths.instance_lock_file).map(Some)
}

fn try_clear_stale_lock(lock_file: &std::path::Path) -> std::io::Result<bool> {
    let content = std::fs::read_to_string(lock_file)?;
    let Some(pid) = parse_pid_from_lock(&content) else {
        return Ok(false);
    };

    if is_process_alive(pid) {
        return Ok(false);
    }

    std::fs::remove_file(lock_file)?;
    Ok(true)
}

fn parse_pid_from_lock(content: &str) -> Option<u32> {
    content
        .lines()
        .find_map(|line| line.strip_prefix("pid="))
        .and_then(|pid| pid.trim().parse::<u32>().ok())
}

#[cfg(windows)]
fn is_process_alive(pid: u32) -> bool {
    // Minimal FFI to avoid external crate dependency.
    type Handle = *mut std::ffi::c_void;
    const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 0x1000;
    const STILL_ACTIVE: u32 = 259;

    unsafe extern "system" {
        fn OpenProcess(dwDesiredAccess: u32, bInheritHandle: i32, dwProcessId: u32) -> Handle;
        fn GetExitCodeProcess(hProcess: Handle, lpExitCode: *mut u32) -> i32;
        fn CloseHandle(hObject: Handle) -> i32;
    }

    let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid) };
    if handle.is_null() {
        return false;
    }

    let mut exit_code: u32 = 0;
    let ok = unsafe { GetExitCodeProcess(handle, &mut exit_code) };
    unsafe { CloseHandle(handle) };

    ok != 0 && exit_code == STILL_ACTIVE
}

#[cfg(not(windows))]
fn is_process_alive(pid: u32) -> bool {
    std::path::Path::new("/proc").join(pid.to_string()).exists()
}

fn write_lock_metadata(
    mut file: File,
    lock_file: &std::path::Path,
) -> Result<SingleInstanceLock, SingleInstanceError> {
    let pid = std::process::id();
    writeln!(&mut file, "pid={pid}").map_err(SingleInstanceError::Io)?;
    file.flush().map_err(SingleInstanceError::Io)?;
    Ok(SingleInstanceLock {
        file,
        lock_file: lock_file.to_path_buf(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppPaths;
    use crate::config::metadata::{LOCK_FILENAME, LOG_FILENAME, SETTINGS_FILENAME};

    #[test]
    fn lock_is_skipped_when_multiple_instances_enabled() {
        let paths = AppPaths::from_env();
        let result = acquire_single_instance_lock(&paths, true).expect("lock acquire should pass");
        assert!(result.is_none());
    }

    #[test]
    fn second_lock_fails_while_first_is_alive() {
        let unique = format!(
            "planetarium-single-instance-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system time should be after epoch")
                .as_nanos()
        );
        let data_dir = std::env::temp_dir().join(unique);
        std::fs::create_dir_all(&data_dir).expect("temp lock dir should be created");

        let paths = AppPaths {
            settings_file: data_dir.join(SETTINGS_FILENAME),
            log_file: data_dir.join(LOG_FILENAME),
            instance_lock_file: data_dir.join(LOCK_FILENAME),
            assets_dir: std::path::PathBuf::from("assets"),
            data_dir: data_dir.clone(),
        };

        let first = acquire_single_instance_lock(&paths, false)
            .expect("first lock should be created")
            .expect("lock guard must exist");
        let second = acquire_single_instance_lock(&paths, false);

        assert!(matches!(
            second,
            Err(SingleInstanceError::AlreadyRunning { .. })
        ));

        drop(first);
        let _ = std::fs::remove_dir_all(data_dir);
    }

    #[test]
    fn stale_lock_is_recovered_when_pid_is_not_alive() {
        let unique = format!(
            "planetarium-stale-lock-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system time should be after epoch")
                .as_nanos()
        );
        let data_dir = std::env::temp_dir().join(unique);
        std::fs::create_dir_all(&data_dir).expect("temp lock dir should be created");

        let paths = AppPaths {
            settings_file: data_dir.join(SETTINGS_FILENAME),
            log_file: data_dir.join(LOG_FILENAME),
            instance_lock_file: data_dir.join(LOCK_FILENAME),
            assets_dir: std::path::PathBuf::from("assets"),
            data_dir: data_dir.clone(),
        };

        std::fs::write(&paths.instance_lock_file, "pid=99999999\n")
            .expect("stale lock file should be created");

        let lock = acquire_single_instance_lock(&paths, false)
            .expect("stale lock should be cleared")
            .expect("lock guard should be acquired after stale cleanup");
        drop(lock);

        let _ = std::fs::remove_dir_all(data_dir);
    }

    #[test]
    fn parse_pid_from_lock_reads_pid_line() {
        assert_eq!(parse_pid_from_lock("pid=123\n"), Some(123));
        assert_eq!(parse_pid_from_lock("foo=bar\npid=42\n"), Some(42));
        assert_eq!(parse_pid_from_lock(""), None);
        assert_eq!(parse_pid_from_lock("pid=abc"), None);
    }
}
