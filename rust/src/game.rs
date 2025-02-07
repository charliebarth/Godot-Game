use std::collections::HashMap;

use godot::{
    classes::{InputEvent, InputMap, Timer},
    prelude::*,
};

const SCREEN_WIDTH: f32 = 1920.0;
const SCREEN_HEIGHT: f32 = 1080.0;
const HALF_SCREEN_HEIGHT: f32 = SCREEN_HEIGHT / 2.0;

use crate::{main_menu::MainMenu, map::Map, player::player::Player, split_screen::SplitScreen};

static mut GAME_MODE: Option<String> = None;

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
    map: Option<Gd<Map>>,
    /// The ID of the winning player.
    winning_player: i32,
    /// A collection of maps/levels that can be loaded.
    maps: HashMap<String, Gd<PackedScene>>,
    /// A reference to the main menu.
    main_menu: Option<Gd<MainMenu>>,
    /// The first split screen for odd numbered players
    split_screen_one: Gd<SplitScreen>,
    /// The second split screen for even numbered players
    split_screen_two: Gd<SplitScreen>,
    day: bool,
    /// The timer for the day/night cycle.
    day_night_timer: Gd<Timer>,
}

#[godot_api]
impl INode2D for Game {
    fn init(base: Base<Node2D>) -> Self {
        const CYCLE_LENGTH: f64 = 10.0;
        let mut day_night_timer = Timer::new_alloc();
        day_night_timer.set_wait_time(CYCLE_LENGTH);
        day_night_timer.set_autostart(true);

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
            split_screen_one: SplitScreen::new_alloc(),
            split_screen_two: SplitScreen::new_alloc(),
            day: true,
            day_night_timer,
        }
    }

    fn ready(&mut self) {
        Input::singleton().connect(
            "joy_connection_changed",
            &Callable::from_object_method(
                &self.base().get_node_as::<Game>("/root/Game"),
                "device_changed",
            ),
        );

        let map_one = load::<PackedScene>("res://scenes/map_one.tscn");
        self.maps.insert("MapOne".to_string(), map_one);

        // Create the two split screens
        // Clone on a Gd is just a new ref not a new instance
        let mut split_screen_one = self.split_screen_one.clone();
        let mut split_screen_two = self.split_screen_two.clone();
        split_screen_one.set_name("SplitScreenOne");
        split_screen_two.set_name("SplitScreenTwo");
        split_screen_two.set_position(Vector2::new(0.0, HALF_SCREEN_HEIGHT));

        self.base_mut().add_child(&split_screen_one);
        self.base_mut().add_child(&split_screen_two);
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
        let disconnect_button = "roll";
        if !self.started && event.is_pressed() {
            if !self.devices.contains(&device_id)
                && input_map.event_is_action(&event, &register_button)
            {
                self.register_player(device_id);
            } else if self.devices.contains(&device_id)
                && input_map.event_is_action(&event, disconnect_button)
            {
                self.disconnect_player(device_id);
            }
        }
    }
}

#[godot_api]
impl Game {
    fn register_player(&mut self, device_id: i32) {
        self.devices.push(device_id);
        self.current_player_id = self.devices.len() as i32;

        let player = self.player_scene.instantiate_as::<Player>();
        self.players.push(player.clone());

        let mut main_menu = self.get_main_menu();
        main_menu.bind_mut().add_player(self.current_player_id);
    }

    fn disconnect_player(&mut self, device_id: i32) {
        let mut main_menu = self.get_main_menu();
        main_menu.bind_mut().remove_player(self.current_player_id);
        let index = self
            .devices
            .iter()
            .position(|&r| r == device_id)
            .expect("Device not found");

        self.players.remove(index);
        self.devices.remove(index);
        self.current_player_id = self.devices.len() as i32;
    }

    fn get_main_menu(&mut self) -> Gd<MainMenu> {
        if self.main_menu.is_none() {
            self.main_menu = Some(self.base().get_node_as::<MainMenu>("MainMenu"));
        }
        self.main_menu
            .as_ref()
            .expect("MainMenu node not found")
            .clone()
    }

    pub fn get_game_mode() -> String {
        unsafe { GAME_MODE.clone().unwrap() }
    }

