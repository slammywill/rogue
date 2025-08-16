use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use rogue::game::GamePlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            TilemapPlugin,
            GamePlugin,
        ))
        .run();
}
