//! Single-instance process lock management.

use crate::core::config::AppPaths;
use std::fmt::{Display, Formatter};
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
#[derive(Debug)]
pub enum SingleInstanceError {
    AlreadyRunning { lock_file: PathBuf },
    Io(std::io::Error),
}

impl Display for SingleInstanceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyRunning { lock_file } => write!(
                f,
                "another game instance is already running (lock file: {:?})",
                lock_file
            ),
            Self::Io(error) => write!(f, "failed to acquire startup lock: {}", error),
        }
    }
}

impl std::error::Error for SingleInstanceError {}

/// Acquire a global lock for the process unless multi-instance mode is enabled.
pub fn acquire_single_instance_lock(
    paths: &AppPaths,
    allow_multiple_instances: bool,
) -> Result<Option<SingleInstanceLock>, SingleInstanceError> {
    if allow_multiple_instances {
        return Ok(None);
    }

    let file = OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(&paths.instance_lock_file)
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                SingleInstanceError::AlreadyRunning {
                    lock_file: paths.instance_lock_file.clone(),
                }
            } else {
                SingleInstanceError::Io(e)
            }
        })?;

    write_lock_metadata(file, &paths.instance_lock_file).map(Some)
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
    use crate::core::config::AppPaths;

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
            settings_file: data_dir.join("settings.toml"),
            log_file: data_dir.join("session.log"),
            instance_lock_file: data_dir.join("instance.lock"),
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
}
