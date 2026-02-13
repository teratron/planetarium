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

impl Validate for crate::game::config::GameplayConfig {
    type Error = Vec<String>;

    fn validate(&self) -> Result<(), Self::Error> {
        let mut errors = Vec::new();

        if self.physics.gravitational_constant <= 0.0 {
            errors.push("gravitational_constant must be positive".to_string());
        }
        if self.physics.max_velocity <= 0.0 {
            errors.push("max_velocity must be positive".to_string());
        }
        if self.planets.min_radius >= self.planets.max_radius {
            errors.push("min_radius must be less than max_radius".to_string());
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
    use crate::game::config::gameplay::GameplayConfig;

    #[test]
    fn valid_gameplay_config() {
        let config = GameplayConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn invalid_gameplay_config_physics() {
        let mut config = GameplayConfig::default();
        config.physics.gravitational_constant = -1.0;
        assert!(config.validate().is_err());
    }
}
