use bevy::prelude::*;

/// There are two part in this plugin:
///
/// 1. save, store all dialogue information
/// 2. remember what the participant has said during the dialogue.
pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        let hardcode = vec![Dialogue {
            id: 0,
            speaker: "monologue".to_string(),
            text: "hello, world".to_string(),
        }];
        app.insert_resource(DialogueRepository(hardcode));
    }
}

#[derive(Debug, Clone, Resource)]
pub struct DialogueRepository(Vec<Dialogue>);

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Dialogue {
    id: usize,
    speaker: String,
    text: String,
}

struct DialogueBuilder {
    cur_id: usize,
}

impl DialogueBuilder {}
