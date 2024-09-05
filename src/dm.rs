use bevy::prelude::*;
use rand::Rng;

use crate::dialogue::{Dialogue, DialogueRepository, DialogueState};

/// DM
///
/// generally dm can do everything. But here DM only do very limited thing:
///
/// act as a virtual character, occasionally say things to player.
pub struct DMPlugin;

impl Plugin for DMPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Dice);
        app.add_systems(Startup, dm_setup);
        app.add_systems(FixedUpdate, dm_say);
    }
}

/// Dice
#[derive(Debug, Clone, Copy, Resource)]
pub struct Dice;

impl Dice {
    /// Roll a dice, producing result between [1, max], all are inclusive.
    pub fn roll(&self, max: u32) -> u32 {
        let mut rand = rand::thread_rng();
        rand.gen_range(1..=max)
    }
}

/// DM as a virtual NPC
fn dm_setup(mut command: Commands) {
    command.spawn((
        DialogueRepository(vec![Dialogue::new("DM", "hello, world")]),
        DialogueState(0),
    ));
}

/// DM can say things. When it says, that thing is logged to the console.
fn dm_say(
    dialogue_repo: Query<&DialogueRepository>,
    mut state: Query<&mut DialogueState>,
    dice: Res<Dice>,
) {
    if state.single().0 >= dialogue_repo.single().0.len() {
        return;
    }
    let say = dialogue_repo.single().0.get(state.single().0).unwrap();
    let pt = dice.roll(20);
    info!("{:?}, rolled {:?}", say, pt);
    state.single_mut().0 += 1;
}
