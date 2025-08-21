use bevy::prelude::*;

pub mod camera;
pub mod cursor;
pub mod gameplay;
pub mod highlighted;
pub mod map;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            camera::CameraPlugin,
            map::MapPlugin,
            highlighted::VisualHighlightPlugin,
        ))
        .init_resource::<cursor::CursorPos>()
        .add_systems(First, cursor::update_cursor_pos)
        .add_systems(
            Update,
            cursor::add_highlight_label.in_set(highlighted::HighlightedSet),
        )
        .add_systems(PostStartup, gameplay::character::spawn_player);
    }
}

// TODO: Maybe move character spawn into a Startup, with a post move to starting pos
