use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::cursor::HighlightedLabel;

#[derive(SystemSet, Eq, PartialEq, Clone, Debug, Hash)]
pub struct HighlightedSet;

pub struct VisualHighlightPlugin;

impl Plugin for VisualHighlightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (remove_highlight_sprite, add_highlight_sprite)
                .chain()
                .after(HighlightedSet),
        );
    }
}

pub fn remove_highlight_sprite(mut query: Query<&mut TileTextureIndex, Without<HighlightedLabel>>) {
    for mut tex_index in &mut query {
        *tex_index = TileTextureIndex(0);
    }
}

pub fn add_highlight_sprite(mut query: Query<&mut TileTextureIndex, With<HighlightedLabel>>) {
    for mut tex_index in &mut query {
        *tex_index = TileTextureIndex(1);
    }
}
