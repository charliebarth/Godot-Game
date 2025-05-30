//! player_light.rs
//!
//! This file contains the implementation of the PlayerLight class, which is responsible for
//! creating and managing the light effect in the game with respect to the player's light point.
//!
//! Author: Charles Barth
//! Version: Spring 2025
use godot::{
    classes::{IPointLight2D, PointLight2D},
    prelude::*,
};

use crate::game::Game;

/// Represents a light in the game
#[derive(GodotClass)]
#[class(base=PointLight2D)]
pub struct PlayerLight {
    /// The base node of the PlayerLight
    base: Base<PointLight2D>,
    /// The default energy of the light
    energy: f32,
}

/// IPointLight2D methods for the PlayerLight
#[godot_api]
impl IPointLight2D for PlayerLight {
    /// The Godot constructor for the PlayerLight class node
    ///
    /// # Arguments
    /// * `base` - The base node type for the PlayerLight
    ///
    /// # Returns
    /// * `PlayerLight` - The PlayerLight node
    fn init(base: Base<PointLight2D>) -> Self {
        Self { base, energy: 1.0 }
    }

    /// This method is called when the node is added to the scene tree.
    /// It gets the enery, game, and light and connects the lights signal to the method
    /// `change_cycle_player`.
    fn ready(&mut self) {
        self.energy = self.base().get_energy();

        let mut game = self.base().get_node_as::<Game>("/root/Game");
        let light = self.base().get_node_as::<PlayerLight>(".");
        game.connect(
            "change_cycle_player",
            &Callable::from_object_method(&light, "transition_light_levels"),
        );
    }
}

/// Methods for the PlayerLight
#[godot_api]
impl PlayerLight {
    /// This function transitions the light levels of the PlayerLight node
    ///
    /// # Arguments
    /// * `light_level` - The light level to transition to
    /// * `transition_time` - The time it takes to transition to the new light level
    #[func]
    pub fn transition_light_levels(&mut self, light_level: f32, transition_time: f64) {
        let target_energy = light_level * self.energy;

        let mut tween = self
            .base_mut()
            .create_tween()
            .expect("Failed to create tween");

        tween.tween_property(
            &self.base_mut().get_node_as::<PlayerLight>("."),
            "energy",
            &Variant::from(target_energy),
            transition_time,
        );
    }
}
