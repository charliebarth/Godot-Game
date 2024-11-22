use std::collections::HashMap;

use godot::{
    classes::{
        control::LayoutPreset, viewport::DefaultCanvasItemTextureFilter, HBoxContainer, InputEvent,
        InputMap, Marker2D, SubViewport, SubViewportContainer,
    },
    prelude::*,
};

const FULLSCREEN_WIDTH: f32 = 1920.0;
const FULLSCREEN_HEIGHT: f32 = 1080.0;

const TWO_PLAYER_WIDTH: f32 = 1920.0;
const TWO_PLAYER_HEIGHT: f32 = 540.0;

const THREE_PLAYER_WIDTH: f32 = 960.0;
const THREE_PLAYER_HEIGHT: f32 = 540.0;

use crate::{
    game::Game,
    main_menu::{self, MainMenu},
    player::player::Player,
};
#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct PlayerManager {
    base: Base<Node2D>,
    /// A map of input device IDs to players.
    players: Vec<Gd<Player>>,
    devices: Vec<i32>,
    register_button: StringName,
    player_scene: Gd<PackedScene>,
    current_player_id: i32,
    started: bool,
    num_alive_players: i32,
    map: Option<Gd<Node2D>>,
    game: Option<Gd<Game>>,
}

#[godot_api]
impl INode2D for PlayerManager {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            players: Vec::new(),
            devices: Vec::new(),
            register_button: "jump".into(),
            player_scene: load::<PackedScene>("res://scenes/player.tscn"),
            current_player_id: 0,
            started: false,
            num_alive_players: 0,
            map: None,
            game: None,
        }
    }

    fn process(&mut self, _delta: f64) {
        if self.started && self.num_alive_players == 1 {
            self.end_game();
        }
    }

    /// This listens for a specific button press (jump by default)
    /// When the button is pressed if it is the first time that device has pressed the button
    /// the device id will be saved and a player will be created and assigned that device id.
    ///
    /// Note: This does not spawn players or start the game simply tracks players and their device ids.
    /// Players can be spawned with the start method.
    ///
    /// Arguments:
    /// * `event` - The input event that triggered this method.
    fn input(&mut self, event: Gd<InputEvent>) {
        let device_id = event.get_device();

        let input_map = InputMap::singleton();
        let register_button = self.register_button.clone();

        if !self.started
            && event.is_pressed()
            && !self.devices.contains(&device_id)
            && input_map.event_is_action(event.clone(), register_button)
        {
            self.current_player_id += 1;
            let mut player = self.player_scene.instantiate_as::<Player>();
            player.bind_mut().set_device_id(device_id);
            player.bind_mut().set_player_id(self.current_player_id);
            player.set_name(format!("Player{}", self.current_player_id).into());
            self.players.push(player);
            self.devices.push(device_id);

            let mut main_menu = self
                .base()
                .get_parent()
                .unwrap()
                .get_node_as::<MainMenu>("MainMenu");

            main_menu.bind_mut().add_player(self.current_player_id);
            self.num_alive_players += 1;
        }
    }
}

#[godot_api]
impl PlayerManager {
    pub fn get_number_of_players(&self) -> i32 {
        self.players.len() as i32
    }

    pub fn set_map(&mut self, map: Gd<Node2D>) {
        self.map = Some(map);
    }

    fn end_game(&mut self) {
        self.started = false;
        for mut child in self.base_mut().get_children().iter_shared() {
            child.queue_free();
        }

        self.get_game()
            .bind_mut()
            .end_game(1, self.players.len() as i32);
    }

    #[func]
    /// This function should be called when a gamemode is started as it will spawn the players and give them their own viewports.
    pub fn start(&mut self) {
        self.started = true;
        let mut players = self.players.clone();
        for player in players.iter_mut() {
            let player_id = player.bind().get_player_id();

            self.split_screen(player_id);
            self.assign_player_to_subviewport(player.clone(), player_id);
            self.adjust_player_camera_zoom(player_id);

            let spawn_position = self.select_spawn_point(player_id);
            player.set_position(spawn_position);
        }
    }

    pub fn remove_player(&mut self, player_id: i32, device_id: i32) {
        self.num_alive_players -= 1;
        self.players.remove(player_id as usize - 1);
        self.devices.retain(|&id| id != device_id);
    }

    fn select_spawn_point(&self, player_id: i32) -> Vector2 {
        let spawn_point_name = match player_id {
            1 => "SpawnOne",
            2 => "SpawnTwo",
            3 => "SpawnThree",
            4 => "SpawnFour",
            _ => "SpawnOne",
        };

        let spawn_point = self.base().get_node_as::<Marker2D>(format!(
            "SplitScreenOne/PlayerOneContainer/PlayerOneViewport/MapOne/{}",
            spawn_point_name
        ));

        spawn_point.get_position()
    }

