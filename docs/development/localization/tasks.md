# Localization: Log and UI String Refactoring

## Description

Replace hardcoded English strings in `info!`, `warn!`, `error!` macros and UI components with standardized Fluent localization keys.

## Tasks

- [x] Identify all hardcoded English strings in logging macros across the project.
- [x] Create an inventory of strings and propose localization keys.
- [x] Add new keys to `assets/locales/en-US/text/menu.ftl` and `assets/locales/ru-RU/text/menu.ftl`.
- [x] Refactor `Localization` engine to support internal logging and mutable bundle access.
- [x] Update systems (`Boot`, `Loading`, `Settings`, `Main Menu`, `Game`) to use `Localization` resource for logging.
- [x] Fix compilation errors in bridge systems and verify with `cargo check`.
- [x] Create a walkthrough documenting the changes.
