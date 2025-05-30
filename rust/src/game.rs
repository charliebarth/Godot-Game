//! game.rs
//!
//! This file contains the Game class which is responsible for managing the game state such as
//! players, maps, game modes, and the main menu. It also handles the day/night cycle and the
//! split screen for multiplayer gameplay.
//!
//! Author: Charles Barth, Michael Imerman, Trinity Pittman
//! Version: Spring 2025
use crate::{
    main_menu::MainMenu, map::Map, player::player::Player, settings::Settings,
    split_screen::SplitScreen,
};

use godot::classes::{Input, Label};
use godot::global::HorizontalAlignment;
use godot::{
    classes::{
        AnimatedSprite2D, DisplayServer, Engine, InputEvent, InputMap, ResourceLoader,
        ShaderMaterial, Timer,
    },
    prelude::*,
};
use std::collections::HashMap;

// The game mode is stored in a static variable so it can be accessed from anywhere in the code
static mut GAME_MODE: Option<String> = None;
// The number of eliminations required to win the game;
// could/should be changed to be more dynamic in the future
const REQUIRED_ELIMINATIONS: i32 = 5;
// The number of rounds required to win the game
const REQUIRED_ROUNDS: i32 = 1;

/// The Game class is responsible for managing the game state such as players,
/// maps, and the main menu.
/// This is also the root node of the scene tree.
#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Game {
    /// The base node of the Game.
    base: Base<Node2D>,
    /// A map of input device IDs to players.
    players: Vec<Gd<Player>>,
    /// A map of Player team names to player ids
    team_tracker: HashMap<String, Vec<i32>>,
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
    /// The ID of the winning player as a string.
    winner: String,
    /// A collection of maps/levels that can be loaded.
    maps: HashMap<String, Gd<PackedScene>>,
    /// A reference to the main menu.
    main_menu: Option<Gd<MainMenu>>,
    /// The tutorial scene.
    tutorial_scene: Gd<PackedScene>,
    /// The tutorial level.
    tutorial_level: Option<Gd<Map>>,
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
    /// The constructor for the Game class.
    /// # Arguments
    /// * `base` - The base node of the Game.
    /// # Returns
    /// * A Game object.
    fn init(base: Base<Node2D>) -> Self {
        const CYCLE_LENGTH: f64 = 10.0;
        let mut day_night_timer = Timer::new_alloc();
        day_night_timer.set_wait_time(CYCLE_LENGTH);
        //day_night_timer.set_autostart(true);

        /// The time it takes to transition between rounds
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
            team_tracker: HashMap::new(),
            devices: Vec::new(),
            register_button: "jump".into(),
            player_scene: load::<PackedScene>("res://scenes/player.tscn"),
            current_player_id: 0,
            started: false,
            map: None,
            winner: "0".to_string(),
            maps: HashMap::new(),
            main_menu: None,
            tutorial_scene: load::<PackedScene>("res://scenes/tutorial.tscn"),
            tutorial_level: None,
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

    /// This is a builtin method for Godot that is called when the node is added
    /// to the scene tree.
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
            &Callable::from_object_method(&target, "round_transition"),
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
    /// When the button is pressed if it is the first time that device has
    /// pressed the button the device id will be saved and a player will be
    /// created and assigned that device id.
    ///
    /// Note: This does not spawn players or start the game simply tracks
    /// players and their device ids.
    /// Players can be spawned with the start method.
    ///
    /// # Arguments:
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
    /// # Arguments:
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
    /// This is the size of one viewport in a 4-player configuration on a
    /// 1920x1080 screen
    const REFERENCE_VIEWPORT: Vector2 = Vector2::new(960.0, 540.0);

    /// Calculates the appropriate zoom factor based on viewport size.
    /// Uses the minimum zoom value to maintain aspect ratio.
    ///
    /// # Arguments:
    /// * `viewport_size` - The size of the viewport to calculate the zoom for.
    ///
    /// # Returns
    /// * Vector2 - The zoom factor for the viewport.
    fn calculate_zoom(&self, viewport_size: Vector2) -> Vector2 {
        let zoom_x = viewport_size.x / Self::REFERENCE_VIEWPORT.x;
        let zoom_y = viewport_size.y / Self::REFERENCE_VIEWPORT.y;
        let zoom = zoom_x.min(zoom_y);
        Vector2::new(zoom, zoom)
    }

    /// This will register a player to the game by adding them to the devices
    /// vector and creating a new player instance. The player will then be added
    /// to the main menu.
    ///
    /// # Arguments:
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
    /// # Arguments:
    /// * `player_id` - The id of the player that got an elimination.
    pub fn update_eliminations(&mut self, player_id: i32) {
        let eliminations = self.eliminations.entry(player_id).or_insert(0);
        *eliminations += 1;
    }

    /// This will disconnect a player from the game.
    /// Disconnecting a player will remove them from the game and shift all
    /// still connected players up.
    /// For example if player 2 is disconnected player 3 will become player 2
    /// and player 4 will become player 3. The device associated with the
    /// disconnected player will be removed from the list of connected devices
    /// as well.
    ///
    /// # Arguments:
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
    /// # Returns
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

    /// Gets the hashmap that stores which team each player is on, if the
    /// hashmap is empty, initializes it with the teams Red and Blue
    ///
    /// # Returns
    /// * a hashmap of strings (team names) mapped to vectors of i32 (player ids)
    fn get_team_tracker(&mut self) -> &mut HashMap<String, Vec<i32>> {
        if self.team_tracker.is_empty() {
            self.team_tracker.insert("Red".to_string(), vec![]);
            self.team_tracker.insert("Blue".to_string(), vec![]);
        }

        &mut self.team_tracker
    }

    /// Resets the team players hashmap and clears each players outline
    /// This will reset the team players by clearing the team tracker and
    /// removing the team colors from the players.
    #[func]
    fn reset_team_players(&mut self) {
        self.get_team_tracker().clear();

        // Remove the outline for each player
        for i in 0..self.players.len() {
            let player = self.players[i].clone();
            let mut player_an = player.get_node_as::<AnimatedSprite2D>("PlayerAnimation");
            player_an.set_use_parent_material(true);
            self.get_main_menu()
                .bind_mut()
                .set_player_team(i as i32, "clear".to_string());
        }
    }

    /// Given the players Id and chosen team, adds a player to the correct
    /// hashmap and sets the players outline to the correct color of their team.
    ///
    /// # Arguments
    /// * `id` - The id of the player to set the team for
    /// * `team` - The team to set the player to
    #[func]
    fn set_player_team(&mut self, id: i32, team: String) {
        self.get_main_menu()
            .bind_mut()
            .set_player_team(id, team.clone());
        let position = self.devices.iter().position(|&el| el == id);
        if position.is_none() {
            return;
        }

        let id = position.expect("id missing") as i32;
        let add: &str = &team; // The team to add the player to
        let rem: &str; // The team to remove the player from

        if team == "Blue" {
            rem = "Red";
        } else {
            rem = "Blue";
        }

        // Add the player to the correct team if they aren't already on it
        let add_to = self
            .get_team_tracker()
            .get_mut(add)
            .expect("Failed to get Key");
        if !add_to.contains(&id) {
            add_to.push(id)
        }

        // Removes the player from the other team if they were on it
        let remove_from = self
            .get_team_tracker()
            .get_mut(rem)
            .expect("Failed to get Key");
        if let Some(i) = remove_from.iter().position(|&el| el == id) {
            remove_from.remove(i);
        }
    }

    /// Given the players Id and chosen team, adds a player to the correct
    /// hashmap and sets the players outline to the correct color of their team.
    ///
    /// # Arguments
    /// * `id` - The id of the player to set the team for
    /// * `team` - The team to set the player to
    fn set_player_team_outline(&mut self, id: i32, team: String) {
        let path: &str;

        if team == "Blue" {
            path = "res://shaders/blue_outline.tres";
        } else {
            path = "res://shaders/red_outline.tres";
        }

        // Sets the players outline to the team color
        let shader = ResourceLoader::singleton().load(path);
        if let Ok(shader) = shader.unwrap().try_cast::<ShaderMaterial>() {
            let player = self.players[id as usize].clone();
            let mut player_an = player.get_node_as::<AnimatedSprite2D>("PlayerAnimation");
            player_an.set_use_parent_material(false);
            player_an.set_material(&shader);
        }
    }

    /// Gets the selected game mode from the settings
    ///
    /// # Returns
    /// * A string representing the game mode (see options below)
    ///     - "Last Player Standing"
    ///     - "Head Hunters"
    pub fn get_game_mode(&mut self) -> String {
        self.settings.bind().get_game_mode()
    }

    /// Gets whether the game is a team game or not from the settings
    ///
    /// # Returns
    /// * true if team game false if solo
    #[func]
    pub fn get_team_game(&mut self) -> bool {
        self.settings.bind().get_team_game()
    }

    /// Sets the game mode for this game in the settings
    ///
    /// # Arguments
    /// * `mode` - The new game mode
    #[func]
    fn set_game_mode(&mut self, mode: String) {
        self.settings.bind_mut().set_game_mode(mode);
    }

    /// Sets whether this game is a team game or not in the settings
    ///
    /// # Arguments
    /// * `team_game` - true if team game false if solo
    #[func]
    fn set_team_game(&mut self, team_game: bool) {
        self.settings.bind_mut().set_team_game(team_game);
    }

    /// Sets the map for this game in the settings
    ///
    /// # Arguments
    /// * `map` - a string representing the map to play on, should be entered
    ///           based on the name of the map node in godot ex. `MapOne`
    #[func]
    fn set_game_map(&mut self, map: String) {
        self.settings.bind_mut().set_map(map);
    }

    /// This will attempt to start the game.
    /// It will check if the appropriate conditions are met to start the game.
    ///
    /// # Conditions
    /// * there must be at least one player
    /// * in team games each player must be on a team
    /// * in team games each team must have at least one player
    #[func]
    pub fn start_game(&mut self) {
        if self.get_team_game() {
            let red_num = self
                .get_team_tracker()
                .get("Red")
                .expect("Couldn't get value")
                .len();
            let blue_num = self
                .get_team_tracker()
                .get("Blue")
                .expect("Couldn't get value")
                .len();
            let team_players = red_num + blue_num;

            if team_players != self.players.len() {
                self.get_main_menu()
                    .bind()
                    .add_notification("All players must choose a team".to_string());
                return;
            } else if blue_num == 0 || red_num == 0 {
                self.get_main_menu()
                    .bind()
                    .add_notification("Each team must have at least one player".to_string());
                return;
            }
        } else if self.players.len() <= 0 {
            self.get_main_menu()
                .bind()
                .add_notification("More players needed to start the game".to_string());
            return;
        }

        self.start_round();

        return;
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

        // If its a team game, set the players outline colors
        if self.get_team_game() {
            let red_team = self
                .get_team_tracker()
                .get("Red")
                .expect("Couldn't get value")
                .clone();
            for id in red_team {
                self.set_player_team_outline(id, "Red".to_string());
            }
            let blue_team = self
                .get_team_tracker()
                .get("Blue")
                .expect("Couldn't get value")
                .clone();
            for id in blue_team {
                self.set_player_team_outline(id, "Blue".to_string());
            }
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
        //self.day_night_cycle();
    }

    /// This will determine the screen size based on the number of players.
    ///
    /// # Returns
    /// * Vector2 - The size of the screen.
    pub fn determine_screen_size(&mut self) -> Vector2 {
        let screen_size: Vector2;
        if self.players.len() == 1 {
            screen_size = Vector2::new(self.screen_size.x, self.screen_size.y);
        } else if self.players.len() == 2 {
            screen_size = Vector2::new(self.screen_size.x, self.screen_size.y / 2.0);
            self.split_screen_two.set_size(screen_size);
        } else {
            screen_size = Vector2::new(self.screen_size.x, self.screen_size.y / 2.0);
            self.split_screen_two.set_size(screen_size);
        }

        self.split_screen_one.set_size(screen_size);
        self.calculate_zoom(screen_size)
    }

    /// This will end the game, end the day/night cycle, and show the winner
    /// screen.
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
        main_menu.bind_mut().swap_to_main_menu();
        self.base_mut().add_child(&main_menu);

        // Show who won!
        if self.get_team_game() {
            main_menu
                .bind_mut()
                .add_notification(format!("Team {} wins!", self.winner));
        } else {
            main_menu
                .bind_mut()
                .add_notification(format!("Player {} wins!", self.winner));
        }
    }

    /// This will be called when a device is connected or disconnected.
    ///
    /// # Arguments:
    /// * `device_id` - The id of the device that was connected or disconnected.
    /// * `connected` - A boolean that determines if the device is connected or
    ///                 disconnected.
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

    /// Gets the number of players currently in this game as an i32
    /// # Returns
    /// * The number of players currently connected
    #[func]
    pub fn get_number_of_players(&self) -> i32 {
        self.players.len() as i32
    }

    /// This will set the map for the game.
    ///
    /// # Arguments:
    /// * `map` - The map to set.
    pub fn set_map(&mut self, map: Gd<Map>) {
        self.map = Some(map);
    }

    /// This will return the current map.
    ///
    /// # Returns
    /// * Gd<Map> - The current map.
    pub fn get_map(&self) -> Gd<Map> {
        self.map.as_ref().expect("Map not found").clone()
    }

    /// This will reset the players by clearing the players vector and re-adding
    /// them.
    fn reset_players(&mut self) {
        self.players.clear();
        self.current_player_id = 0;
        for device_id in self.devices.iter() {
            self.current_player_id += 1;
            let mut player = self.player_scene.instantiate_as::<Player>();
            player.bind_mut().set_device_id(device_id.clone());
            player.bind_mut().set_player_id(self.current_player_id);
            player.set_name(format!("Player{}", self.current_player_id).as_str());
            // Add player team adding here
            self.players.push(player);
        }
    }

    #[func]
    /// This will disconnect a player from the game.
    /// If there is only one player left in the game they will be declared the
    /// winner.
    ///
    /// # Arguments:
    /// * `player_id` - The id of the player to disconnect.
    /// * `instance_elims` - The number of eliminations the player got in this
    ///                      instance.
    pub fn remove_player(&mut self, player_id: i32, instance_elims: i32) {
        // before removing the player, update the eliminations for the player
        // associated with the player_id in the hashmap

        // get the number of eliminations for the player in the hashmap
        let eliminations = self.eliminations.get(&player_id).unwrap();

        // update the eliminations in the hashmap with the eliminations in this
        // instance + the eliminations in the hashmap
        self.eliminations
            .insert(player_id, eliminations + instance_elims);

        let mut index: usize = 999;
        for i in 0..self.players.len() {
            if self.players[i].bind().get_player_id() == player_id {
                index = i;
            }
        }

        if index != 999 {
            let mut player = self.players.remove(index);
            player.queue_free();
        }

        let player_length = self.players.len();

        // if there is only one player left in the game, they are the winner of
        // that round
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

    /// Adds up the number of wins/eliminations for a team based on the hashmap
    /// passed in.
    ///
    /// # Arguments
    /// * `team_color` - The color of the team to calculate.
    /// * `wins_check` - The hashmap the number of wins for each player is
    ///                  stored in.
    /// # Returns
    /// * (i32) - The number of wins/eliminations.
    fn get_team_eliminations(&mut self, team_color: &str, wins_check: &HashMap<i32, i32>) -> i32 {
        let team_tracker = self.get_team_tracker().clone();
        let team = team_tracker.get(team_color).expect("Couldn't get value");

        godot_print!("Hashmap of wins {:?}", wins_check);
        godot_print!("Hashmap of teams {:?}", team);
        // let elims = &self.eliminations;
        let mut team_elims: i32 = 0;
        for id in team.clone() {
            if wins_check.contains_key(&(id + 1)) {
                team_elims += wins_check.get(&(id + 1)).unwrap();
            }
        }

        team_elims
    }

    /// This will check if a player has reached the required elimination count.
    ///
    /// # Returns
    /// * (bool) - If a player has reached the required elimination count.
    fn check_win_condition(&mut self) -> bool {
        if self.devices.len() == 1 {
            return true;
        }

        let end_condition: i32;
        let wins_check: HashMap<i32, i32>;

        if self.get_game_mode() == "Head Hunters" {
            end_condition = REQUIRED_ELIMINATIONS;
            wins_check = self.eliminations.clone();
        } else {
            // Last Player Standing
            end_condition = REQUIRED_ROUNDS;
            wins_check = self.round_wins.clone();
        }

        if self.get_team_game() {
            // Get the number of elminations/rounds won for each team
            let red = self.get_team_eliminations("Red", &wins_check);
            let blue = self.get_team_eliminations("Blue", &wins_check);

            // Check if either team has reached the end condition
            if red == end_condition {
                self.winner = "Red".to_string();
                return true;
            } else if blue == end_condition {
                self.winner = "Blue".to_string();
                return true;
            }
        } else {
            // check if a player has reached the required number of eliminations
            // by checking the hashmap
            for (_, eliminations) in wins_check.iter() {
                if *eliminations >= end_condition {
                    // set the winning player to the player with the required
                    // number of eliminations
                    self.winner = (wins_check
                        .iter()
                        .position(|(&k, &v)| v == end_condition)
                        .unwrap() as i32
                        + 1)
                    .to_string();
                    return true;
                }
            }
        }
        // if no player has reached the required number of eliminations/rounds, return false
        false
    }

    /// This will start a new round. It will reset the players and start the
    /// game again.
    fn start_new_round(&mut self) {
        // Reset split screens to size 0,0
        self.split_screen_one.bind_mut().reset();
        self.split_screen_two.bind_mut().reset();

        self.reset_players();

        // Display the winner message
        if let Some(winner_label) = &mut self.winner_label {
            let winner_text = format!("Player {} wins!", self.winner);
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

        let brightness_map = if self.day { 0.4 } else { 1.0 };
        let scale_map = if self.day { 0.6 } else { 1.0 };
        self.base_mut().emit_signal(
            "change_cycle_map",
            &[
                Variant::from(brightness_map),
                Variant::from(TRANSITION_TIME),
                Variant::from(scale_map),
            ],
        );

        self.day = !self.day;
    }

    /// This will change the day/night cycle for the map
    ///
    /// # Arguments
    /// * `light_level` - The light level to set the map to
    /// * `transition_time` - The time it takes to transition to the new light level
    /// * `scale` - The scale to set the map to
    #[signal]
    pub fn change_cycle_map(light_level: f32, transition_time: f64, scale: f32);

    /// This will change the day/night cycle for the players
    ///
    /// # Arguments
    /// * `light_level` - The light level to set the players to
    /// * `transition_time` - The time it takes to transition to the new light level
    #[signal]
    pub fn change_cycle_player(light_level: f32, transition_time: f64);

    /// This will start the tutorial and set the map to the tutorial map.
    #[func]
    pub fn start_tutorial(&mut self) {
        // First remove the main menu
        let main_menu = self.get_main_menu();
        if main_menu.is_inside_tree() {
            self.base_mut().remove_child(&main_menu);
        }

        let tutorial_level = self.tutorial_scene.instantiate_as::<Map>();
        self.set_map(tutorial_level);

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
    }

    /// This will end the tutorial and return to the main menu.
    #[func]
    pub fn end_tutorial(&mut self) {
        self.started = false;

        self.split_screen_one.bind_mut().reset();
        self.split_screen_two.bind_mut().reset();

        self.reset_players();

        let main_menu = self.get_main_menu();
        self.base_mut().add_child(&main_menu);
    }
}
