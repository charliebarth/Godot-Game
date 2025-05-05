//! metal_pickups.rs
//!
//! This file contains the implementation of the MetalPickup class, which is responsible for
//! creating and managing metal pickups in the game. It handles the spawning of metal vials
//! and their behavior when the timer times out.
//!
//! Author: Trinity Pittman, Charles Barth
//! Version: Spring 2025
use godot::classes::{Engine, IMarker2D, Marker2D, Timer};
use godot::prelude::*;

use crate::items::metal_vial::MetalVial;
use crate::settings::Settings;

/// The time between vial spawns
const WAIT_TIME: f64 = 30.0;
/// Where a vial is moved to on pickup
const OFF_MAP: Vector2 = Vector2::new(-100000., 100000.);

/// Represents a Metal Pickup
#[derive(GodotClass)]
#[class(base=Marker2D)]
pub struct MetalPickup {
    /// The base node of the Metal Pickup.
    base: Base<Marker2D>,
    /// The metal vial object corresponding to this Metal Pickup.
    metal_vial: Option<Gd<MetalVial>>,
}

/// Godot methods for the MetalPickup
#[godot_api]
impl IMarker2D for MetalPickup {
    /// Constructor for the Metal Pickup.
    ///
    /// # Arguments
    /// * `base` (Base<Marker2D>) - The base node of the metal pickup.
    fn init(base: Base<Marker2D>) -> Self {
        Self {
            base,
            metal_vial: None,
        }
    }

    /// Called when this node enters the scene tree. Makes the metal vial and
    /// starts the timer for spawning.
    fn ready(&mut self) {
        self.make_vial();
        self.get_metal_vial().set_global_position(OFF_MAP);

        let mut timer: Gd<Timer> = self.base().get_node_as("./Timer");
        timer.set_autostart(true);
        timer.set_wait_time(WAIT_TIME);
        timer.set_one_shot(false);
        timer.start();
    }
}

/// Methods for the Metal Pickup
#[godot_api]
impl MetalPickup {
    /// Makes a vial based on the current game mode.
    fn make_vial(&mut self) {
        // Get the metal vial scene and instantiate it
        let metal_scene = load::<PackedScene>("res://scenes/metal_vial.tscn");
        let mut metal = metal_scene.instantiate_as::<MetalVial>().clone();
        metal.set_name("MetalVialPickup");
        metal.set_visible(true);

        // Find the game mode and set metals based on the mode
        let mut new_metals = Vec::new();

        let mode = self.find_game_mode();

        if mode == "Last Player Standing".to_string() {
            // TODO set what we want the vials to incr
            new_metals.push("pewter");
            new_metals.push("iron");
            new_metals.push("steel");
        } else if mode == "Head Hunters".to_string() {
            new_metals.push("pewter");
            new_metals.push("iron");
            new_metals.push("steel");
        }

        metal.bind_mut().set_metals(new_metals);

        self.metal_vial = Some(metal);

        // Add metal vial to node tree
        let vial = self.get_metal_vial();
        self.base_mut().add_child(&vial);
    }

    /// Getter method for the metal vial.
    ///
    /// # Returns
    /// * (Gd<MetalVial>) - The metal vial if it exists.
    fn get_metal_vial(&mut self) -> Gd<MetalVial> {
        self.metal_vial
            .as_ref()
            .expect("Could not find metal vial")
            .clone()
    }

    /// Finds this games game mode from the settings.
    ///
    /// # Returns
    /// * (String) - A string representing the game mode.
    fn find_game_mode(&mut self) -> String {
        let settings = Engine::singleton()
            .get_singleton("Settings")
            .expect("settings singleton missing")
            .try_cast::<Settings>()
            .expect("settings is not a Settings");

        let mode = settings.bind().get_game_mode();

        mode
    }

    /// Called when the timer ends. If the metal vial was picked up, it is moved
    /// back to the pickup location.
    #[func]
    fn on_timer_timeout(&mut self) {
        if self.get_metal_vial().get_global_position() == OFF_MAP {
            self.get_metal_vial()
                .set_global_position(self.base().get_global_position());
        }
    }
}
