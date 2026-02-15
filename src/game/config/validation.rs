//! # Gameplay Config Validation
//!
//! Validation logic for game-specific balance parameters.

use super::gameplay::GameplayConfig;
use menu::settings::validation::Validate;

impl Validate for GameplayConfig {
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
