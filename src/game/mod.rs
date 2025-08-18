use bevy::prelude::*;

pub mod camera;
pub mod map;
pub mod gameplay;
pub mod cursor;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            camera::CameraPlugin,
            map::MapPlugin,
        ))
            .init_resource::<cursor::CursorPos>()
            .add_systems(First, cursor::update_cursor_pos)
            .add_systems(Update, cursor::highlight_tile)
            .add_systems(PostStartup, gameplay::character::spawn_player);
    }
}

// TODO: Maybe move character spawn into a Startup, with a post move to starting pos
