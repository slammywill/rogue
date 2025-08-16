use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use rogue::game::GamePlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Rogue"),
                    ..Default::default()
                }),
                ..default()
            }),
            TilemapPlugin,
            GamePlugin,
        ))
        .run();
}
