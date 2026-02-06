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
    /// Absolute path to the assets directory.
    assets_dir: std::path::PathBuf,
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

        let full_primary = self.assets_dir.join(&primary_path);
        if full_primary.exists() {
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
    paths: Res<crate::core::config::AppPaths>,
) {
    info!(
        "[Localization] Setting up Fluent engine for locale: {}",
        settings.language
    );

    let primary_lang = parse_language_id(&settings.language);
    // Use the same parsing function to ensure consistent fallback handling for en-US as well.
    let fallback_lang: LanguageIdentifier = parse_language_id("en-US");

    let mut main_bundle = FluentBundle::new_concurrent(vec![primary_lang.clone()]);
    let mut fallback_bundle = FluentBundle::new_concurrent(vec![fallback_lang.clone()]);

    // Load common strings (menu.ftl)
    load_ftl_into_bundle(&mut fallback_bundle, &paths.assets_dir, "en-US", "menu.ftl");

    // Resolve and log the chosen locale directory for clarity (helps explain missing-file warnings).
    let requested_locale = settings.language.clone();
    let resolved = resolve_locale_dir(&paths.assets_dir, &requested_locale);
    if !resolved.eq_ignore_ascii_case(&requested_locale) {
        info!(
            "[Localization] Resolved requested locale '{}' -> '{}'",
            requested_locale, resolved
        );
    }

    // Only attempt to load locale files if the assets locales directory exists.
    if paths.assets_dir.join("locales").exists() {
        load_ftl_into_bundle(&mut main_bundle, &paths.assets_dir, &resolved, "menu.ftl");
    } else {
        warn!(
            "[Localization] Locales directory not present under assets ({}); skipping per-locale load and using fallback en-US",
            paths.assets_dir.display()
        );
    }

    commands.insert_resource(Localization::new(
        primary_lang,
        main_bundle,
        fallback_bundle,
        paths.assets_dir.clone(),
    ));
}

/// System that applies language changes at runtime when `UserSettings` changes.
pub fn apply_language_change_system(
    settings: Res<crate::core::config::UserSettings>,
    mut prev: Local<Option<String>>,
    paths: Res<crate::core::config::AppPaths>,
    mut commands: Commands,
) {
    if !settings.is_changed() {
        return;
    }

    if prev.as_deref() != Some(settings.language.as_str()) {
        info!(
            "[Localization] Applying language change: {}",
            settings.language
        );

        let primary_lang = parse_language_id(&settings.language);
        let fallback_lang: LanguageIdentifier = parse_language_id("en-US");

        let mut main_bundle = FluentBundle::new_concurrent(vec![primary_lang.clone()]);
        let mut fallback_bundle = FluentBundle::new_concurrent(vec![fallback_lang.clone()]);

        load_ftl_into_bundle(&mut fallback_bundle, &paths.assets_dir, "en-US", "menu.ftl");

        let requested_locale = settings.language.clone();
        let resolved = resolve_locale_dir(&paths.assets_dir, &requested_locale);
        if !resolved.eq_ignore_ascii_case(&requested_locale) {
            info!(
                "[Localization] Resolved requested locale '{}' -> '{}'",
                requested_locale, resolved
            );
        }

        // Only attempt to load locale files if the assets locales directory exists.
        if paths.assets_dir.join("locales").exists() {
            load_ftl_into_bundle(&mut main_bundle, &paths.assets_dir, &resolved, "menu.ftl");
        } else {
            warn!(
                "[Localization] Locales directory not present under assets ({}); skipping per-locale load and using fallback en-US",
                paths.assets_dir.display()
            );
        }

        commands.insert_resource(Localization::new(
            primary_lang,
            main_bundle,
            fallback_bundle,
            paths.assets_dir.clone(),
        ));

        info!(
            "[Localization] Language resource updated to {}",
            settings.language
        );
    }

    *prev = Some(settings.language.clone());
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
    assets_dir: &std::path::Path,
    locale: &str,
    file: &str,
) {
    // Resolve the best matching locale directory in the assets folder.
    let resolved_locale = resolve_locale_dir(assets_dir, locale);
    let path = assets_dir.join(format!("locales/{}/text/{}", resolved_locale, file));

    match std::fs::read_to_string(&path) {
        Ok(content) => match FluentResource::try_new(content) {
            Ok(resource) => {
                if let Err(e) = bundle.add_resource(resource) {
                    error!(
                        "[Localization] Failed to add {} to bundle: {:?}",
                        path.display(),
                        e
                    );
                } else {
                    info!(
                        "[Localization] Loaded locale resource: {} (resolved from '{}')",
                        path.display(),
                        locale
                    );
                }
            }
            Err(e) => {
                error!(
                    "[Localization] Failed to parse FTL file {}: {:?}",
                    path.display(),
                    e
                );
            }
        },
        Err(_e) => {
            // Do not print raw OS errors (they may be localized). Give a clear,
            // actionable warning that avoids noisy or confusing OS messages.
            if !assets_dir.join("locales").exists() {
                warn!(
                    "[Localization] Locales directory not found under assets ({}). Ensure locale files are present in the assets folder. Falling back to 'en-US'.",
                    assets_dir.display()
                );
            } else {
                warn!(
                    "[Localization] Locale '{}' not found at '{}'. Falling back to 'en-US'.",
                    resolved_locale,
                    path.display()
                );
            }
        }
    }
}

