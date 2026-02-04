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
#[non_exhaustive]
pub struct Localization {
    /// The currently active locale (e.g., "en-US").
    pub current_locale: LanguageIdentifier,
    /// The main Fluent bundle for the current locale.
    main_bundle: fluent_bundle::bundle::FluentBundle<
        FluentResource,
        intl_memoizer::concurrent::IntlLangMemoizer,
    >,
    /// The fallback Fluent bundle (usually en-US).
    fallback_bundle: fluent_bundle::bundle::FluentBundle<
        FluentResource,
        intl_memoizer::concurrent::IntlLangMemoizer,
    >,
}

impl Localization {
    /// Creates a new localization engine with the specified bundles.
    pub fn new(
        locale: LanguageIdentifier,
        main_bundle: fluent_bundle::bundle::FluentBundle<
            FluentResource,
            intl_memoizer::concurrent::IntlLangMemoizer,
        >,
        fallback_bundle: fluent_bundle::bundle::FluentBundle<
            FluentResource,
            intl_memoizer::concurrent::IntlLangMemoizer,
        >,
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
    pub fn t_with_args(&self, key: &str, args: Option<&FluentArgs>) -> String {
        // 1. Try main bundle
        if let Some(msg) = self.main_bundle.get_message(key) {
            if let Some(pattern) = msg.value() {
                let mut errors = vec![];
                let result = self.main_bundle.format_pattern(pattern, args, &mut errors);
                for err in errors {
                    error!("[Localization] Format error (main) for '{}': {}", key, err);
                }
                return result.to_string();
            }
        }

        // 2. Try fallback bundle
        if let Some(msg) = self.fallback_bundle.get_message(key) {
            if let Some(pattern) = msg.value() {
                let mut errors = vec![];
                let result = self
                    .fallback_bundle
                    .format_pattern(pattern, args, &mut errors);
                for err in errors {
                    error!(
                        "[Localization] Format error (fallback) for '{}': {}",
                        key, err
                    );
                }
                return result.to_string();
            }
        }

        warn!("[Localization] Missing key in all bundles: {}", key);
        key.to_string()
    }

    /// Returns the localized path for an asset (e.g., audio/click.ogg).
    /// Tries the current locale directory first, then falls back to en-US.
    pub fn get_path(&self, sub_path: &str) -> String {
        let primary_path = format!("locales/{}/{}", self.current_locale, sub_path);
        let fallback_path = format!("locales/en-US/{}", sub_path);

        // We check if the file exists in the assets folder.
        // Bevy's AssetServer usually handles 'assets/' prefix automatically,
        // but since we are resolving paths manually here, we check against the disk.
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

    let primary_lang: LanguageIdentifier = settings.language.parse().unwrap_or_else(|_| {
        warn!(
            "[Localization] Invalid language '{}'. Using en-US.",
            settings.language
        );
        "en-US".parse().unwrap()
    });

    let fallback_lang: LanguageIdentifier = "en-US".parse().unwrap();

    let mut main_bundle = FluentBundle::new_concurrent(vec![primary_lang.clone()]);
    let mut fallback_bundle = FluentBundle::new_concurrent(vec![fallback_lang.clone()]);

    // Load common strings (menu.ftl)
    load_ftl_into_bundle(&mut fallback_bundle, "en-US", "menu.ftl");

    if primary_lang != fallback_lang {
        load_ftl_into_bundle(&mut main_bundle, &primary_lang.to_string(), "menu.ftl");
    } else {
        // If primary is English, main and fallback are the same (or we can just reuse resources)
        load_ftl_into_bundle(&mut main_bundle, "en-US", "menu.ftl");
    }

    commands.insert_resource(Localization::new(
        primary_lang,
        main_bundle,
        fallback_bundle,
    ));
}

fn load_ftl_into_bundle(
    bundle: &mut fluent_bundle::bundle::FluentBundle<
        FluentResource,
        intl_memoizer::concurrent::IntlLangMemoizer,
    >,
    locale: &str,
    file: &str,
) {
    let path = format!("assets/locales/{}/text/{}", locale, file);
    match std::fs::read_to_string(&path) {
        Ok(content) => {
            let resource = FluentResource::try_new(content).expect("Failed to parse FTL");
            if let Err(e) = bundle.add_resource(resource) {
                error!("[Localization] Failed to add {} to bundle: {:?}", path, e);
            } else {
                info!("[Localization] Loaded locale resource: {}", path);
            }
        }
        Err(e) => {
            warn!(
                "[Localization] Missing resource file: {}. Error: {}",
                path, e
            );
        }
    }
}
