use std::collections::HashMap;

use godot::{
    classes::{DisplayServer, Engine, InputEvent, InputMap, Timer},
    prelude::*,
};
use godot::classes::Label;
use godot::global::HorizontalAlignment;
use crate::{
    main_menu::MainMenu,
    map::Map,
    player::player::Player,
    settings::Settings,
    split_screen::{self, SplitScreen},
};

static mut GAME_MODE: Option<String> = None;
// The number of eliminations required to win the game; could/should be changed to be more dynamic in the future
const REQUIRED_ELIMINATIONS: i32 = 5;
// The number of rounds required to win the game
const REQUIRED_ROUNDS: i32 = 3;

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
    /// The size of the screen
    screen_size: Vector2,
    /// The settings for the game
    settings: Gd<Settings>,
    /// The number of kills for each player
    eliminations: HashMap<i32, i32>,
    /// The number of round wins each player has
    round_wins: HashMap<i32, i32>,
    /// A flag to determine if a new round should be started
    should_start_new_round: bool,
    /// Timer for round transition
    round_transition_timer: Gd<Timer>,
    /// Label to display the winner message
    winner_label: Option<Gd<Label>>,
}

#[godot_api]
impl INode2D for Game {
    fn init(base: Base<Node2D>) -> Self {
        const CYCLE_LENGTH: f64 = 10.0;
        let mut day_night_timer = Timer::new_alloc();
        day_night_timer.set_wait_time(CYCLE_LENGTH);
        day_night_timer.set_autostart(true);

        const ROUND_TRANSITION_TIME: f64 = 3.0;
        let mut round_transition_timer = Timer::new_alloc();
        // set the transition time to 3 seconds
        round_transition_timer.set_wait_time(ROUND_TRANSITION_TIME);
        // set the timer to be one shot so it doesn't repeat
        round_transition_timer.set_one_shot(true);

        let screen_size = DisplayServer::singleton().screen_get_size();
        let settings = Engine::singleton()
            .get_singleton("Settings")
            .expect("settings singleton missing")
            .try_cast::<Settings>()
            .expect("settings is not a Settings");

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
            screen_size: Vector2::new(screen_size.x as f32, screen_size.y as f32),
            settings,
            eliminations: HashMap::new(),
            round_wins: HashMap::new(),
            should_start_new_round: false,
            round_transition_timer,
            winner_label: None,
        }
    }

    /// This is a builtin method for Godot that is called when the node is added to the scene tree.
    /// This is where the signals are connected and the maps are loaded.
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

        let map_two = load::<PackedScene>("res://scenes/map_two.tscn");
        self.maps.insert("MapTwo".to_string(), map_two);

        let map_three = load::<PackedScene>("res://scenes/map_three.tscn");
        self.maps.insert("MapThree".to_string(), map_three);

        // Create the two split screens
        // Clone on a Gd is just a new ref not a new instance
        let mut split_screen_one = self.split_screen_one.clone();
        let mut split_screen_two = self.split_screen_two.clone();
        split_screen_one.set_name("SplitScreenOne");
        split_screen_two.set_name("SplitScreenTwo");
        split_screen_two.set_position(Vector2::new(0.0, self.screen_size.y / 2.0));

        self.base_mut().add_child(&split_screen_one);
        self.base_mut().add_child(&split_screen_two);

        // Add the round transition timer to the scene tree
        let mut timer = self.round_transition_timer.clone();
        self.base_mut().add_child(&timer);

        let target = self.base().clone();
         timer.connect(
            "timeout",
            &Callable::from_object_method(
                &target, "round_transition"),
        );

        // Create the winner label
        let mut winner_label = Label::new_alloc();
        winner_label.set_visible(false);
        winner_label.set_position(Vector2::new(self.screen_size.x / 2.0, 50.0));
        winner_label.set_horizontal_alignment(HorizontalAlignment::CENTER);
        self.base_mut().add_child(&winner_label);
        self.winner_label = Some(winner_label);
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

    /// This is a built in method for Godot that is called every frame.
    ///
    /// Arguments:
    /// * `delta` - The time in seconds since the last frame.
    fn process(&mut self, _delta: f64) {
        if self.should_start_new_round {
            self.start_round();
            self.should_start_new_round = false;
        }
    }
}

#[godot_api]
impl Game {
    /// Reference viewport size for a single player pane at zoom 1.0
    /// This is the size of one viewport in a 4-player configuration on a 1920x1080 screen
    const REFERENCE_VIEWPORT: Vector2 = Vector2::new(960.0, 540.0);

    /// Calculates the appropriate zoom factor based on viewport size.
    /// Uses the minimum zoom value to maintain aspect ratio.
    ///
    /// Arguments:
    /// * `viewport_size` - The size of the viewport to calculate the zoom for.
    ///
    /// Returns:
    /// * Vector2 - The zoom factor for the viewport.
    fn calculate_zoom(&self, viewport_size: Vector2) -> Vector2 {
        let zoom_x = viewport_size.x / Self::REFERENCE_VIEWPORT.x;
        let zoom_y = viewport_size.y / Self::REFERENCE_VIEWPORT.y;
        let zoom = zoom_x.min(zoom_y);
        Vector2::new(zoom, zoom)
    }

