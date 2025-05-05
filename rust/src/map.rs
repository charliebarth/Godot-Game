//! map.rs
//!
//! This file contains the Map class, which is responsible for managing the spawn points in the game.
//! It includes functions for initializing the spawn points and retrieving their positions.
//!
//! Author: Charles Barth
//! Version: Spring 2025
use std::collections::HashMap;

use godot::{
    classes::{INode2D, Marker2D, Node2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Node2D)]
/// Map class responsible for managing spawn points
pub struct Map {
    /// The base node of the Map 
    base: Base<Node2D>,
    /// A HashMap of spawn point names to positions
    spawn_points: HashMap<String, Vector2>,
}

#[godot_api]
/// INode2D methods for the Map
impl INode2D for Map {
    /// The Godot constructor for the Map class.
    ///
    /// # Arguments
    /// * `base` - The base node of the Map.
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            spawn_points: HashMap::new(),
        }
    }

    /// This function is called when the Map node is ready.
    fn ready(&mut self) {
        let spawn_points = self.base().get_node_as::<Node>("SpawnPoints");
        for child in spawn_points.get_children().iter_shared() {
            let marker = child.cast::<Marker2D>();
            let name = marker.get_name().to_string();
            let position = marker.get_position();
            self.spawn_points.insert(name, position);
        }
    }
}

#[godot_api]
/// Methods for the Map
impl Map {
    /// This function retrieves the spawn point for a given player.
    ///
    /// # Arguments
    /// * `name` - The name of the spawn point.
    #[func]
    pub fn get_spawn_point(&self, name: String) -> Vector2 {
        godot_print!("Spawn point: {:?}", self.spawn_points);
        self.spawn_points
            .get(&name)
            .expect("Spawn point not found")
            .clone()
    }
}
