//! Platform-specific path resolution without external dependencies

use std::env;
use std::path::PathBuf;

/// Get platform-specific data directory
pub fn get_data_dir(app_name: &str) -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        env::var("APPDATA")
            .ok()
            .map(|p| PathBuf::from(p).join(app_name))
            .unwrap_or_else(|| PathBuf::from("data"))
    }

    #[cfg(target_os = "macos")]
    {
        env::var("HOME")
            .ok()
            .map(|home| {
                PathBuf::from(home)
                    .join("Library/Application Support")
                    .join(app_name)
            })
            .unwrap_or_else(|| PathBuf::from("data"))
    }

    #[cfg(target_os = "linux")]
    {
        env::var("XDG_DATA_HOME")
            .ok()
            .map(|p| PathBuf::from(p).join(app_name))
            .or_else(|| {
                env::var("HOME")
                    .ok()
                    .map(|home| PathBuf::from(home).join(".local/share").join(app_name))
            })
            .unwrap_or_else(|| PathBuf::from("data"))
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        PathBuf::from("data")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_dir_is_valid() {
        let dir = get_data_dir("test_app");
        assert!(!dir.as_os_str().is_empty());
    }
}