    fn assign_player_to_subviewport(&self, player: Gd<Player>, player_id: i32) {
        let mut subviewport: Gd<SubViewport>;
        match player_id {
            1 => {
                subviewport = self.base().get_node_as::<SubViewport>(
                    "SplitScreenOne/PlayerOneContainer/PlayerOneViewport",
                );
            }
            2 => {
                subviewport = self.base().get_node_as::<SubViewport>(
                    "SplitScreenTwo/PlayerTwoContainer/PlayerTwoViewport",
                );
            }
            3 => {
                subviewport = self.base().get_node_as::<SubViewport>(
                    "SplitScreenOne/PlayerThreeContainer/PlayerThreeViewport",
                );
            }
            4 => {
                subviewport = self.base().get_node_as::<SubViewport>(
                    "SplitScreenTwo/PlayerFourContainer/PlayerFourViewport",
                );
            }
            _ => return,
        }

        subviewport
            .set_canvas_cull_mask(1 << player_id as u32 * 2 | 1 | 1 << player_id as u32 * 2 - 1); // player_id * 2, player_id * 2 + 1, and layer 1
        subviewport.add_child(player);
    }

    fn adjust_player_camera_zoom(&self, player_id: i32) {
        if player_id == 2 {
            self.adjust_two_player_camera_zoom();
        } else if player_id == 3 {
            self.adjust_three_player_camera_zoom();
        } else if player_id == 4 {
            self.adjust_four_player_camera_zoom();
        }
    }

    fn adjust_two_player_camera_zoom(&self) {
        let mut camera1 = self.base().get_node_as::<Camera2D>(
            "SplitScreenOne/PlayerOneContainer/PlayerOneViewport/Player1/Camera2D",
        );
        let mut camera2 = self.base().get_node_as::<Camera2D>(
            "SplitScreenTwo/PlayerTwoContainer/PlayerTwoViewport/Player2/Camera2D",
        );

        camera1.set_zoom(Vector2::new(1.0, 1.0));
        camera2.set_zoom(Vector2::new(1.0, 1.0));
    }

    fn adjust_three_player_camera_zoom(&self) {
        let mut camera3 = self.base().get_node_as::<Camera2D>(
            "SplitScreenOne/PlayerThreeContainer/PlayerThreeViewport/Player3/Camera2D",
        );

        camera3.set_zoom(Vector2::new(1.0, 1.0));
    }

    fn adjust_four_player_camera_zoom(&self) {
        let mut camera4 = self.base().get_node_as::<Camera2D>(
            "SplitScreenTwo/PlayerFourContainer/PlayerFourViewport/Player4/Camera2D",
        );

        camera4.set_zoom(Vector2::new(1.0, 1.0));
    }

    fn split_screen(&mut self, player_id: i32) {
        match player_id {
            1 => self.one_player_split_screen(),
            2 => self.two_player_split_screen(),
            3 => {
                self.three_player_split_screen();
                self.four_player_split_screen();
                self.add_fourth_viewport_camera();
            }
            4 => self.remove_fourth_viewport_camera(),
            _ => {}
        }
    }

    fn one_player_split_screen(&mut self) {
        let mut split_screen_one = HBoxContainer::new_alloc();
        let mut p1_container = SubViewportContainer::new_alloc();
        let mut p1_viewport = SubViewport::new_alloc();
        p1_viewport.set_use_hdr_2d(true);
        p1_viewport.set_as_audio_listener_2d(true);

        split_screen_one.set_name("SplitScreenOne".into());
        p1_container.set_name("PlayerOneContainer".into());
        p1_viewport.set_name("PlayerOneViewport".into());
        p1_viewport.set_default_canvas_item_texture_filter(DefaultCanvasItemTextureFilter::NEAREST);

        p1_container.add_child(p1_viewport);
        split_screen_one.add_child(p1_container);
        self.base_mut().add_child(split_screen_one);

        self.reparent_level();
        self.assign_one_player_screen_sizes();
    }

    fn assign_one_player_screen_sizes(&self) {
        let mut split_screen_one = self.base().get_node_as::<HBoxContainer>("SplitScreenOne");
        let mut p1_container =
            split_screen_one.get_node_as::<SubViewportContainer>("PlayerOneContainer");
        let mut p1_viewport = p1_container.get_node_as::<SubViewport>("PlayerOneViewport");

        p1_viewport.set_size(Vector2i::new(
            FULLSCREEN_WIDTH as i32,
            FULLSCREEN_HEIGHT as i32,
        ));
        p1_container.set_size(Vector2::new(FULLSCREEN_WIDTH, FULLSCREEN_HEIGHT));
        split_screen_one.set_size(Vector2::new(FULLSCREEN_WIDTH, FULLSCREEN_HEIGHT));
    }

