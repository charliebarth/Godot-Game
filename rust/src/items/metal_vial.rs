use godot::classes::{Area2D, AudioStreamPlayer2D, IArea2D};
/// Represents a Metal Vial.
///
/// Author : Trinity Pittman
/// Version : Fall 2024
use godot::prelude::*;

use crate::player::player::Player;

const OFF_MAP: Vector2 = Vector2::new(-100000., 100000.);

#[derive(GodotClass)]
#[class(base=Area2D)]
/// Represents a Metal Vial
pub struct MetalVial {
    // The base node of the MetalVial
    base: Base<Area2D>,
    /// A vector of metals this metal vial increments
    metals: Vec<&'static str>,
    /// The amt the metal vial increments the metals by
    amt: f64,
}

#[godot_api]
/// Godot methods for MetalVial
impl IArea2D for MetalVial {
    /// The Godot contructor for the MetalVial class node
    ///
    /// # Arguments
    /// * `base` - The base node type for the MetalVial
    ///
    /// # Returns
    /// * `MetalVial` - The MetalVial node
    fn init(base: Base<Area2D>) -> Self {
        let mut metals = Vec::new();
        metals.push("iron");
        metals.push("steel");
        metals.push("pewter");

        Self {
            base,
            metals,
            amt: 30.,
        }
    }
}

#[godot_api]
// Methods owned by the MetalVial
impl MetalVial {
    /// When someone enters this metal vial hit box we call the method to add metal to that players  
    /// metal bars.
    ///
    /// # Arguments
    /// * `body` (`Gd<Node2D>`) - the Node that enters this metal vial.
    #[func]
    fn metal_pickup(&mut self, body: Gd<Node2D>) {
        let body_name = body.get_name();
        godot_print!("Metal entered by {body_name}"); // Prints who picked up the coin

        if let Ok(mut player) = body.try_cast::<Player>() {
            player.bind_mut().adjust_metals(self.get_metals(), self.amt); // Dereference and call the method

            self.play_sound(player);

            // Set position
            // let pos = Vector2::new(100000., -100000.);
            self.base_mut().set_global_position(OFF_MAP);

            // self.base_mut().queue_free(); // Remove the vial from the scene
        } else {
            godot_print!("Something other than player entered the coin.");
        }
    }

    /// Gets the metals this metal vial increments
    ///
    /// # Returns
    /// * The metals the vials increment
    fn get_metals(&mut self) -> Vec<&'static str> {
        self.metals.clone()
    }

    /// Sets the metals this metal vial will increment
    ///
    /// # Arguments
    /// * `metals` (`Vec<StringName>`) - the names of the metals to set
    pub fn set_metals(&mut self, metals: Vec<&'static str>) {
        self.metals = metals
    }

    fn play_sound(&mut self, player: Gd<Player>) {
        // Play swallowing sound
        let mut sound_effect: Gd<AudioStreamPlayer2D> = player.get_node_as("./Vial");

        sound_effect.play();
    }
}
