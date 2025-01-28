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

use crate::{main_menu::MainMenu, player::player::Player};

/// The Game class is responsible for managing the game state such as players, maps, and the main menu.
/// This is also the root node of the scene tree.
#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Game {
    /// The base node of the Game.
    base: Base<Node2D>,
    /// A map of input device IDs to players.
    players: Vec<Gd<Player>>,
    /// A list of connected input devices.
    devices: Vec<i32>,
    /// The name of the button that players must press to register.
    register_button: StringName,
    /// The scene for the player node.
    player_scene: Gd<PackedScene>,
    /// The ID of the next player to register.
    current_player_id: i32,
    /// Whether the game has started.
    started: bool,
    /// The current map.
    map: Option<Gd<Node2D>>,
    /// The ID of the winning player.
    winning_player: i32,
    /// A collection of maps/levels that can be loaded.
    maps: HashMap<String, Gd<PackedScene>>,
    /// A reference to the main menu.
    main_menu: Option<Gd<MainMenu>>,
}

#[godot_api]
impl INode2D for Game {
    /// The Godot constructor for the Game class.
    ///
    /// # Arguments
    /// * `base` - The base node of the Game.
    ///
    /// # Returns
    /// * `Game` - A new instance of the Game class.
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            players: Vec::new(),
            devices: Vec::new(),
            register_button: "jump".into(),
            player_scene: load::<PackedScene>("res://scenes/player.tscn"),
            current_player_id: 0,
            started: false,
            map: None,
            winning_player: 0,
            maps: HashMap::new(),
            main_menu: None,
        }
    }

    /// This is a built in method for Godot that is called when a node is first added to the scene.
    /// This registers signal from the built in input singleton for when a device is connected or disconnected.
    /// It also loads the maps that can be used in the game.
    fn ready(&mut self) {
        Input::singleton().connect(
            "joy_connection_changed".into(),
            Callable::from_object_method(
                &self.base().get_node_as::<Game>("/root/Game"),
                "device_changed",
            ),
        );

        let map_one = load::<PackedScene>("res://scenes/map_one.tscn");
        self.maps.insert("MapOne".to_string(), map_one);
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
        let disconnect_button = "roll".into();
        if !self.started && event.is_pressed() {
            if !self.devices.contains(&device_id)
                && input_map.event_is_action(event.clone(), register_button)
            {
                self.register_player(device_id);
            } else if self.devices.contains(&device_id)
                && input_map.event_is_action(event.clone(), disconnect_button)
            {
                self.disconnect_player(device_id);
            }
        }
    }
}

#[godot_api]
impl Game {
    /// This will register a player to the game.
    /// Registering a player will create a new player object and assign it the device id and player id.
    /// Players will be assigned a name when the game starts.
    ///
    /// Arguments:
    /// * `device_id` - The device id of the player to register.
    fn register_player(&mut self, device_id: i32) {
        self.devices.push(device_id);
        self.current_player_id = self.devices.len() as i32;

        let player = self.player_scene.instantiate_as::<Player>();
        self.players.push(player.clone());

        let mut main_menu = self.get_main_menu();
        main_menu.bind_mut().add_player(self.current_player_id);
    }

    /// This will disconnect a player from the game.
    /// Disconnecting a player will remove them from the game and shift all still connected players up.
    /// For example if player 2 is disconnected player 3 will become player 2 and player 4 will become player 3.
    /// The device associated with the disconnected player will be removed from the list of connected devices as well.
    ///
    /// Arguments:
    /// * `device_id` - The device id of the player to disconnect.
    fn disconnect_player(&mut self, device_id: i32) {
        let mut main_menu = self.get_main_menu();
        main_menu.bind_mut().remove_player(self.current_player_id);
        // The device id index will also be the index of the appropriate player
        let index = self
            .devices
            .iter()
            .position(|&r| r == device_id)
            .expect("Device not found");

        self.players.remove(index);
        self.devices.remove(index);
        self.current_player_id = self.devices.len() as i32;
    }

    /// This will get the main menu node.
    ///
    /// Returns:
    /// * `MainMenu` - The main menu node.
    fn get_main_menu(&mut self) -> Gd<MainMenu> {
        if self.main_menu.is_none() {
            self.main_menu = Some(self.base().get_node_as::<MainMenu>("MainMenu"));
        }

        self.main_menu
            .as_ref()
            .expect("MetalLine node not found")
            .clone()
    }

    /// This will attempt to start the game.
    /// It will check if the appropriate conditions are met to start the game.
    ///
    /// Arguments:
    /// * `test_mode` - A boolean that determines if the game should only launch with exactly 1 player.
    ///
    /// Note: If test mode is true the game will only start if there is exactly 1 player. Otherwise the game will start only if there are at least 2 players.
    #[func]
    pub fn attempt_start(&mut self, test_mode: bool) {
        if !test_mode && self.players.len() >= 2 || (test_mode && self.players.len() == 1) {
            self.start_game();
            return;
        }

        let notification = (if test_mode {
            "Incorrect number of players for test mode. Need exactly 1 player."
        } else {
            "Not enough players to start game."
        })
        .to_string();

        self.get_main_menu()
            .bind_mut()
            .add_notification(notification);
    }

