use bevy::prelude::*;
use fluent_bundle::bundle::FluentBundle;
// We need to re-export Localization for boot/mod.rs if it uses crate::boot::localization::Localization
use localization::utils::{load_ftl_into_bundle, parse_language_id};
pub use localization::{LanguageChanged, Localization, LocalizedStrings};

use crate::boot::AppPaths;
use crate::config::UserSettings;

/// System to load initial locales based on user settings.
pub fn setup_localization(
    mut commands: Commands,
    settings: Res<UserSettings>,
    paths: Res<AppPaths>,
) {
    let primary_lang = parse_language_id(&settings.language);
    let fallback_lang = parse_language_id("en-US");

    let main_bundle =
        FluentBundle::<_, intl_memoizer::concurrent::IntlLangMemoizer>::new_concurrent(vec![
            primary_lang.clone(),
        ]);
    let mut fallback_bundle =
        FluentBundle::<_, intl_memoizer::concurrent::IntlLangMemoizer>::new_concurrent(vec![
            fallback_lang.clone(),
        ]);

    // Load common strings (menu.ftl) into fallback bundle
    load_ftl_into_bundle(&mut fallback_bundle, &paths.assets_dir, "en-US", "menu.ftl");

    let mut localization = Localization::new(
        primary_lang.clone(),
        main_bundle,
        fallback_bundle,
        paths.assets_dir.clone(),
    );

    let mut info_args = fluent_bundle::FluentArgs::new();
    info_args.set("locale", settings.language.clone());
    info!(
        "{}",
        localization.t_with_args("log-loc-setup", Some(&info_args))
    );

    // Resolve and log the chosen locale directory for clarity
    let requested_locale = settings.language.clone();
    let resolved = localization::utils::resolve_locale_dir(&paths.assets_dir, &requested_locale);
    if !resolved.eq_ignore_ascii_case(&requested_locale) {
        let mut res_args = fluent_bundle::FluentArgs::new();
        res_args.set("requested", requested_locale);
        res_args.set("resolved", resolved.clone());
        info!(
            "{}",
            localization.t_with_args("log-loc-resolved", Some(&res_args))
        );
    }

    // Only attempt to load locale files if the assets locales directory exists.
    if paths.assets_dir.join("locales").exists() {
        localization::utils::load_ftl_into_bundle(
            localization.main_bundle_mut(),
            &paths.assets_dir,
            &resolved,
            "menu.ftl",
        );
    } else {
        let mut warn_args = fluent_bundle::FluentArgs::new();
        warn_args.set("path", paths.assets_dir.display().to_string());
        warn!(
            "{}",
            localization.t_with_args("log-loc-missing-dir", Some(&warn_args))
        );
    }

    commands.insert_resource(localization);
    commands.insert_resource(LocalizedStrings::new(&primary_lang));
}

/// System that applies language changes at runtime when `UserSettings` changes.
pub fn apply_language_change_system(
    settings: Res<UserSettings>,
    mut prev: Local<Option<String>>,
    paths: Res<AppPaths>,
    mut commands: Commands,
    mut strings: ResMut<LocalizedStrings>,
    mut events: MessageWriter<LanguageChanged>,
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
        let resolved =
            localization::utils::resolve_locale_dir(&paths.assets_dir, &requested_locale);
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
        let old_locale = parse_language_id(prev.as_deref().unwrap_or(""));
        strings.invalidate(&primary_lang);
        events.write(LanguageChanged {
            old: old_locale,
            new: primary_lang.clone(),
        });

        info!(
            "[Localization] Language resource updated to {}",
            settings.language
        );
    }

    *prev = Some(settings.language.clone());
}
