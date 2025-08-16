use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_camera);
    }
}

#[derive(Component)]
#[require(Camera2d)]
pub struct MainCamera;

fn initialize_camera(mut commands: Commands) {
    commands.spawn(MainCamera);
}

