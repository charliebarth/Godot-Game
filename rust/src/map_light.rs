//! map_light.rs
//!
//! This file contains the MapLight class, which is responsible for managing the light
//! levels in the game. It includes functions for transitioning the light levels and
//! setting the light energy.
//!
//! Author: Charles Barth
//! Version: Spring 2025
use godot::{
    classes::{IPointLight2D, PointLight2D},
    prelude::*,
};

use crate::game::Game;

#[derive(GodotClass)]
#[class(base=PointLight2D)]
/// Represents a light in the game
pub struct MapLight {
    /// The base node of the MapLight
    base: Base<PointLight2D>,
    /// The default energy of the light
    energy: f32,
    scale: f32,
}

#[godot_api]
/// IPointLight2D methods for the MapLight
impl IPointLight2D for MapLight {
    /// The Godot constructor for the MapLight class node
    ///
    /// # Arguments
    /// * `base` - The base node type for the MapLight
    ///
    /// # Returns
    /// * `MapLight` - The MapLight node
    fn init(base: Base<PointLight2D>) -> Self {
        Self {
            base,
            energy: 1.0,
            scale: 1.0,
        }
    }

    /// The Godot ready function for the MapLight class node
    /// This run when the node first enters the scene tree
    fn ready(&mut self) {
        self.energy = self.base().get_energy();
        self.scale = self.base().get_scale().x;

        let mut game = self.base().get_node_as::<Game>("/root/Game");
        let light = self.base().get_node_as::<MapLight>(".");
        game.connect(
            "change_cycle_map",
            &Callable::from_object_method(&light, "transition_light_levels"),
        );
    }
}

#[godot_api]
/// Methods for the MapLight
impl MapLight {
    /// This function transitions the light levels of the MapLight node
    ///
    /// # Arguments
    /// * `light_level` - The light level to transition to
    /// * `transition_time` - The time it takes to transition to the new light level
    /// * `scale` - The scale of the light
    #[func]
    pub fn transition_light_levels(&mut self, light_level: f32, transition_time: f64, scale: f32) {
        let target_energy = light_level * self.energy;
        let target_scale = scale * self.scale;
        let target_scale_vec = Vector2::new(target_scale, target_scale);

        let mut tween = self
            .base_mut()
            .create_tween()
            .expect("Failed to create tween");

        tween.tween_property(
            &self.base_mut().get_node_as::<MapLight>("."),
            "energy",
            &Variant::from(target_energy),
            transition_time,
        );

        let mut tween = self
            .base_mut()
            .create_tween()
            .expect("Failed to create tween");

        tween.tween_property(
            &self.base_mut().get_node_as::<MapLight>("."),
            "scale",
            &Variant::from(target_scale_vec),
            transition_time,
        );
    }
}
