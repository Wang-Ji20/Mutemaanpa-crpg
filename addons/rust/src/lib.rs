use godot::prelude::*;

mod player;
mod interaction_component;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
