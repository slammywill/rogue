use bevy::prelude::*;
use rogue::game::GamePlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            GamePlugin,
        ))
        .run();
}
