#[allow(unused_imports)]

use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};

mod helpers;


use helpers::ascii::{AsciiPlugin, AsciiSheet};
use helpers::debug::DebugPlugin;
//use helpers::graphics::GraphicsPlugin;
use helpers::nine_sprite::NineSpritePlugin;
use helpers::tilemap::TileMapPlugin;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.10;
pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Overworld
}

fn startup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    let height = 900.0;
    App::new()
        .insert_resource(ClearColor(CLEAR))
       // .insert_resource(Msaa {samples: 1})
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "I am a window!".to_string(),
                width: 1500.,
                height: 800.,
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(spawn_camera)
        .add_state(GameState::Overworld)
        .add_plugin(DebugPlugin)
        .add_plugin(TileMapPlugin)
        .add_plugin(NineSpritePlugin)
        .add_plugin(AsciiPlugin)
        //.add_plugin(GraphicsPlugin)
        .add_system(frame_limiter)
        .run();
}

// Janky
// https://github.com/bevyengine/bevy/issues/1343
fn frame_limiter() {
    use std::{thread, time};
    thread::sleep(time::Duration::from_millis(10));
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
   // camera.projection.scaling_mode = ScalingMode::None;
    camera.projection.top = 1.0;
    camera.projection.bottom = -1.0;
    camera.projection.right = 1.0 * RESOLUTION;
    camera.projection.left = -1.0 * RESOLUTION;
    commands.spawn_bundle(camera);
}