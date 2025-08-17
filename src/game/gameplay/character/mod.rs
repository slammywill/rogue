use bevy::prelude::*;

mod character;
mod movement;
pub use character::Character;

#[derive(Default)]
pub struct PlayerSprite {
    pub position: Vec2,
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Sprite {
            image: asset_server.load("textures/tiles/grass.png"),
            anchor: bevy::sprite::Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(0., 0., 1.),
    ));
}
