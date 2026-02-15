//! Localization Utilities
//!
//! Helper functions for language parsing, Fluent bundle loading, and locale resolution.

use bevy::prelude::*;
use fluent_bundle::FluentResource;
use fluent_bundle::bundle::FluentBundle;
use unic_langid::LanguageIdentifier;

/// Type alias for Fluent bundle with concurrent memoizer.
pub type FluentBundleType =
    FluentBundle<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer>;

/// Parse a language string into a `LanguageIdentifier`, with fallback to en-US.
pub fn parse_language_id(lang_str: &str) -> LanguageIdentifier {
    lang_str.parse().unwrap_or_else(|_| {
        warn!(
            "[Localization] Invalid language '{}'. Using en-US.",
            lang_str
        );
        "en-US".parse().expect("en-US is a valid language ID")
    })
}

/// Load a Fluent Translation List (.ftl) file into a bundle.
pub fn load_ftl_into_bundle(
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
pub fn resolve_locale_dir(assets_dir: &std::path::Path, locale: &str) -> String {
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
