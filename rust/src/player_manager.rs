use godot::{
    classes::{InputEvent, InputMap, Marker2D},
    prelude::*,
};

use crate::player::player::Player;
#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct PlayerManager {
    base: Base<Node2D>,
    /// A map of input device IDs to players.
    players: Vec<i32>,
    register_button: StringName,
    player_scene: Gd<PackedScene>,
}

#[godot_api]
impl INode2D for PlayerManager {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            players: Vec::new(),
            register_button: "jump".into(),
            player_scene: load::<PackedScene>("res://scenes/player.tscn"),
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let device = event.get_device();
        let input_map = InputMap::singleton();
        let register_button = self.register_button.clone();

        if !self.players.contains(&device)
            && input_map.event_is_action(event.clone(), register_button)
        {
            let mut player = self.player_scene.instantiate_as::<Player>();
            let mut level = self.base().get_parent().unwrap();
            let spawn_position = self.spawn_player(device);

            player.bind_mut().set_device_id(device);
            player.set_position(spawn_position);

            level.add_child(player);

            self.players.push(device);
        }
    }
}

impl PlayerManager {
    fn spawn_player(&self, player_id: i32) -> Vector2 {
        let level = self.base().get_parent().unwrap();

        let spawn_point_name = match player_id {
            0 => "SpawnOne",
            1 => "SpawnTwo",
            2 => "SpawnThree",
            3 => "SpawnFour",
            _ => "SpawnOne",
        };

        let spawn_point = level.get_node_as::<Marker2D>(spawn_point_name);

        spawn_point.get_position()
    }
}