    /// This will register a player to the game by adding them to the devices vector
    /// and creating a new player instance. The player will then be added to the main menu.
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

        // Add the player's ID and eliminations to the hashmap
        self.eliminations.insert(self.current_player_id, 0);
    }

    /// Updates the number of elimination for a player as they get a new one.
    ///
    /// Arguments:
    /// * `player_id` - The id of the player that got an elimination.
    pub fn update_eliminations(&mut self, player_id: i32) {
        let eliminations = self.eliminations.entry(player_id).or_insert(0);
        *eliminations += 1;
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
        let index = self
            .devices
            .iter()
            .position(|&r| r == device_id)
            .expect("Device not found");

        self.players.remove(index);
        self.devices.remove(index);
        self.current_player_id = self.devices.len() as i32;
    }

    /// This will return the main menu node.
    ///
    /// Returns:
    /// * Gd<MainMenu> - The main menu node.
    fn get_main_menu(&mut self) -> Gd<MainMenu> {
        if self.main_menu.is_none() {
            self.main_menu = Some(self.base().get_node_as::<MainMenu>("MainMenu"));
        }
        self.main_menu
            .as_ref()
            .expect("MainMenu node not found")
            .clone()
    }

    /// This will return the game mode from the settings.
    ///
    /// Returns:
    /// * String - The name of the game mode.
    pub fn get_game_mode(&mut self) -> String {
        self.settings.bind().get_game_mode()
    }

    /// This will return the team game from the settings.
    ///
    /// Returns:
    /// * bool - If the game is a team game or not.
    pub fn get_team_game(&mut self) -> bool {
        self.settings.bind().get_team_game()
    }
    
    /// This will attempt to start the game.
    /// It will check if the appropriate conditions are met to start the game.
    ///
    /// Arguments:
    /// * `test_mode` - A boolean that determines if the game should only launch with exactly 1
    ///                 player.
    ///
    /// Note: If test mode is true the game will only start if there is exactly 1 player.
    /// Otherwise the game will start only if there are at least 2 players.
    #[func]
    pub fn start_game(&mut self) {
        self.start_round();
        return;
    }

    /// This will set the game mode.
    ///
    /// Arguments:
    /// * `mode` - The name of the game mode to set.
    #[func]
    fn set_game_mode(&mut self, mode: String) {
        self.settings.bind_mut().set_game_mode(mode);
    }

    /// This will set the game mode to team game or not.
    ///
    /// Arguments:
    /// * `team_game` - A boolean that determines if the game is a team game or not.
    #[func]
    fn set_team_game(&mut self, team_game: bool) {
        self.settings.bind_mut().set_team_game(team_game);
    }

    /// This will set the game map.
    ///
    /// Arguments:
    /// * `map` - The name of the map to set.
    #[func]
    fn set_game_map(&mut self, map: String) {
        self.settings.bind_mut().set_map(map);
    }

    /// This will start a round of the game.
    /// Note: This will remove the main menu and instantiate the map.
    #[func]
    pub fn start_round(&mut self) {
        // First remove the main menu
        let main_menu = self.get_main_menu();
        if main_menu.is_inside_tree() {
            self.base_mut().remove_child(&main_menu);
        }

        // Next instantiate the map
        let map = self
            .maps
            .get(self.settings.bind().get_selected_map().as_str())
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
        let zoom: Vector2 = self.determine_screen_size();

        for player in self.players.iter_mut() {
            player.bind_mut().set_zoom(zoom);
        }

        split_screen_one.bind_mut().add_players(odd_players);
        split_screen_two.bind_mut().add_players(even_players);

        // Set the position of the players to the spawn point
        for (index, player) in players.iter_mut().enumerate() {
            let player_id = index as i32 + 1;
            player.set_position(self.get_map().bind().get_spawn_point(player_id.to_string()));
        }

        self.started = true;
        self.day_night_cycle();
    }

    /// This will determine the screen size based on the number of players.
    ///
    /// Returns:
    /// * Vector2 - The size of the screen.
    pub fn determine_screen_size(&mut self) -> Vector2 {
        let screen_size: Vector2;
        if self.players.len() == 1 {
            screen_size = Vector2::new(self.screen_size.x, self.screen_size.y);
        } else if self.players.len() == 2 {
            screen_size = Vector2::new(self.screen_size.x, self.screen_size.y / 2.0);
            self.split_screen_two.set_size(screen_size);
        } else {
            screen_size = Vector2::new(self.screen_size.x / 2.0, self.screen_size.y / 2.0);
            self.split_screen_two.set_size(screen_size);
        }

        self.split_screen_one.set_size(screen_size);
        self.calculate_zoom(screen_size)
    }

    /// This will end the game, end the day/night cycle, and show the winner screen.
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

    /// This will be called when a device is connected or disconnected.
    ///
    /// Arguments:
    /// * `device_id` - The id of the device that was connected or disconnected.
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
                player.bind_mut().set_disconnected(false);
            } else {
                player.bind_mut().set_disconnected(true);
            }
        } else if self.devices.contains(&device_id) && !self.started {
            self.disconnect_player(device_id);
        }
    }

    /// This will return the number of players in the game.
    ///
    /// Returns:
    /// * i32 - The number of players in the game.
    pub fn get_number_of_players(&self) -> i32 {
        self.players.len() as i32
    }

    /// This will set the map for the game.
    ///
    /// Arguments:
    /// * `map` - The map to set.
    pub fn set_map(&mut self, map: Gd<Map>) {
        self.map = Some(map);
    }

    /// This will return the current map.
    ///
    /// Returns:
    /// * Gd<Map> - The current map.
    pub fn get_map(&self) -> Gd<Map> {
        self.map.as_ref().expect("Map not found").clone()
    }

    /// This will reset the players by clearing the players vector and re-adding them.
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

    /// This will disconnect a player from the game.
    /// If there is only one player left in the game they will be declared the winner.
    ///
    /// Arguments:
    /// * `player_id` - The id of the player to disconnect.
    /// * `instance_elims` - The number of eliminations the player got in this instance.
    pub fn remove_player(&mut self, player_id: i32, instance_elims: i32) {
        // before removing the player, update the eliminations for the player
        // associated with the player_id in the hashmap

        // get the number of eliminations for the player in the hashmap
        let eliminations = self.eliminations.get(&player_id).unwrap();

        // update the eliminations in the hashmap with the eliminations in this instance + the
        // eliminations in the hashmap
        self.eliminations
            .insert(player_id, eliminations + instance_elims);

        self.players.remove(player_id as usize - 1);

        let player_length = self.players.len();

        // if there is only one player left in the game, they are the winner of that round
        if player_length == 1 {
            let last_player_id = self.players[0].bind().get_player_id();
            let wins = self.round_wins.entry(last_player_id).or_insert(0);
            *wins += 1;
        }

        if player_length <= 1 {
            if !self.check_win_condition() {
                self.start_new_round();
            } else {
                self.end_game();
            }
        }
    }

    /// This will check if a player has reached the required elimination count.
    ///
    /// Returns:
    /// * bool - If a player has reached the required elimination count.
    fn check_win_condition(&mut self) -> bool {
        if self.devices.len() == 1 {
            return true;
        }

        if self.get_game_mode() == "Head Hunters" {
            // check if a player has reached the required number of eliminations by checking the hashmap
            for (_, eliminations) in self.eliminations.iter() {
                if *eliminations >= REQUIRED_ELIMINATIONS {
                    // set the winning player to the player with the required number of eliminations
                    self.winning_player = self
                        .eliminations
                        .iter()
                        .position(|(&k, &v)| v == REQUIRED_ELIMINATIONS)
                        .unwrap() as i32
                        + 1;
                    return true;
                }
            }
        } else if self.get_game_mode() == "Last Player Standing" {
            // check if a player has reached the required number of round wins
            for (_, round_wins) in self.round_wins.iter() {
                if *round_wins >= REQUIRED_ROUNDS {
                    // set the winning player to the player with the required number of round wins
                    self.winning_player = self
                        .round_wins
                        .iter()
                        .position(|(&k, &v)| v == REQUIRED_ROUNDS)
                        .unwrap() as i32
                        + 1;
                    return true;
                }
            }
        }
        false
    }

    /// This will start a new round. It will reset the players and start the game again.
    fn start_new_round(&mut self) {
        // Reset split screens to size 0,0
        self.split_screen_one.bind_mut().reset();
        self.split_screen_two.bind_mut().reset();

        self.reset_players();

        // Display the winner message
        if let Some(winner_label) = &mut self.winner_label {
            let winner_text = format!("Player {} wins!", self.winning_player);
            winner_label.set_text(&winner_text);
            winner_label.set_visible(true);
        }

        self.round_transition_timer.start();
    }

    #[func]
    /// This starts the round transition and removes the winner message.
    fn round_transition(&mut self) {
        // Hide the winner label
        if let Some(winner_label) = &mut self.winner_label {
            winner_label.set_visible(false);
        }

        // Start the new round
        self.should_start_new_round = true;
    }

    /// This will start the day/night cycle.
    fn day_night_cycle(&mut self) {
        let game = self.base().get_node_as::<Game>(".");
        self.day_night_timer.connect(
            "timeout",
            &Callable::from_object_method(&game, "cycle_change"),
        );

        let day_night_timer = self.day_night_timer.clone();
        self.base_mut().add_child(&day_night_timer);
    }

    /// This will change the day/night cycle.
    #[func]
    pub fn cycle_change(&mut self) {
        const TRANSITION_TIME: f64 = 3.0;
        let brightness: f32 = if self.day { 0.0 } else { 1.0 };
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
    pub fn change_cycle_map(light_level: f32, transition_time: f64, scale: f32);

    #[signal]
    pub fn change_cycle_player(light_level: f32, transition_time: f64);
}