    /// This will start the game.
    /// Note: This will remove the main menu and instantiate the map.
    #[func]
    pub fn start_game(&mut self) {
        // First remove the main menu
        let main_menu = self.get_main_menu();
        self.base_mut().remove_child(main_menu);

        // Next instantiate the map, add it to the tree, and trigger it's camera pan (might make the map auto camera pan on ready/load)
        // Note: Currently defaulting to MapOne
        let map = self
            .maps
            .get("MapOne")
            .expect("Map not found")
            .instantiate_as::<Node2D>();

        // TODO: Call map method to trigger a camera pan across the map. Once that method has returned start the game.
        // The player manager will reparent the map under the first subviewport and will remove it and all it's children at the end of the game.
        self.set_map(map);

        self.started = true;
        let mut players = self.players.clone();
        for (index, player) in players.iter_mut().enumerate() {
            let player_id = index as i32 + 1;
            player.set_name(format!("Player{}", player_id).into());

            let mut bound_player = player.bind_mut();
            bound_player.set_device_id(self.devices[index]);
            bound_player.set_player_id(player_id);
            drop(bound_player);

            self.split_screen(player_id);
            self.assign_player_to_subviewport(player.clone(), player_id);
            self.adjust_player_camera_zoom(player_id);

            let spawn_position = self.select_spawn_point(player_id);
            player.set_position(spawn_position);
        }
    }

    /// This will end the game.
    /// It will clean up all the split screen viewports and players.
    /// After cleanup returns the main menu and shows a message stating who won the game.
    #[func]
    pub fn end_game(&mut self) {
        self.started = false;
        for mut child in self.base_mut().get_children().iter_shared() {
            if child.get_name().to_string().starts_with("SplitScreen") {
                child.queue_free();
            }
        }

        self.reset_players();

        // Show a winner screen
        // Next add the main menu back
        let mut main_menu = self.get_main_menu();
        self.base_mut().add_child(main_menu.clone());
        main_menu
            .bind_mut()
            .add_notification(format!("Player {} wins!", self.winning_player));
    }

    /// This function is called when a device is connected or disconnected.
    /// It is hooked up to the joy_connection_changed signal from the Input singleton.
    /// If a device is connected and the player it will reconnect the player to the device.
    /// If a device is disconnected it will disconnect the player from the device and remove them from the game.
    ///
    /// Arguments:
    /// * `device_id` - The id of the device that has changed.
    /// * `connected` - A boolean that determines if the device is connected or disconnected.
    #[func]
    pub fn device_changed(&mut self, device_id: i32, connected: bool) {
        if self.devices.contains(&device_id) && self.started {
            let index = self
                .devices
                .iter()
                .position(|&r| r == device_id)
                .expect("Device not found");

            let mut player = self.players.get(index).expect("Player not found").clone();

            if connected {
                player.bind_mut().set_disconnected(false); // tells the player they are no longer disconnected
            } else {
                player.bind_mut().set_disconnected(true); // tells the player they are disconnected
            }
        } else if self.devices.contains(&device_id) && !self.started {
            self.disconnect_player(device_id);
        }
    }

    /// This function returns the number of players in the game.
    ///
    /// Returns:
    /// * `i32` - The number of players in the game.
    pub fn get_number_of_players(&self) -> i32 {
        self.players.len() as i32
    }

    /// This function selects the map for the game.
    ///
    /// Arguments:
    /// * `map` - The map to select.
    pub fn set_map(&mut self, map: Gd<Node2D>) {
        self.map = Some(map);
    }

    /// This function resets the players in the game.
    /// It cleans up any old data and reinstaniates the players.
    fn reset_players(&mut self) {
        self.players.clear();
        self.current_player_id = 0;
        for device_id in self.devices.iter() {
            self.current_player_id += 1;
            let mut player = self.player_scene.instantiate_as::<Player>();
            player.bind_mut().set_device_id(device_id.clone());
            player.bind_mut().set_player_id(self.current_player_id);
            player.set_name(format!("Player{}", self.current_player_id).into());
            self.players.push(player);
        }
    }

