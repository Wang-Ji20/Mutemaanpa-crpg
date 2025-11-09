use godot::classes::{
    Area2D, CircleShape2D, CollisionShape2D, Node2D, PhysicsRayQueryParameters2D,
};
use godot::obj::WithBaseField;
use godot::prelude::*;

/// Interaction component does the following things:
/// - Defines an interaction area
/// - Detects when the player enters/exits the area
/// - Provides an interaction prompt
/// - Checks for line of sight if required
#[derive(GodotClass)]
#[class(base = Node2D)]
struct InteractionComponent {
    base: Base<Node2D>,

    #[export]
    interaction_prompt: GString,
    #[export]
    requires_line_of_sight: bool,
    #[export]
    interaction_area_path: NodePath,

    player_in_range: Option<Gd<Node2D>>,
    interaction_area: Option<Gd<Area2D>>,
}

impl InteractionComponent {
    fn find_or_create_interaction_area(&mut self) -> Gd<Area2D> {
        // Case 1: explicit path
        if !self.interaction_area_path.is_empty()
            && self.base().has_node(&self.interaction_area_path)
        {
            let node = self
                .base()
                .get_node_as::<Area2D>(&self.interaction_area_path);
            return node;
        }

        // Case 2: find by name
        for child in self.base().get_children().iter_shared() {
            if let Ok(area) = child.try_cast::<Area2D>() {
                if area.get_name() == "InteractionArea".into() {
                    return area;
                }
            }
        }

        // Case 3: create ghost area
        godot_warn!("No InteractionArea found. Creating a ghost one.");
        let mut area = Area2D::new_alloc();
        area.set_name("InteractionArea");
        area.set_collision_layer(0);
        area.set_collision_mask(2);

        let mut shape_node = CollisionShape2D::new_alloc();
        let mut shape = CircleShape2D::new_gd();
        shape.set_radius(50.);
        shape_node.set_shape(&shape);
        area.add_child(&shape_node);

        self.base_mut().add_child(&area);

        area
    }

    fn has_line_of_sight(&self) -> bool {
        if let (Some(parent), Some(player)) = (self.base().get_parent(), &self.player_in_range) {
            let parent = parent.cast::<Node2D>();
            let player = player.clone();
            if let Some(mut world) = self.base().get_world_2d() {
                let mut space_state = world.get_direct_space_state().unwrap();

                let mut query = PhysicsRayQueryParameters2D::new_gd();
                query.set_from(parent.get_global_position());
                query.set_to(player.get_global_position());
                query.set_collision_mask(1);

                let result = space_state.intersect_ray(&query);
                return result.is_empty();
            }
        }
        false
    }

    fn _on_player_entered(&mut self, area: Gd<Area2D>) {
        if let Some(player) = area.get_parent() {
            let player2d = player.try_cast::<Node2D>().unwrap();
            self.signals().player_entered().emit(&player2d);
            self.player_in_range = Some(player2d);
        }
    }

    fn _on_player_exited(&mut self, _area: Gd<Area2D>) {
        let player = self.player_in_range.as_ref().unwrap().clone();
        self.signals().player_exited().emit(&player);
        self.player_in_range = None;
    }
}

#[godot_api]
impl InteractionComponent {
    #[signal]
    fn interacted(who: Gd<Node2D>);

    #[signal]
    fn player_entered(who: Gd<Node2D>);

    #[signal]
    fn player_exited(who: Gd<Node2D>);

    #[func]
    fn try_interact(&mut self) -> bool {
        if let Some(_player) = &self.player_in_range {
            if self.requires_line_of_sight && !self.has_line_of_sight() {
                return false;
            }
            let player = _player.clone();
            self.signals().interacted().emit(&player);
            true
        } else {
            false
        }
    }
}

#[godot_api]
impl INode2D for InteractionComponent {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            interaction_prompt: "Click the item to interact".into(),
            requires_line_of_sight: false,
            interaction_area_path: NodePath::from(""),
            player_in_range: None,
            interaction_area: None,
        }
    }

    fn ready(&mut self) {
        let area = self.find_or_create_interaction_area();
        area.signals()
            .area_entered()
            .connect_other(&self.to_gd(), Self::_on_player_entered);
        area.signals()
            .area_exited()
            .connect_other(&self.to_gd(), Self::_on_player_exited);

        self.interaction_area = Some(area);
    }
}
