use bevy::prelude::*;

mod character;
mod movement;
pub use character::Character;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let character = commands.spawn(Character::default()).id();


}