    fn two_player_split_screen(&mut self) {
        let p1_viewport = self
            .base()
            .get_node_as::<SubViewport>("SplitScreenOne/PlayerOneContainer/PlayerOneViewport");

        let mut split_screen_two = HBoxContainer::new_alloc();
        let mut p2_container = SubViewportContainer::new_alloc();
        let mut p2_viewport = SubViewport::new_alloc();
        p2_viewport.set_use_hdr_2d(true);
        p2_viewport.set_as_audio_listener_2d(true);

        split_screen_two.set_name("SplitScreenTwo".into());
        p2_container.set_name("PlayerTwoContainer".into());
        p2_viewport.set_name("PlayerTwoViewport".into());
        p2_viewport.set_default_canvas_item_texture_filter(DefaultCanvasItemTextureFilter::NEAREST);
        p2_viewport.set_world_2d(p1_viewport.get_world_2d());
        split_screen_two.set_position(Vector2::new(0.0, 542.0));

        p2_container.add_child(p2_viewport);
        split_screen_two.add_child(p2_container);
        self.base_mut().add_child(split_screen_two);

        self.assign_two_player_screen_sizes();
    }

    fn assign_two_player_screen_sizes(&self) {
        let mut split_screen_one = self.base().get_node_as::<HBoxContainer>("SplitScreenOne");
        let mut p1_container =
            split_screen_one.get_node_as::<SubViewportContainer>("PlayerOneContainer");
        let mut p1_viewport = p1_container.get_node_as::<SubViewport>("PlayerOneViewport");

        p1_viewport.set_size(Vector2i::new(
            TWO_PLAYER_WIDTH as i32,
            TWO_PLAYER_HEIGHT as i32,
        ));
        p1_container.set_size(Vector2::new(TWO_PLAYER_WIDTH, TWO_PLAYER_HEIGHT));
        split_screen_one.set_size(Vector2::new(TWO_PLAYER_WIDTH, TWO_PLAYER_HEIGHT));
        split_screen_one.set_anchors_preset(LayoutPreset::CENTER_TOP);

        let mut split_screen_two = self.base().get_node_as::<HBoxContainer>("SplitScreenTwo");
        let mut p2_container =
            split_screen_two.get_node_as::<SubViewportContainer>("PlayerTwoContainer");
        let mut p2_viewport = p2_container.get_node_as::<SubViewport>("PlayerTwoViewport");

        p2_viewport.set_size(Vector2i::new(
            TWO_PLAYER_WIDTH as i32,
            TWO_PLAYER_HEIGHT as i32,
        ));
        p2_container.set_size(Vector2::new(TWO_PLAYER_WIDTH, TWO_PLAYER_HEIGHT));
        split_screen_two.set_size(Vector2::new(TWO_PLAYER_WIDTH, TWO_PLAYER_HEIGHT));
        split_screen_two.set_anchors_preset(LayoutPreset::CENTER_TOP);
    }

    fn three_player_split_screen(&mut self) {
        let mut split_screen_one = self.base().get_node_as::<HBoxContainer>("SplitScreenOne");
        let p1_viewport =
            split_screen_one.get_node_as::<SubViewport>("PlayerOneContainer/PlayerOneViewport");

        let mut p3_container = SubViewportContainer::new_alloc();
        let mut p3_viewport = SubViewport::new_alloc();
        p3_viewport.set_use_hdr_2d(true);
        p3_viewport.set_as_audio_listener_2d(true);

        p3_container.set_name("PlayerThreeContainer".into());
        p3_viewport.set_name("PlayerThreeViewport".into());
        p3_viewport.set_default_canvas_item_texture_filter(DefaultCanvasItemTextureFilter::NEAREST);
        p3_viewport.set_world_2d(p1_viewport.get_world_2d());

        p3_container.add_child(p3_viewport);
        split_screen_one.add_child(p3_container);

        self.assign_three_player_screen_sizes();
    }

