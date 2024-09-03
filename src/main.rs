use bevy::prelude::*;
use dm::DMPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;

mod dm;
mod movement;
mod npc;
mod player;
mod dialogue;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DMPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(MovementPlugin)
        .run();
}