    /// This will disconnect a player from the game.
    /// If there is only one player left in the game they will be declared the winner.
    ///
    /// Arguments:
    /// * `player_id` - The id of the player to disconnect.
    pub fn remove_player(&mut self, player_id: i32) {
        // self.players.remove(player_id as usize - 1);
        //
        // if self.started && self.players.len() == 1 {
        //     let player = self.players.get(0).expect("Player not found");
        //     self.winning_player = player.bind().get_player_id();
        //     self.end_game();
        // } else if self.started && self.players.len() == 0 {
        //     self.winning_player = player_id;
        //     self.end_game();
        // }

        let player_index = player_id as usize - 1;
        let player = self.players.get(player_index).expect("Player not found");

        // TODO: Increment the number of eliminations of the player that eliminated this player
        // how to get the player that eliminated this player?

        // check if a player has reached the required elimination count
        if self.check_win_condition() {
            self.end_game()
        } else {
            self.players.remove(player_index);

            if self.players.len() == 1 {
                self.start_new_round();
            }
        }
    }

    /// This will check if a player has reached the required elimination count.
    ///
    /// Returns:
    /// * `bool` - True if a player has reached the required elimination count, false otherwise.
    fn check_win_condition(&self) -> bool {
        // The number of eliminations required to win the game; could/should be changed to be more dynamic in the future
        const REQUIRED_ELIMINATIONS: i32 = 10;
        // check if any player has reached the required number of eliminations
        self.players.iter().any(|player| player.bind().get_eliminations() >= REQUIRED_ELIMINATIONS)
    }

    /// This will start a new round. It will reset the players and start the game again.
    ///
    fn start_new_round(&mut self) {
        self.respawn_players();
        self.start_game();
    }

    /// This will select a spawn point for a player based on their player id.
    ///
    /// Arguments:
    /// * `player_id` - The id of the player to select a spawn point for.
    ///
    /// Returns:
    /// * `Vector2` - The spawn point for the player.
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

    /// This will assign a player to a subviewport/split screen pane.
    ///
    /// # Arguments
    /// * `player` - The player to assign to the subviewport.
    /// * `player_id` - The id of the player to assign to the subviewport.
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

    /// This will adjust the camera zoom for a player based on the number of players and thus size of each split screen pane.
    ///
    /// # Arguments
    /// * `player_id` - The id of the player to adjust the camera zoom for.
    fn adjust_player_camera_zoom(&self, player_id: i32) {
        if player_id == 2 {
            self.adjust_two_player_camera_zoom();
        } else if player_id == 3 {
            self.adjust_three_player_camera_zoom();
        } else if player_id == 4 {
            self.adjust_four_player_camera_zoom();
        }
    }

    /// This will adjust the camera zoom for a two player split screen.
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

    /// This will adjust the camera zoom for a three player split screen.
    fn adjust_three_player_camera_zoom(&self) {
        let mut camera3 = self.base().get_node_as::<Camera2D>(
            "SplitScreenOne/PlayerThreeContainer/PlayerThreeViewport/Player3/Camera2D",
        );

        camera3.set_zoom(Vector2::new(1.0, 1.0));
    }

    /// This will adjust the camera zoom for a four player split screen.
    fn adjust_four_player_camera_zoom(&self) {
        let mut camera4 = self.base().get_node_as::<Camera2D>(
            "SplitScreenTwo/PlayerFourContainer/PlayerFourViewport/Player4/Camera2D",
        );

        camera4.set_zoom(Vector2::new(1.0, 1.0));
    }

    /// This will split the screen based on the number of players.
    ///
    /// # Arguments
    /// * `player_id` - The id of the player to split the screen for.
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

    /// This will add the first viewport for a single player. The viewport will be fullscreen.
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

    /// This will resize the screen to fullscreen for a single player.
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

    /// This will add the second viewport for a two player split screen.
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

    /// This will resize the screen to fit two players.
    /// Each split screen will be the full width of the screen and half the height.
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

    /// This will add the third and fourth viewports for a three player split screen.
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

    /// This will resize the screen to fit three players.
    /// Each split screen will be half the width of the screen and half the height.
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

    /// This will add the fourth viewport for a four player split screen.
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

    /// This will resize the screen to fit four players.
    /// Each split screen will be half the width of the screen and half the height.
    fn assign_four_player_screen_sizes(&self) {
        let mut split_screen_two = self.base().get_node_as::<HBoxContainer>("SplitScreenTwo");
        let mut p4_container =
            split_screen_two.get_node_as::<SubViewportContainer>("PlayerFourContainer");
        let mut p4_viewport = p4_container.get_node_as::<SubViewport>("PlayerFourViewport");

        p4_viewport.set_size(Vector2i::new(960, 540));
        p4_container.set_size(Vector2::new(960.0, 540.0));
        split_screen_two.set_anchors_preset(LayoutPreset::TOP_LEFT);
    }

    /// This will reparent the map to the first subviewport.
    /// This is required so that the map can be seen by the players.
    fn reparent_level(&mut self) {
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

    /// This will remove the map overview camera from the fourth viewport.
    /// This camera is placed there during a three player game to provide an overview of the entire level.
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
}
