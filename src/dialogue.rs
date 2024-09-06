use bevy::prelude::*;
use bevy_yarnspinner::prelude::{YarnProject, YarnSpinnerPlugin};
use bevy_yarnspinner_example_dialogue_view::ExampleYarnSpinnerDialogueViewPlugin;

/// There are two part in this plugin:
///
/// 1. save, store all dialogue information
/// 2. remember what the participant has said during the dialogue.
pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(YarnSpinnerPlugin::new());
        app.add_plugins(ExampleYarnSpinnerDialogueViewPlugin::new());
        app.add_systems(
            Update,
            spawn_dialogue_runner.run_if(resource_added::<YarnProject>),
        );
    }
}

fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    let mut dialogue_runner = project.create_dialogue_runner();
    // Start the dialog at the node with the title "Start"
    dialogue_runner.start_node("HelloWorld");
    commands.spawn(dialogue_runner);
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