    fn assign_three_player_screen_sizes(&self) {
        let split_screen_one = self.base().get_node_as::<HBoxContainer>("SplitScreenOne");
        let mut p1_container =
            split_screen_one.get_node_as::<SubViewportContainer>("PlayerOneContainer");
        let mut p1_viewport = p1_container.get_node_as::<SubViewport>("PlayerOneViewport");

        p1_viewport.set_size(Vector2i::new(
            THREE_PLAYER_WIDTH as i32,
            THREE_PLAYER_HEIGHT as i32,
        ));
        p1_container.set_size(Vector2::new(THREE_PLAYER_WIDTH, THREE_PLAYER_HEIGHT));

        let mut p3_container =
            split_screen_one.get_node_as::<SubViewportContainer>("PlayerThreeContainer");
        let mut p3_viewport = p3_container.get_node_as::<SubViewport>("PlayerThreeViewport");

        p3_viewport.set_size(Vector2i::new(
            THREE_PLAYER_WIDTH as i32,
            THREE_PLAYER_HEIGHT as i32,
        ));
        p3_container.set_size(Vector2::new(THREE_PLAYER_WIDTH, THREE_PLAYER_HEIGHT));

        let split_screen_two = self.base().get_node_as::<HBoxContainer>("SplitScreenTwo");
        let mut p2_container =
            split_screen_two.get_node_as::<SubViewportContainer>("PlayerTwoContainer");
        let mut p2_viewport = p2_container.get_node_as::<SubViewport>("PlayerTwoViewport");

        p2_viewport.set_size(Vector2i::new(
            THREE_PLAYER_WIDTH as i32,
            THREE_PLAYER_HEIGHT as i32,
        ));
        p2_container.set_size(Vector2::new(THREE_PLAYER_WIDTH, THREE_PLAYER_HEIGHT));
    }

    fn four_player_split_screen(&self) {
        let mut split_screen_two = self.base().get_node_as::<HBoxContainer>("SplitScreenTwo");
        let p2_viewport =
            split_screen_two.get_node_as::<SubViewport>("PlayerTwoContainer/PlayerTwoViewport");

        let mut p4_container = SubViewportContainer::new_alloc();
        let mut p4_viewport = SubViewport::new_alloc();
        p4_viewport.set_use_hdr_2d(true);
        p4_viewport.set_as_audio_listener_2d(true);

        p4_container.set_name("PlayerFourContainer".into());
        p4_viewport.set_name("PlayerFourViewport".into());
        p4_viewport.set_default_canvas_item_texture_filter(DefaultCanvasItemTextureFilter::NEAREST);
        p4_viewport.set_world_2d(p2_viewport.get_world_2d());

        p4_container.add_child(p4_viewport);
        split_screen_two.add_child(p4_container);

        self.assign_four_player_screen_sizes();
    }

    fn assign_four_player_screen_sizes(&self) {
        let mut split_screen_two = self.base().get_node_as::<HBoxContainer>("SplitScreenTwo");
        let mut p4_container =
            split_screen_two.get_node_as::<SubViewportContainer>("PlayerFourContainer");
        let mut p4_viewport = p4_container.get_node_as::<SubViewport>("PlayerFourViewport");

        p4_viewport.set_size(Vector2i::new(960, 540));
        p4_container.set_size(Vector2::new(960.0, 540.0));
        split_screen_two.set_anchors_preset(LayoutPreset::TOP_LEFT);
    }

    fn reparent_level(&mut self) {
        godot_print!("Reparenting level");
        if let Some(map) = self.map.take() {
            let mut p1_viewport = self
                .base()
                .get_node_as::<SubViewport>("SplitScreenOne/PlayerOneContainer/PlayerOneViewport");
            p1_viewport.add_child(map);
        } else {
            godot_error!("Map not found. Unable to start game.");
            self.base().get_tree().expect("Tree not found").quit();
        }
    }

    fn remove_fourth_viewport_camera(&self) {
        let mut overview_container = self
            .base()
            .get_node_as::<SubViewport>("SplitScreenTwo/PlayerFourContainer/PlayerFourViewport");
        let camera = overview_container.get_node_as::<Camera2D>("OverviewCamera");

        overview_container.remove_child(camera);
    }

    /// This will add a camera that provides an overview of the entire level.
    fn add_fourth_viewport_camera(&self) {
        let mut camera = Camera2D::new_alloc();
        camera.set_name("OverviewCamera".into());
        camera.set_position(Vector2::new(20.0, -225.0));
        camera.set_zoom(Vector2::new(0.37, 0.37));

        let mut overview_container = self
            .base()
            .get_node_as::<SubViewport>("SplitScreenTwo/PlayerFourContainer/PlayerFourViewport");

        overview_container.set_canvas_cull_mask(1);
        overview_container.add_child(camera);
    }

    fn get_game(&mut self) -> Gd<Game> {
        if self.game.is_none() {
            self.game = Some(self.base().get_node_as::<Game>("/root/Game"));
        }

        self.game
            .as_ref()
            .expect("MetalLine node not found")
            .clone()
    }
}
