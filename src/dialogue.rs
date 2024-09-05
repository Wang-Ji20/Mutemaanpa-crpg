use bevy::prelude::*;

/// There are two part in this plugin:
///
/// 1. save, store all dialogue information
/// 2. remember what the participant has said during the dialogue.
pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Debug, Clone, Component)]
pub struct DialogueRepository(pub Vec<Dialogue>);

#[derive(Debug, Clone, Component)]
pub struct DialogueState(pub usize);

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Dialogue {
    speaker: String,
    text: String,
}

impl Dialogue {
    pub fn new(speaker: impl ToString, text: impl ToString) -> Dialogue {
        Dialogue {
            speaker: speaker.to_string(),
            text: text.to_string(),
        }
    }
}
