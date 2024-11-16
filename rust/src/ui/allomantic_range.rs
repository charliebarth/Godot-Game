use std::borrow::Borrow;

use godot::classes::{Area2D, CharacterBody2D, IArea2D};
/// Represents an Allomantic Line.
///
/// Author : Trinity Pittman
/// Version : 10/27/2024
use godot::prelude::*;

use crate::items::coin::Coin;
use crate::items::metal_vial::MetalVial;
use crate::ui::allomantic_line::AllomanticLine;

/// Struct that represents an Allomantic Range
#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct AllomanticRange {
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for AllomanticRange {
    /// Constructor for the Allomantic Range
    fn init(base: Base<Area2D>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {}
}

#[godot_api]
impl AllomanticRange {
    /// When something enters this hitbox we...
    ///
    /// Args:
    ///      body (Gd<Node2D>): the Node that enters this hitbox  
    #[func]
    fn enter_range(&mut self, body: Gd<Node2D>) {
        let body_name = body.get_name();
        godot_print!("Alomantic range entered by {body_name}");

        // Find this nodes parent for later
        let parent: Gd<CharacterBody2D> = self
            .base_mut()
            .get_owner()
            .unwrap()
            .cast::<CharacterBody2D>();
        let metal_potential: Gd<Node2D> = body.clone();

        if body.has_method(StringName::from("is_metal")) {
            godot_print!("IS METAL (entering): {}", body_name);
            let mut line: Gd<AllomanticLine> = AllomanticLine::new_alloc();
            line.set_visible(true);
            line.bind_mut().initialize_fields(metal_potential, parent);
            // line.set_modulate(Color { r: (1.5), g: (1.5), b: (1.5), a: (1.) });
            // line.set_self_modulate(Color { r: (1.5), g: (1.5), b: (1.5), a: (1.) });
            // line.set_position(Vector2::new(0.0,0.0));
            // line.bind_mut().draw();

            // line.bind_mut().setup();
            self.base_mut().add_child(line);
        } else {
            godot_print!("Something other than a metal object entered the allomantic range.");
        }
    }

    #[func]
    fn exit_range(&mut self, body: Gd<Node2D>) {
        let body_name = body.get_name();
        godot_print!("Alomantic range exited by {body_name}");

        let metal: Gd<Node2D> = body.clone();

        let children: Array<Gd<Node>> = self.base_mut().get_children();
        for i in 0..children.len() {
            // Go through the children and find the line
            let child: Gd<Node> = children.get(i).expect("");
            // godot_print!("CHILD NAME {}", child.get_name());
            if let Ok(mut line) = child.try_cast::<AllomanticLine>() {
                if line.bind_mut().get_metal().unwrap().get_name() == metal.get_name() {
                    line.queue_free();
                    godot_print!("REMOVED METAL {}", metal.get_name())
                }
            }
        }
    }
}
