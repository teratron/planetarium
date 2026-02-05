use bevy::prelude::*;
use planetarium::launcher::menu::widgets::{
    ButtonAction, SliderSpec, spawn_primary_button, spawn_slider,
};
use planetarium::ui::theme::Theme;

#[test]
fn spawn_primary_button_creates_primary_button_component() {
    let mut app = App::new();
    app.insert_resource(Theme::default());

    app.add_systems(Startup, |mut commands: Commands, theme: Res<Theme>| {
        let parent = commands.spawn(Node { ..default() }).id();
        spawn_primary_button(&mut commands, &theme, "TEST", ButtonAction::Exit, parent);
    });

    app.update();

    {
        let world = app.world_mut();
        let mut query = world.query::<&planetarium::launcher::menu::widgets::PrimaryButton>();
        let mut found = false;
        for pb in query.iter(world) {
            assert_eq!(pb.label, "TEST");
            assert_eq!(pb.action, ButtonAction::Exit);
            found = true;
        }
        assert!(
            found,
            "PrimaryButton was not spawned by spawn_primary_button"
        );
    }
}

#[test]
fn spawn_slider_creates_slider_component() {
    let mut app = App::new();
    app.insert_resource(Theme::default());

    app.add_systems(Startup, |mut commands: Commands, theme: Res<Theme>| {
        let parent = commands.spawn(Node { ..default() }).id();
        let spec = SliderSpec {
            min: 0.0,
            max: 100.0,
            value: 42.0,
        };
        spawn_slider(&mut commands, &theme, "VOL", spec, "audio.master", parent);
    });

    app.update();

    {
        let world = app.world_mut();
        let mut query = world.query::<&planetarium::launcher::menu::widgets::Slider>();
        let mut found = false;
        for slider in query.iter(world) {
            assert_eq!(slider.label, "VOL");
            assert_eq!(slider.min, 0.0);
            assert_eq!(slider.max, 100.0);
            assert_eq!(slider.value, 42.0);
            assert_eq!(slider.setting_key, "audio.master");
            found = true;
        }
        assert!(found, "Slider was not spawned by spawn_slider");
    }
}
