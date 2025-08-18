use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod character_cmp;
mod movement;
use bevy_ecs_tilemap::tiles::TilePos;
pub use character_cmp::Character;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut tile_query: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapTileSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
) {
    let (tm_size, tm_grid_size, tm_tile_size, tm_type, tm_anchor) =
        tile_query.single_mut().unwrap();

    let tile_pos = TilePos::new(tm_size.x - 1, 0);
    let transform = Transform::from_translation(
        tile_pos
            .center_in_world(tm_size, tm_grid_size, tm_tile_size, tm_type, tm_anchor)
            .extend(1.0) - Vec3::new(0., 10., 0.)
    );
    commands.spawn((
        Sprite {
            image: asset_server.load("textures/tiles/person.png"),
            anchor: bevy::sprite::Anchor::BottomCenter,
            ..default()
        },
        tile_pos,
        transform,
        Character,
    ));
}
