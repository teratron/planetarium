//! # Localization Engine
//!
//! Handles multi-language support using Project Fluent.
//! Provides a resource for retrieving localized strings with automatic fallback support.

use bevy::prelude::*;
use fluent_bundle::bundle::FluentBundle;
use fluent_bundle::{FluentArgs, FluentResource};
use unic_langid::LanguageIdentifier;

/// Type alias for Fluent bundle with concurrent memoizer.
type FluentBundleType = FluentBundle<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer>;

/// Resource that manages localized strings and bundles.
#[derive(Resource)]
#[non_exhaustive]
pub struct Localization {
    /// The currently active locale (e.g., "en-US").
    pub current_locale: LanguageIdentifier,
    /// The main Fluent bundle for the current locale.
    main_bundle: FluentBundleType,
    /// The fallback Fluent bundle (usually en-US).
    fallback_bundle: FluentBundleType,
}

impl Localization {
    /// Creates a new localization engine with the specified bundles.
    pub fn new(
        locale: LanguageIdentifier,
        main_bundle: FluentBundleType,
        fallback_bundle: FluentBundleType,
    ) -> Self {
        Self {
            current_locale: locale,
            main_bundle,
            fallback_bundle,
        }
    }

    /// Translates a key into the current language.
    /// Returns the localized string or the key itself if not found.
    pub fn t(&self, key: &str) -> String {
        self.t_with_args(key, None)
    }

    /// Translates a key with additional arguments.
    ///
    /// Attempts translation in this order:
    /// 1. Main bundle (current locale)
    /// 2. Fallback bundle (en-US)
    /// 3. Returns the key itself as fallback
    pub fn t_with_args(&self, key: &str, args: Option<&FluentArgs>) -> String {
        // Try main bundle first
        if let Some(translated) = self.try_translate(&self.main_bundle, key, args, "main") {
            return translated;
        }

        // Fall back to fallback bundle
        if let Some(translated) = self.try_translate(&self.fallback_bundle, key, args, "fallback") {
            return translated;
        }

        // If all else fails, return the key itself
        warn!("[Localization] Missing key in all bundles: {}", key);
        key.to_string()
    }

    /// Attempt to translate a key using the given bundle.
    fn try_translate(
        &self,
        bundle: &FluentBundleType,
        key: &str,
        args: Option<&FluentArgs>,
        bundle_name: &str,
    ) -> Option<String> {
        let msg = bundle.get_message(key)?;
        let pattern = msg.value()?;
        
        let mut errors = vec![];
        let result = bundle.format_pattern(pattern, args, &mut errors);
        
        if !errors.is_empty() {
            for err in errors {
                error!(
                    "[Localization] Format error ({}) for '{}': {}",
                    bundle_name, key, err
                );
            }
        }
        
        Some(result.to_string())
    }

    /// Returns the localized path for an asset (e.g., audio/click.ogg).
    /// Tries the current locale directory first, then falls back to en-US.
    pub fn get_path(&self, sub_path: &str) -> String {
        let primary_path = format!("locales/{}/{}", self.current_locale, sub_path);
        let fallback_path = format!("locales/en-US/{}", sub_path);

        let full_primary = format!("assets/{}", primary_path);
        if std::path::Path::new(&full_primary).exists() {
            primary_path
        } else {
            fallback_path
        }
    }
}

/// System to load initial locales based on user settings.
pub fn setup_localization(
    mut commands: Commands,
    settings: Res<crate::core::config::UserSettings>,
) {
    info!(
        "[Localization] Setting up Fluent engine for locale: {}",
        settings.language
    );

    let primary_lang = parse_language_id(&settings.language);
    let fallback_lang: LanguageIdentifier = "en-US".parse().unwrap();

    let mut main_bundle = FluentBundle::new_concurrent(vec![primary_lang.clone()]);
    let mut fallback_bundle = FluentBundle::new_concurrent(vec![fallback_lang.clone()]);

    // Load common strings (menu.ftl)
    load_ftl_into_bundle(&mut fallback_bundle, "en-US", "menu.ftl");

    if primary_lang != fallback_lang {
        load_ftl_into_bundle(&mut main_bundle, &primary_lang.to_string(), "menu.ftl");
    } else {
        load_ftl_into_bundle(&mut main_bundle, "en-US", "menu.ftl");
    }

    commands.insert_resource(Localization::new(
        primary_lang,
        main_bundle,
        fallback_bundle,
    ));
}

/// Parse a language string into a `LanguageIdentifier`, with fallback to en-US.
fn parse_language_id(lang_str: &str) -> LanguageIdentifier {
    lang_str.parse().unwrap_or_else(|_| {
        warn!(
            "[Localization] Invalid language '{}'. Using en-US.",
            lang_str
        );
        "en-US".parse().expect("en-US is a valid language ID")
    })
}

/// Load a Fluent Translation List (.ftl) file into a bundle.
fn load_ftl_into_bundle(
    bundle: &mut FluentBundleType,
    locale: &str,
    file: &str,
) {
    let path = format!("assets/locales/{}/text/{}", locale, file);
    match std::fs::read_to_string(&path) {
        Ok(content) => match FluentResource::try_new(content) {
            Ok(resource) => {
                if let Err(e) = bundle.add_resource(resource) {
                    error!("[Localization] Failed to add {} to bundle: {:?}", path, e);
                } else {
                    info!("[Localization] Loaded locale resource: {}", path);
                }
            }
            Err(e) => {
                error!(
                    "[Localization] Failed to parse FTL file {}: {:?}",
                    path, e
                );
            }
        },
        Err(e) => {
            warn!(
                "[Localization] Missing resource file: {}. Error: {}",
                path, e
            );
        }
    }
}
