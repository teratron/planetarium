//! # Localization Engine
//!
//! Handles multi-language support using Project Fluent.
//! Provides a resource for retrieving localized strings with automatic fallback support.

use bevy::prelude::*;
use fluent_bundle::FluentArgs;
use std::collections::HashMap;
use unic_langid::LanguageIdentifier;

pub mod utils;

pub use utils::FluentBundleType;

/// Fired when the UI language is changed successfully at runtime.
#[derive(Message, Debug, Clone)]
pub struct LanguageChanged {
    /// Previous locale.
    pub old: LanguageIdentifier,
    /// New locale.
    pub new: LanguageIdentifier,
}

/// Component to mark UI text that should be updated when the language changes.
#[derive(Component, Debug, Clone)]
pub struct LocalizedText(pub String); // The key into the localization bundles

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
    /// Absolute path to the assets directory.
    assets_dir: std::path::PathBuf,
}

/// Cache for localized strings to avoid repeated bundle lookups during UI spawn.
#[derive(Resource, Debug, Default)]
#[non_exhaustive]
pub struct LocalizedStrings {
    cache: HashMap<String, String>,
    locale_tag: String,
}

impl LocalizedStrings {
    pub fn new(locale: &LanguageIdentifier) -> Self {
        Self {
            cache: HashMap::new(),
            locale_tag: locale.to_string(),
        }
    }

    /// Fetch a localized string, caching results for the current locale.
    pub fn get(&mut self, key: &str, loc: &Localization) -> String {
        let current = loc.current_locale.to_string();
        if self.locale_tag != current {
            self.locale_tag = current;
            self.cache.clear();
        }

        if let Some(value) = self.cache.get(key) {
            return value.clone();
        }

        let value = loc.t(key);
        self.cache.insert(key.to_string(), value.clone());
        value
    }

    /// Clears the cache and updates the tracked locale.
    pub fn invalidate(&mut self, locale: &LanguageIdentifier) {
        self.locale_tag = locale.to_string();
        self.cache.clear();
    }
}

impl Localization {
    /// Creates a new localization engine with the specified bundles.
    pub fn new(
        locale: LanguageIdentifier,
        main_bundle: FluentBundleType,
        fallback_bundle: FluentBundleType,
        assets_dir: std::path::PathBuf,
    ) -> Self {
        Self {
            current_locale: locale,
            main_bundle,
            fallback_bundle,
            assets_dir,
        }
    }

    /// Provides mutable access to the main bundle.
    pub fn main_bundle_mut(&mut self) -> &mut FluentBundleType {
        &mut self.main_bundle
    }

    /// Provides mutable access to the fallback bundle.
    pub fn fallback_bundle_mut(&mut self) -> &mut FluentBundleType {
        &mut self.fallback_bundle
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
        let mut warn_args = FluentArgs::new();
        warn_args.set("key", key);
        warn!(
            "{}",
            self.t_silent("log-loc-missing-key", Some(&warn_args), false)
        );
        key.to_string()
    }

    /// Internal translation helper to avoid recursion when logging missing keys.
    fn t_silent(&self, key: &str, args: Option<&FluentArgs>, log_errors: bool) -> String {
        if let Some(translated) =
            self.try_translate_internal(&self.main_bundle, key, args, "main", log_errors)
        {
            return translated;
        }
        if let Some(translated) =
            self.try_translate_internal(&self.fallback_bundle, key, args, "fallback", log_errors)
        {
            return translated;
        }
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
        self.try_translate_internal(bundle, key, args, bundle_name, true)
    }

    fn try_translate_internal(
        &self,
        bundle: &FluentBundleType,
        key: &str,
        args: Option<&FluentArgs>,
        bundle_name: &str,
        log_errors: bool,
    ) -> Option<String> {
        let msg = bundle.get_message(key)?;
        let pattern = msg.value()?;

        let mut errors = vec![];
        let result = bundle.format_pattern(pattern, args, &mut errors);

        if log_errors && !errors.is_empty() {
            for err in errors {
                let mut err_args = FluentArgs::new();
                err_args.set("bundle", bundle_name);
                err_args.set("key", key);
                err_args.set("error", err.to_string());
                error!(
                    "{}",
                    self.t_silent("log-loc-format-error", Some(&err_args), false)
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

        let full_primary = self.assets_dir.join(&primary_path);
        if full_primary.exists() {
            primary_path
        } else {
            fallback_path
        }
    }
}

pub struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<LanguageChanged>()
            .add_systems(Update, update_localized_texts);
    }
}

/// System that updates all UI elements with `LocalizedText` when language changes.
pub fn update_localized_texts(
    mut events: MessageReader<LanguageChanged>,
    localization: Res<Localization>,
    mut query: Query<(&mut Text, &LocalizedText)>,
) {
    // Only run if a LanguageChanged message was received
    if events.read().next().is_some() {
        info!("[Localization] Updating UI texts for new language...");
        for (mut text, localized) in &mut query {
            text.0 = localization.t(&localized.0);
        }
    }
}