/// Attempt to map a requested locale string to an available locale directory.
fn resolve_locale_dir(assets_dir: &std::path::Path, locale: &str) -> String {
    let locales_dir = assets_dir.join("locales");

    // 1. Direct match
    let direct = locales_dir.join(locale);
    if direct.exists() {
        return locale.to_string();
    }

    // 2. Try short code (e.g., 'ru' from 'russian' or 'ru-RU').
    let short = locale
        .split(|c: char| ['-', '_'].contains(&c))
        .next()
        .unwrap_or(locale)
        .to_lowercase();

    // Common mappings for user-friendly names
    let common_map = [
        ("russian", "ru-RU"),
        ("russian (russian)", "ru-RU"),
        ("english", "en-US"),
        ("en", "en-US"),
    ];

    for (k, v) in &common_map {
        if locale.eq_ignore_ascii_case(k) || short.eq_ignore_ascii_case(k) {
            let candidate = locales_dir.join(v);
            if candidate.exists() {
                info!(
                    "[Localization] Mapped locale '{}' -> '{}' using common map",
                    locale, v
                );
                return v.to_string();
            }
        }
    }

    // 3. Fallback: try to find any locale folder that starts with the short code.
    if let Ok(entries) = std::fs::read_dir(&locales_dir) {
        for entry in entries.flatten() {
            if let Ok(ft) = entry.file_type()
                && ft.is_dir()
                && let Some(name_os) = entry.file_name().to_str()
                && name_os.to_lowercase().starts_with(&short)
            {
                info!(
                    "[Localization] Mapped locale '{}' -> '{}' by prefix match",
                    locale, name_os
                );
                return name_os.to_string();
            }
        }
    }

    // 4. As a last resort, return the original locale (so the caller will log missing file as before).
    locale.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn resolve_locale_dir_maps_russian() {
        let base = std::env::temp_dir().join("planetarium_loc_test");
        let _ = fs::remove_dir_all(&base);
        let locales = base.join("locales");
        fs::create_dir_all(locales.join("en-US/text")).unwrap();
        fs::create_dir_all(locales.join("ru-RU/text")).unwrap();

        let resolved = resolve_locale_dir(&base, "russian");
        assert_eq!(resolved, "ru-RU");

        let resolved_short = resolve_locale_dir(&base, "ru");
        assert_eq!(resolved_short, "ru-RU");

        let resolved_exact = resolve_locale_dir(&base, "en-US");
        assert_eq!(resolved_exact, "en-US");

        // Clean up
        let _ = fs::remove_dir_all(&base);
    }
}
