use std::ops::Index;

use godot::{
    classes::{
        viewport::DefaultCanvasItemTextureFilter, HBoxContainer, InputEvent, InputMap, Marker2D,
        SubViewport, SubViewportContainer,
    },
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

        if event.is_pressed()
            && !self.players.contains(&device)
            && input_map.event_is_action(event.clone(), register_button)
        {
            let mut player = self.player_scene.instantiate_as::<Player>();
            let mut root = self.base().get_parent().unwrap();

            self.split_screen();

            self.players.push(device);
            let player_id = self
                .players
                .iter()
                .position(|&x| x == device)
                .expect("Player missing") as i32;
            let spawn_position = self.select_spawn_point(player_id);
            player.bind_mut().set_device_id(device);
            player.set_position(spawn_position);

            if self.players.len() == 1 {
                let camera = root.get_node_as::<Camera2D>("OverviewCamera");
                root.remove_child(camera);
            }

            self.assign_player_to_subviewport(player, player_id);
        }
    }
}

impl PlayerManager {
    fn select_spawn_point(&self, player_id: i32) -> Vector2 {
        let level = self.base().get_parent().unwrap();

        let spawn_point_name = match player_id {
            0 => "SpawnOne",
            1 => "SpawnTwo",
            2 => "SpawnThree",
            3 => "SpawnFour",
            _ => "SpawnOne",
        };

        let spawn_point = level.get_node_as::<Marker2D>(format!(
            "SplitScreenOne/PlayerOneContainer/PlayerOneViewport/MapOne/{}",
            spawn_point_name
        ));

        spawn_point.get_position()
    }

    fn assign_player_to_subviewport(&self, player: Gd<Player>, player_id: i32) {
        let root = self.base().get_parent().unwrap();
        let mut subviewport: Gd<SubViewport>;
        match player_id {
            0 => {
                subviewport = root.get_node_as::<SubViewport>(
                    "SplitScreenOne/PlayerOneContainer/PlayerOneViewport",
                );
            }
            1 => {
                subviewport = root.get_node_as::<SubViewport>(
                    "SplitScreenOne/PlayerTwoContainer/PlayerTwoViewport",
                );
            }
            2 => {
                subviewport = root.get_node_as::<SubViewport>(
                    "SplitScreenTwo/PlayerThreeContainer/PlayerThreeViewport",
                );
            }
            3 => {
                subviewport = root.get_node_as::<SubViewport>(
                    "SplitScreenTwo/PlayerFourContainer/PlayerFourViewport",
                );
            }
            _ => return,
        }

        subviewport.add_child(player);
    }
    fn split_screen(&self) {
        let root = self.base().get_parent().unwrap();

        match self.players.len() {
            0 => self.one_player_split_screen(root),
            1 => self.two_player_split_screen(root),
            2 => self.three_player_split_screen(root),
            3 => self.four_player_split_screen(root),
            _ => {}
        }
    }

    fn one_player_split_screen(&self, mut root: Gd<Node>) {
        let mut split_screen_one = HBoxContainer::new_alloc();
        let mut p1_container = SubViewportContainer::new_alloc();
        let mut p1_viewport = SubViewport::new_alloc();

        split_screen_one.set_name("SplitScreenOne".into());
        p1_container.set_name("PlayerOneContainer".into());
        p1_viewport.set_name("PlayerOneViewport".into());
        p1_viewport.set_default_canvas_item_texture_filter(DefaultCanvasItemTextureFilter::NEAREST);

        split_screen_one.set_size(Vector2::new(1920.0, 1080.0));
        p1_container.set_size(Vector2::new(1920.0, 1080.0));
        p1_viewport.set_size(Vector2i::new(1920, 1080));

        p1_container.add_child(p1_viewport);
        split_screen_one.add_child(p1_container);
        root.add_child(split_screen_one);

        self.reparent_level();
    }

