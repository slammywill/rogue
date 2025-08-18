use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

#[derive(Component)]
pub struct HighlightedLabel;

#[derive(Resource)]
pub struct CursorPos(Vec2);
impl Default for CursorPos {
    // Initialise far away, will be properly set when cursor moves
    fn default() -> Self {
        Self(Vec2::new(-1000.0, -1000.0))
    }
}

pub fn update_cursor_pos(
    cam_query: Query<(&GlobalTransform, &Camera)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_pos: ResMut<CursorPos>,
) {
    for cursor_moved in cursor_moved_events.read() {
        // To get the mouse's world position, we have to transform its window position by
        // any transforms on the camera. This is done by projecting the cursor position into
        // camera space (world space).
        for (cam_transform, cam) in cam_query.iter() {
            if let Ok(pos) = cam.viewport_to_world_2d(cam_transform, cursor_moved.position) {
                *cursor_pos = CursorPos(pos);
            }
        }
    }
}

pub fn highlight_tile(
    mut commands: Commands,
    cursor_pos: Res<CursorPos>,
    tilemap_query: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapTileSize,
        &TilemapType,
        &TilemapAnchor,
        &TileStorage,
        &Transform,
    )>,
    hightlighted_query: Query<Entity, With<HighlightedLabel>>,
) {
    // Unhighlight previously highlighted tile
    for highlighted in hightlighted_query.iter() {
        commands.entity(highlighted).remove::<HighlightedLabel>();
    }

    for (tm_size, tm_grid_size, tm_tile_size, tm_type, tm_anchor, tile_storage, tm_transform) in
        tilemap_query.iter()
    {
        let cursor_pos = cursor_pos.0;

        // Ensure cursor's world position is correct relative to the map due to any map transformation.
        let cursor_in_map_pos: Vec2 = {
            let cursor_pos = Vec4::from((cursor_pos, 0.0, 1.0));
            let cursor_in_map_pos = tm_transform.compute_matrix().inverse() * cursor_pos;
            cursor_in_map_pos.xy()
        };

        if let Some(tile_pos) = TilePos::from_world_pos(
            &cursor_in_map_pos,
            tm_size,
            tm_grid_size,
            tm_tile_size,
            tm_type,
            tm_anchor,
        ) {
            if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                commands.entity(tile_entity).insert(HighlightedLabel);
            }
        }
    }
}
