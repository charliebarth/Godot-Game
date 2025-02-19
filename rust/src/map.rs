use std::collections::HashMap;

use godot::{
    classes::{INode2D, Marker2D, Node2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Map {
    base: Base<Node2D>,
    spawn_points: HashMap<String, Vector2>,
}

#[godot_api]
impl INode2D for Map {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            spawn_points: HashMap::new(),
        }
    }

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
impl Map {
    #[func]
    pub fn get_spawn_point(&self, name: String) -> Vector2 {
        self.spawn_points
            .get(&name)
            .expect("Spawn point not found")
            .clone()
    }
}