    fn two_player_split_screen(&self, mut root: Gd<Node>) {
        let mut split_screen_one = root.get_node_as::<HBoxContainer>("SplitScreenOne");
        let mut p1_container =
            split_screen_one.get_node_as::<SubViewportContainer>("PlayerOneContainer");
        let mut p1_viewport = p1_container.get_node_as::<SubViewport>("PlayerOneViewport");

        p1_container.set_size(Vector2::new(960.0, 1080.0));
        p1_viewport.set_size(Vector2i::new(960, 1080));

        let mut p2_container = SubViewportContainer::new_alloc();
        let mut p2_viewport = SubViewport::new_alloc();

        p2_container.set_name("PlayerTwoContainer".into());
        p2_viewport.set_name("PlayerTwoViewport".into());
        p2_viewport.set_default_canvas_item_texture_filter(DefaultCanvasItemTextureFilter::NEAREST);
        p2_viewport.set_world_2d(p1_viewport.get_world_2d());

        p2_container.set_size(Vector2::new(960.0, 1080.0));
        p2_viewport.set_size(Vector2i::new(960, 1080));

        p2_container.add_child(p2_viewport);
        split_screen_one.add_child(p2_container);
    }

    fn three_player_split_screen(&self, mut root: Gd<Node>) {
        let mut split_screen_one = root.get_node_as::<HBoxContainer>("SplitScreenOne");
        let mut p1_container =
            split_screen_one.get_node_as::<SubViewportContainer>("PlayerOneContainer");
        let mut p1_viewport = p1_container.get_node_as::<SubViewport>("PlayerOneViewport");

        p1_container.set_size(Vector2::new(960.0, 540.0));
        p1_viewport.set_size(Vector2i::new(960, 540));
        split_screen_one.set_size(Vector2::new(1920.0, 540.0));

        let mut p2_container =
            split_screen_one.get_node_as::<SubViewportContainer>("PlayerTwoContainer");
        let mut p2_viewport = p2_container.get_node_as::<SubViewport>("PlayerTwoViewport");

        p2_container.set_size(Vector2::new(960.0, 540.0));
        p2_viewport.set_size(Vector2i::new(960, 540));

        let mut split_screen_two = HBoxContainer::new_alloc();
        let mut p3_container = SubViewportContainer::new_alloc();
        let mut p3_viewport = SubViewport::new_alloc();

        split_screen_two.set_name("SplitScreenTwo".into());
        p3_container.set_name("PlayerThreeContainer".into());
        p3_viewport.set_name("PlayerThreeViewport".into());
        p3_viewport.set_default_canvas_item_texture_filter(DefaultCanvasItemTextureFilter::NEAREST);
        p3_viewport.set_world_2d(p2_viewport.get_world_2d());

        split_screen_two.set_size(Vector2::new(1920.0, 540.0));
        p3_container.set_size(Vector2::new(1920.0, 540.0));
        p3_viewport.set_size(Vector2i::new(1920, 540));
        split_screen_two.set_position(Vector2::new(0.0, 542.0));

        p3_container.add_child(p3_viewport);
        split_screen_two.add_child(p3_container);
        root.add_child(split_screen_two);
    }

    fn four_player_split_screen(&self, mut root: Gd<Node>) {
        let mut split_screen_two = root.get_node_as::<HBoxContainer>("SplitScreenTwo");
        let mut p3_container =
            split_screen_two.get_node_as::<SubViewportContainer>("PlayerThreeContainer");
        let mut p3_viewport = p3_container.get_node_as::<SubViewport>("PlayerThreeViewport");

        p3_container.set_size(Vector2::new(960.0, 540.0));
        p3_viewport.set_size(Vector2i::new(960, 540));

        let mut p4_container = SubViewportContainer::new_alloc();
        let mut p4_viewport = SubViewport::new_alloc();

        p4_container.set_name("PlayerFourContainer".into());
        p4_viewport.set_name("PlayerFourViewport".into());
        p4_viewport.set_default_canvas_item_texture_filter(DefaultCanvasItemTextureFilter::NEAREST);
        p4_viewport.set_world_2d(p3_viewport.get_world_2d());

        p4_container.set_size(Vector2::new(960.0, 540.0));
        p4_viewport.set_size(Vector2i::new(960, 540));

        p4_container.add_child(p4_viewport);
        split_screen_two.add_child(p4_container);
    }

    fn reparent_level(&self) {
        let root = self.base().get_parent().unwrap();
        let mut level = root.get_node_as::<Node2D>("MapOne");
        let p1_viewport =
            root.get_node_as::<SubViewport>("SplitScreenOne/PlayerOneContainer/PlayerOneViewport");
        level.reparent(p1_viewport);
    }
}
