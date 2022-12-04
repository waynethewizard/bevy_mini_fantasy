use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::prelude::*;

use crate::{
    helpers::ascii::{spawn_ascii_sprite, AsciiSheet},
    //npc::Npc,
    GameState, TILE_SIZE,
};

pub struct TileMapPlugin;

#[derive(Component)]
struct Map;

#[derive(Component)]
pub struct EncounterSpawner;

#[derive(Component)]
pub struct TileCollider;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Overworld).with_system(create_simple_map),
        )
        .add_system_set(SystemSet::on_resume(GameState::Overworld).with_system(show_map))
        .add_system_set(SystemSet::on_pause(GameState::Overworld).with_system(hide_map));
    }
}

fn hide_map(
    children_query: Query<&Children, With<Map>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Map>>,
) {
    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = false;
            }
        }
    }
}

fn show_map(
    children_query: Query<&Children, With<Map>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Map>>,
) {
    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = true;
            }
        }
    }
}

fn create_simple_map(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let file = File::open("assets/map.txt").expect("No map file found");
    let mut tiles = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let color = match char {
                    '#' => Color::rgb(100.7, 0.7, 1.0),
                    '@' => Color::rgb(110.5, 0.5, 1.0),
                    '~' => Color::rgb(110.2, 0.9, 1.0),
                    _ => Color::rgb(10.9, 0.9, 0.9),
                };
                let tile = spawn_ascii_sprite(
                    &mut commands,
                    &ascii,
                    char as usize,
                    color,
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
                    Vec3::splat(1.0),
                );
                tiles.push(tile);
            }
        }
    }

    commands
        .spawn((Map, Name::new("Map"),
               SpatialBundle::default()))
        .push_children(&tiles);
}