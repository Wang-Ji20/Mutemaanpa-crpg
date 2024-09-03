use bevy::prelude::*;
use rand::Rng;

/// DM
///
/// generally dm can do everything. But here DM only do very limited thing:
/// 1. roll dice
/// 2. act as a virtual character, occasionally say things to player.
/// 3. log the events
pub struct DMPlugin;

impl Plugin for DMPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Dice);
    }
}

#[derive(Debug, Clone, Copy, Resource)]
pub struct Dice;

impl Dice {
    /// Roll a dice, producing result between [1, max], all are inclusive.
    pub fn roll(&self, max: u32) -> u32 {
        let mut rand = rand::thread_rng();
        rand.gen_range(1..=max)
    }
}
