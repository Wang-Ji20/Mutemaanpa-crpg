use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Npc {
    pub dialogue: Vec<String>
}
