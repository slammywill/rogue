use bevy::prelude::*;

pub mod camera;
pub mod map;
pub mod gameplay;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            camera::CameraPlugin,
            map::MapPlugin,
        ));
    }
}
