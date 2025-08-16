use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_map);
    }
}

fn initialize_map(mut commands: Commands) {
    todo!();
}
