mod camera;
mod debug;
mod movement;
mod spaceship;
mod asteroids;
mod asset_loader;
mod collision_detection;
mod despawn;

use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use collision_detection::CollisionDetectionPlugin;
use debug::DebugPlugin;
use despawn::DespawnPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins((
            DefaultPlugins,
            AssetLoaderPlugin,
            CameraPlugin,
            SpaceshipPlugin,
            MovementPlugin,
            DebugPlugin,
            AsteroidPlugin,
            CollisionDetectionPlugin,
            DespawnPlugin,
        ))
        .run();
}
