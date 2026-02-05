use bevy::prelude::*;
use planetarium::ui::theme::Theme;

#[test]
fn setup_debug_overlay_spawns_expected_texts() {
    let mut app = App::new();

    // The diagnostics overlay setup expects a Theme resource to exist.
    app.insert_resource(Theme::default());

    // Add just the startup system directly to avoid other plugin systems that expect runtime resources.
    app.add_systems(
        Startup,
        planetarium::launcher::diagnostics::setup_debug_overlay,
    );

    // Run startup systems
    app.update();

    let mut found_fps = false;
    let mut found_state = false;
    let mut found_entities = false;

    let world = app.world_mut();
    let mut query = world.query::<&Text>();
    for text in query.iter(world) {
        let s = text.0.trim();
        if s.starts_with("FPS:") {
            found_fps = true;
        }
        if s.starts_with("STATE:") {
            found_state = true;
        }
        if s.starts_with("ENTITIES:") {
            found_entities = true;
        }
    }

    assert!(
        found_fps && found_state && found_entities,
        "Diagnostics overlay should spawn labels for FPS, STATE and ENTITIES"
    );
}
