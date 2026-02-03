//! # Localization Engine
//!
//! Handles multi-language support using Project Fluent.
//! Provides a resource for retrieving localized strings.

use bevy::prelude::*;
use fluent_bundle::bundle::FluentBundle;
use fluent_bundle::{FluentArgs, FluentResource};
use unic_langid::LanguageIdentifier;

/// Resource that manages localized strings and bundles.
#[derive(Resource)]
pub struct Localization {
    /// The currently active locale (e.g., "en-US").
    pub current_locale: LanguageIdentifier,
    /// The Fluent bundle for the current locale.
    bundle: fluent_bundle::bundle::FluentBundle<
        FluentResource,
        intl_memoizer::concurrent::IntlLangMemoizer,
    >,
}

impl Localization {
    /// Creates a new localization engine with the specified bundle.
    pub fn new(
        locale: LanguageIdentifier,
        bundle: fluent_bundle::bundle::FluentBundle<
            FluentResource,
            intl_memoizer::concurrent::IntlLangMemoizer,
        >,
    ) -> Self {
        Self {
            current_locale: locale,
            bundle,
        }
    }

    /// Translates a key into the current language.
    /// Returns the localized string or the key itself if not found.
    pub fn t(&self, key: &str) -> String {
        self.t_with_args(key, None)
    }

    /// Translates a key with additional arguments.
    pub fn t_with_args(&self, key: &str, args: Option<&FluentArgs>) -> String {
        let msg = match self.bundle.get_message(key) {
            Some(m) => m,
            None => {
                warn!("[Localization] Missing key: {}", key);
                return key.to_string();
            }
        };

        let pattern = match msg.value() {
            Some(p) => p,
            None => return key.to_string(),
        };

        let mut errors = vec![];
        let result = self.bundle.format_pattern(pattern, args, &mut errors);

        for err in errors {
            error!("[Localization] Format error for '{}': {}", key, err);
        }

        result.to_string()
    }
}

/// System to load initial locales.
/// In a real app, this would be more complex (async loading, etc.).
pub fn setup_localization(mut commands: Commands) {
    info!("[Localization] Setting up Fluent engine...");

    // For now, we manually load the English locale as default.
    // In L-202, we will implement the proper multi-locale loader.
    let locale: LanguageIdentifier = "en-US".parse().expect("Failed to parse default locale");
    let mut bundle: FluentBundle<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer> =
        FluentBundle::new_concurrent(vec![locale.clone()]);

    // Note: We are using a relative path from the project root here.
    // In a final build, this would be handled by AssetServer.
    let ftl_path = "assets/locales/en-US/text/menu.ftl";
    match std::fs::read_to_string(ftl_path) {
        Ok(ftl_content) => {
            let resource =
                FluentResource::try_new(ftl_content).expect("Failed to parse Fluent resource");
            bundle
                .add_resource(resource)
                .expect("Failed to add resource to bundle");
            info!("[Localization] Loaded locale: {}", locale);
        }
        Err(e) => {
            error!("[Localization] Failed to load {}: {}", ftl_path, e);
        }
    }

    commands.insert_resource(Localization::new(locale, bundle));
}
