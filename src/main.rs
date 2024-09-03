use bevy::prelude::*;
use movement::MovementPlugin;
use player::PlayerPlugin;

mod movement;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(MovementPlugin)
        .run();
}