    fn set_game_mode(&mut self, mode: String) {
        unsafe { GAME_MODE = Some(mode) }
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
        // TODO - make this actually set the right game mode
        self.set_game_mode("last_player_standing".to_string());

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

    #[func]
    pub fn start_game(&mut self) {
        // First remove the main menu
        let main_menu = self.get_main_menu();
        self.base_mut().remove_child(&main_menu);

        // Next instantiate the map
        let map = self
            .maps
            .get("MapOne")
            .expect("Map not found")
            .instantiate_as::<Map>();
        self.set_map(map);

        // Set the name, device id, and player id for each player
        let mut players = self.players.clone();
        for (index, player) in players.iter_mut().enumerate() {
            let player_id = index as i32 + 1;
            player.set_name(format!("Player{}", player_id).as_str());

            let mut bound_player = player.bind_mut();
            bound_player.set_device_id(self.devices[index]);
            bound_player.set_player_id(player_id);
        }

        // Get references to split screens
        let mut split_screen_one = self.split_screen_one.clone();
        let mut split_screen_two = self.split_screen_two.clone();

        // Add level to first split screen, second will share world
        split_screen_one.bind_mut().add_level(self.get_map());
        split_screen_two
            .bind_mut()
            .add_world(split_screen_one.bind().get_world());

        // Adjust split screen sizes based on player count
        let mut odd_players = Vec::new();
        let mut even_players = Vec::new();

        for (i, player) in self.players.iter().enumerate() {
            if (i + 1) % 2 == 0 {
                even_players.push(player.clone());
            } else {
                odd_players.push(player.clone());
            }
        }

        // Set sizes and add players
        // The split screens will resize the individual panes based on the number of players added
        if self.players.len() == 1 {
            const SINGLE_PLAYER_ZOOM: Vector2 = Vector2::new(2.0, 2.0);
            self.players[0].bind_mut().set_zoom(SINGLE_PLAYER_ZOOM);
            split_screen_one.set_size(Vector2::new(SCREEN_WIDTH, SCREEN_HEIGHT));
            split_screen_one.bind_mut().add_players(odd_players);
        } else {
            split_screen_one.set_size(Vector2::new(SCREEN_WIDTH, HALF_SCREEN_HEIGHT));
            split_screen_two.set_size(Vector2::new(SCREEN_WIDTH, HALF_SCREEN_HEIGHT));
            split_screen_one.bind_mut().add_players(odd_players);
            split_screen_two.bind_mut().add_players(even_players);
        }

        // Set the position of the players to the spawn point
        for (index, player) in players.iter_mut().enumerate() {
            let player_id = index as i32 + 1;
            player.set_position(self.get_map().bind().get_spawn_point(player_id.to_string()));
        }

        self.started = true;
        self.day_night_cycle();
    }

    #[func]
    pub fn end_game(&mut self) {
        self.started = false;

        // End the day/night cycle and ensure all lights are at full brightness
        let day_night_timer = self.day_night_timer.clone();
        self.base_mut().remove_child(&day_night_timer);
        self.day = false;
        self.cycle_change();

        // Reset split screens to size 0,0
        self.split_screen_one.bind_mut().reset();
        self.split_screen_two.bind_mut().reset();

        self.reset_players();

        // Show winner screen
        let mut main_menu = self.get_main_menu();
        self.base_mut().add_child(&main_menu);
        main_menu
            .bind_mut()
            .add_notification(format!("Player {} wins!", self.winning_player));
    }

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
                player.bind_mut().set_disconnected(false);
            } else {
                player.bind_mut().set_disconnected(true);
            }
        } else if self.devices.contains(&device_id) && !self.started {
            self.disconnect_player(device_id);
        }
    }

    pub fn get_number_of_players(&self) -> i32 {
        self.players.len() as i32
    }

    pub fn set_map(&mut self, map: Gd<Map>) {
        self.map = Some(map);
    }

    pub fn get_map(&self) -> Gd<Map> {
        self.map.as_ref().expect("Map not found").clone()
    }

    fn reset_players(&mut self) {
        self.players.clear();
        self.current_player_id = 0;
        for device_id in self.devices.iter() {
            self.current_player_id += 1;
            let mut player = self.player_scene.instantiate_as::<Player>();
            player.bind_mut().set_device_id(device_id.clone());
            player.bind_mut().set_player_id(self.current_player_id);
            player.set_name(format!("Player{}", self.current_player_id).as_str());
            self.players.push(player);
        }
    }

    pub fn remove_player(&mut self, player_id: i32) {
        self.players.remove(player_id as usize - 1);

        if self.started && self.players.len() == 1 {
            let player = self.players.get(0).expect("Player not found");
            self.winning_player = player.bind().get_player_id();
            self.end_game();
        } else if self.started && self.players.len() == 0 {
            self.winning_player = player_id;
            self.end_game();
        }
    }

    fn day_night_cycle(&mut self) {
        let game = self.base().get_node_as::<Game>(".");
        self.day_night_timer.connect(
            "timeout",
            &Callable::from_object_method(&game, "cycle_change"),
        );

        let day_night_timer = self.day_night_timer.clone();
        self.base_mut().add_child(&day_night_timer);
    }

    #[func]
    pub fn cycle_change(&mut self) {
        const TRANSITION_TIME: f64 = 3.0;
        let brightness: f32 = if self.day { 0.05 } else { 1.0 };
        self.base_mut().emit_signal(
            "change_cycle_player",
            &[Variant::from(brightness), Variant::from(TRANSITION_TIME)],
        );

        let brigtness_map = if self.day { 0.4 } else { 1.0 };
        let scale_map = if self.day { 0.6 } else { 1.0 };
        self.base_mut().emit_signal(
            "change_cycle_map",
            &[
                Variant::from(brigtness_map),
                Variant::from(TRANSITION_TIME),
                Variant::from(scale_map),
            ],
        );

        self.day = !self.day;
    }

    #[signal]
    pub fn change_cycle_map(&self, light_level: f32, transition_time: f64, scale: f32);

    #[signal]
    pub fn change_cycle_player(&self, light_level: f32, transition_time: f64);
}
