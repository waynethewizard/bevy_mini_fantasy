use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::helpers::{debug::ENABLE_INSPECTOR};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_graphics)
            .add_system(animate_sprites);
        if ENABLE_INSPECTOR {
            app.register_type::<AnimatedSprite>();
        }
    }
}

#[derive(Resource)]
pub struct GraphicsHandles {
    pub characters: Handle<TextureAtlas>,
    pub tiles: Handle<TextureAtlas>,
}

#[derive(Component, Inspectable)]
pub enum FacingDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct AnimatedSprite {
    pub current_frame: usize,
    pub timer: Timer,
}

fn load_graphics(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("characters.png");
    let atlas =
        TextureAtlas::from_grid(image, Vec2::splat(16.0), 12, 9, None, None);
    let character_handle = atlases.add(atlas);

    let image = assets.load("basictiles.png");
    let atlas =
        TextureAtlas::from_grid(image, Vec2::splat(16.0), 8, 15, None, None);
    let tile_handle = atlases.add(atlas);

    commands.insert_resource(GraphicsHandles {
        characters: character_handle,
        tiles: tile_handle,
    });

}

//TODO restructure this to support animations more generally
fn animate_sprites(mut sprites: Query<&mut AnimatedSprite>, time: Res<Time>) {
    for mut sprite in sprites.iter_mut() {
        sprite.timer.tick(time.delta());
        if sprite.timer.just_finished() {
            //Probs not dangerous but
            //FIXME handle wrap around
            sprite.current_frame += 1;
        }
    }
}
