use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, spawn_character);
	}
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Accumulateinput {
	pub movement: Vec2,
}

#[derive(Component, Default)]
pub struct Player;

/// The actual position of the player in the physics simulation.
/// This is separate from the `Transform`, which is merely a visual representation.
///
/// If you want to make sure that this component is always initialized
/// with the same value as the `Transform`'s translation, you can
/// use a [component lifecycle hook](https://docs.rs/bevy/0.14.0/bevy/ecs/component/struct.ComponentHooks.html)
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PhysicalTranslation(Vec3);

/// The value [`PhysicalTranslation`] had in the last fixed timestep.
/// Used for interpolation in the `interpolate_rendered_transform` system.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PreviousPhysicalTranslation(Vec3);

fn spawn_character(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let player = (
		Mesh3d(meshes.add(Cuboid::new(1.0,1.0,1.0))),
		MeshMaterial3d(materials.add(Color::srgb_u8(124,144,255))),
		Player::default(),	
		Accumulateinput::default(),
		Velocity::default(),
		PhysicalTranslation::default(),
		PreviousPhysicalTranslation::default(),
		Transform {
			translation: Vec3::new(0.0,1.5,0.0),
			rotation: Quat::IDENTITY,
			scale: Vec3::splat(0.5),
		}
		);

	commands.spawn(player);
}
