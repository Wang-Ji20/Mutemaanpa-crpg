use godot::classes::{CharacterBody2D, ICharacterBody2D, Input};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    speed: f64,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Hello, world!");
        Self { speed: 400.0, base }
    }

    fn physics_process(&mut self, delta: f64) {
        let input_direction =
            Input::singleton().get_vector("move_left", "move_right", "move_up", "move_down");

        let delta_velocity = input_direction * self.speed as f32 * delta as f32;

        self.base_mut().move_and_collide(delta_velocity);
    }
}
