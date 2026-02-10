//! Localization Systems
//!
//! Bevy systems for initializing and managing localization at runtime.

use bevy::prelude::*;
use fluent_bundle::bundle::FluentBundle;

use super::utils::{load_ftl_into_bundle, parse_language_id};
use super::{Localization, LocalizedStrings};

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
    let fallback_lang = parse_language_id("en-US");

    let mut main_bundle =
        FluentBundle::<_, intl_memoizer::concurrent::IntlLangMemoizer>::new_concurrent(vec![
            primary_lang.clone(),
        ]);
    let mut fallback_bundle =
        FluentBundle::<_, intl_memoizer::concurrent::IntlLangMemoizer>::new_concurrent(vec![
            fallback_lang.clone(),
        ]);

    // Load common strings (menu.ftl)
    load_ftl_into_bundle(&mut fallback_bundle, &paths.assets_dir, "en-US", "menu.ftl");

    // Resolve and log the chosen locale directory for clarity (helps explain missing-file warnings).
    let requested_locale = settings.language.clone();
    let resolved = super::utils::resolve_locale_dir(&paths.assets_dir, &requested_locale);
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
        primary_lang.clone(),
        main_bundle,
        fallback_bundle,
        paths.assets_dir.clone(),
    ));
    commands.insert_resource(LocalizedStrings::new(&primary_lang));
}

/// System that applies language changes at runtime when `UserSettings` changes.
pub fn apply_language_change_system(
    settings: Res<crate::core::config::UserSettings>,
    mut prev: Local<Option<String>>,
    paths: Res<crate::core::config::AppPaths>,
    mut commands: Commands,
    mut strings: ResMut<LocalizedStrings>,
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
        let fallback_lang = parse_language_id("en-US");

        let mut main_bundle =
            FluentBundle::<_, intl_memoizer::concurrent::IntlLangMemoizer>::new_concurrent(vec![
                primary_lang.clone(),
            ]);
        let mut fallback_bundle =
            FluentBundle::<_, intl_memoizer::concurrent::IntlLangMemoizer>::new_concurrent(vec![
                fallback_lang.clone(),
            ]);

        load_ftl_into_bundle(&mut fallback_bundle, &paths.assets_dir, "en-US", "menu.ftl");

        let requested_locale = settings.language.clone();
        let resolved = super::utils::resolve_locale_dir(&paths.assets_dir, &requested_locale);
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
            primary_lang.clone(),
            main_bundle,
            fallback_bundle,
            paths.assets_dir.clone(),
        ));
        strings.invalidate(&primary_lang);

        info!(
            "[Localization] Language resource updated to {}",
            settings.language
        );
    }

    *prev = Some(settings.language.clone());
}
