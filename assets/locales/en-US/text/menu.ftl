# Main Menu Strings
# This file is part of the reusable launcher template.
menu-title = PLANETARIUM
menu-play = Play
menu-settings = Settings
menu-exit = Exit
menu-back = Back

# Settings Tabs
settings-title = Settings
tab-graphics = Graphics
tab-audio = Audio
tab-controls = Controls
tab-general = General

# Graphics Settings
setting-resolution = Resolution
setting-fullscreen = Fullscreen
setting-vsync = VSync
setting-quality = Quality Preset
setting-world-detail = World Detail

# Audio Settings
setting-master-volume = Master Volume
setting-music-volume = Music Volume
setting-sfx-volume = Sound Effects
setting-ui-volume = Interface Volume
setting-ambience-volume = Ambience

# General Settings
setting-language = Language
setting-theme = Interface Theme
setting-allow-multiple-instances = Allow Multiple Instances

# Controls
control-forward = Move Forward
control-backward = Move Backward
control-left = Move Left
control-right = Move Right
control-jump = Jump
control-sprint = Sprint
control-interact = Interact

# Values & Common
val-on = ON
val-off = OFF
label-version = Version: { $version }
label-loading = Loading...

val-low = Low
val-medium = Medium
val-high = High
val-ultra = Ultra

lang-en = English
lang-ru = Russian

theme-dark = Dark
theme-light = Light

# Pause Menu
pause-title = Paused
pause-resume = Resume
pause-settings = Settings
pause-main-menu = Exit to Main Menu
pause-exit-game = Exit Game

# --- Logs & System Messages ---
# Boot
log-boot-init = [BootPlugin] Initializing...
log-boot-complete = [BootPlugin] Boot sequence complete. Transitioning...

# Localization
log-loc-setup = [Localization] Setting up Fluent engine for locale: { $locale }
log-loc-resolved = [Localization] Resolved requested locale '{ $requested }' -> '{ $resolved }'
log-loc-missing-dir = [Localization] Locales directory not present under assets ({ $path }); skipping per-locale load and using fallback en-US
log-loc-applying = [Localization] Applying language change: { $locale }
log-loc-updated = [Localization] Language resource updated to { $locale }
log-loc-updating-ui = [Localization] Updating UI texts for new language...
log-loc-missing-key = [Localization] Missing key in all bundles: { $key }
log-loc-format-error = [Localization] Format error ({ $bundle }) for '{ $key }': { $error }

# Loading
log-loading-reset = [LoadingUI] Tracker reset for new loading cycle.
log-loading-spawn = [LoadingUI] Spawning loading screen...
log-loading-failed = [LoadingUI] Asset failed to load: { $asset }
log-loading-complete = [LoadingUI] Content loaded. Fading out to InGame.
log-loading-cleanup = [LoadingUI] Cleaning up loading screen.

ui-loading-title = LOADING CONTENT
ui-loading-init = Initializing...

# Loading Hints
hint-scan-clusters = Scanning local star clusters...
hint-calibrate-gravity = Calibrating planetary gravity models...
hint-warm-reactors = Warming up fusion core reactors...
hint-sync-trajectories = Synchronizing orbital trajectories...
hint-opt-nav = Optimizing light-speed navigation...

# Loading Progress Info
info-loading-engine = Initializing Engine...
info-loading-stars = Loading Star Catalogs...
info-loading-textures = Synthesizing Planetary Textures...
info-loading-models = Building Atmospheric Models...
info-loading-finalizing = Finalizing World State...

# Settings
log-settings-switch-tab = [Settings] Switching to tab: { $tab }

# Game
log-game-init = [Game] Initializing 3D game world...
log-game-sphere-fail = [Game] Failed to generate sphere mesh; skipping planet placeholder.
log-game-enjoy = [Game] Handover complete. Enjoy the Cosmos!
