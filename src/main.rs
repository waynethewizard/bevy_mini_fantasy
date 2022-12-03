#[allow(unused_imports)]

use bevy::prelude::*;

mod helpers;

use helpers::ascii::{AsciiPlugin, AsciiSheet};
use helpers::debug::DebugPlugin;
use helpers::graphics::GraphicsPlugin;
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

    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(Msaa {samples: 1})
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            window: WindowDescriptor {
                width: 1270.0,
                height: 720.0,
                title: String::from(
                    "Bevy MiniFantasy.",
                ),
                ..Default::default()
            },
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_state(GameState::Overworld)
        .add_plugin(DebugPlugin)
        .add_plugin(TileMapPlugin)
        .add_plugin(NineSpritePlugin)
        .add_plugin(AsciiPlugin)
        .add_plugin(GraphicsPlugin)
        .add_startup_system(startup)
        .add_system(helpers::camera::movement)
        .add_system(frame_limiter)
        .run();
}

// Janky
// https://github.com/bevyengine/bevy/issues/1343
fn frame_limiter() {
    use std::{thread, time};
    thread::sleep(time::Duration::from_millis(10));
}
