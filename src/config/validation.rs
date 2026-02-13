//! # Settings Validation
//!
//! Centralized validation for user settings sub-structures.

/// Trait for validating settings structures.
pub trait Validate {
    /// The error type returned by validation.
    type Error;

    /// Validates the structure and returns either a sanitized copy or an error.
    fn validate(&self) -> Result<(), Self::Error>;
}

impl Validate for super::settings::DisplaySettings {
    type Error = Vec<String>;

    fn validate(&self) -> Result<(), Self::Error> {
        let mut errors = Vec::new();

        if self.width < 640 || self.width > 7680 {
            errors.push(format!("width {} out of range [640, 7680]", self.width));
        }

        if self.height < 480 || self.height > 4320 {
            errors.push(format!("height {} out of range [480, 4320]", self.height));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Validate for super::settings::AudioSettings {
    type Error = Vec<String>;

    fn validate(&self) -> Result<(), Self::Error> {
        let mut errors = Vec::new();

        if !(0.0..=1.0).contains(&self.master_volume) {
            errors.push(format!(
                "master_volume {} out of range [0.0, 1.0]",
                self.master_volume
            ));
        }
        if !(0.0..=1.0).contains(&self.music_volume) {
            errors.push(format!(
                "music_volume {} out of range [0.0, 1.0]",
                self.music_volume
            ));
        }
        if !(0.0..=1.0).contains(&self.sfx_volume) {
            errors.push(format!(
                "sfx_volume {} out of range [0.0, 1.0]",
                self.sfx_volume
            ));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::settings::{AudioSettings, DisplaySettings};

    #[test]
    fn valid_display_settings() {
        let ds = DisplaySettings {
            width: 1920,
            height: 1080,
            fullscreen: false,
            vsync: true,
        };
        assert!(ds.validate().is_ok());
    }

    #[test]
    fn invalid_display_width() {
        let ds = DisplaySettings {
            width: 100,
            height: 1080,
            fullscreen: false,
            vsync: true,
        };
        assert!(ds.validate().is_err());
    }

    #[test]
    fn valid_audio_settings() {
        let audio = AudioSettings {
            master_volume: 0.8,
            music_volume: 0.6,
            sfx_volume: 0.5,
        };
        assert!(audio.validate().is_ok());
    }

    #[test]
    fn invalid_audio_settings() {
        let audio = AudioSettings {
            master_volume: 1.5,
            music_volume: -0.1,
            sfx_volume: 0.5,
        };
        let err = audio.validate().unwrap_err();
        assert_eq!(err.len(), 2);
    }
}
