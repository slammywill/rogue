use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_camera);
        app.add_systems(Update, zoom_control_system);
    }
}

#[derive(Component)]
#[require(Camera2d)]
pub struct MainCamera;

fn initialize_camera(mut commands: Commands) {
    commands.spawn(MainCamera);
}

fn zoom_control_system(
    mut evr_scroll: EventReader<MouseWheel>,
    mut projection: Single<&mut Projection, With<MainCamera>>,
) {
    let Projection::Orthographic(ortho) = projection.as_mut() else {
        return;
    };

    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                // Line is for hardware scroll e.g. wheel on mouse
                ortho.scale += 0.2 * ev.y;
            }
            MouseScrollUnit::Pixel => {
                // Pixel is for smooth scroll e.g. trackpad
                ortho.scale += 0.2 * ev.y;
            }
        }
    }
    ortho.scale = ortho.scale.clamp(0.2, 2.0);
}
