//! player_tin_light.rs
//!
//! This module defines the PlayerTinLight class, which represents a light source that is
//! used for Tin lighting.
//!
//! Author: Michael Imerman
//! Version: Spring 2025
use godot::{
    classes::{IPointLight2D, PointLight2D},
    prelude::*,
};


#[derive(GodotClass)]
#[class(base=PointLight2D)]
/// Represents a light in the game
pub struct PlayerTinLight {
    /// The base node of the PlayerLight
    base: Base<PointLight2D>,
    /// The default energy of the light
    energy: f32,
}

#[godot_api]
impl IPointLight2D for PlayerTinLight {
    /// The constructor for the PlayerTinLight class.
    ///
    /// # Arguments
    /// * `base` - The base node of the PlayerLight.
    ///
    /// # Returns
    /// A new instance of PlayerTinLight.
    fn init(base: Base<PointLight2D>) -> Self {
        Self { base, energy: 1.0 }
    }

    /// The Godot method that is called when the node is added to the scene.
    fn ready(&mut self) {
        self.energy = self.base().get_energy();
    }
}

#[godot_api]
impl PlayerTinLight {
    /// Adjusts the light level of the player's tin light.
    ///
    /// # Arguments
    /// * `light_level` - The target light level.
    /// * `transition_time` - The time it takes to transition to the target light level.
    #[func]
    pub fn adjust_tin_light(&mut self, light_level: f32, transition_time: f64) {
        let target_energy = light_level * self.energy;

        let mut tween = self
            .base_mut()
            .create_tween()
            .expect("Failed to create tween");

        tween.tween_property(
            &self.base_mut().get_node_as::<PlayerTinLight>("."),
            "energy",
            &Variant::from(target_energy),
            transition_time,
        );
    }
}
