//! settings.rs
//!
//! This file contains the implementation of the Settings class, which is responsible for
//! managing the game settings. It includes player settings, environment settings, map settings,
//! general settings, and game mode settings.
//!
//! Author: Trinity Pittman, Charles Barth
//! Version: Spring 2025
use godot::{classes::Object, prelude::*};

/// This is a struct for all the adjustable settings in the game
#[derive(GodotClass)]
#[class(base=Object)]
pub struct Settings {
    /// The base node of the Settings class.
    base: Base<Object>,
    /// The player settings.
    pub player: PlayerSettings,
    /// The environment settings.
    pub environment: EnvironmentSettings,
    /// The map settings.
    pub map: MapSettings,
    /// The general settings.
    pub general: GeneralSettings,
    /// The game mode
    pub game_mode: GameMode,
}

/// IObject methods for Settings
#[godot_api]
impl IObject for Settings {
    /// Constructer for the Settings class.
    /// # Arguments
    /// * `base` - The base node of the Settings.
    ///
    /// # Returns
    /// * A settings object.
    fn init(base: Base<Object>) -> Self {
        Self {
            base,
            player: PlayerSettings::default(),
            environment: EnvironmentSettings::default(),
            map: MapSettings::default(),
            general: GeneralSettings::default(),
            game_mode: GameMode::default(),
        }
    }
}

/// Methods for the Settings
#[godot_api]
impl Settings {
    /// Gets the player's max health.
    /// # Returns
    /// * (f32) - The player's max health.
    #[func]
    pub fn get_player_max_health(&self) -> f32 {
        self.player.max_health
    }

    /// Gets the selected map.
    /// # Returns
    /// * (String) - The selected map.
    #[func]
    pub fn get_selected_map(&self) -> String {
        self.map.selected_map.clone()
    }

    /// Sets the selected map.
    /// # Arguments
    /// * (String) - The selected map.
    #[func]
    pub fn set_map(&mut self, map: String) {
        self.map.selected_map = map;
    }

    /// Gets the gravity.
    /// # Returns
    /// * (f32) - The gravity.
    #[func]
    pub fn get_gravity(&self) -> f32 {
        self.environment.gravity
    }

    /// Gets the debug mode.
    /// # Returns
    /// * (bool) - Whether the debug mode is enabled.
    #[func]
    pub fn is_debug_mode(&self) -> bool {
        self.general.debug_mode
    }

    /// Gets the game mode.
    /// # Returns
    /// * A string representing the game mode, defaults to "Last Player Standing".
    #[func]
    pub fn get_game_mode(&self) -> String {
        self.game_mode.game_mode.clone()
    }

    /// Sets the game mode.
    /// # Arguments
    /// * `mode` - The new game mode.
    #[func]
    pub fn set_game_mode(&mut self, mode: String) {
        self.game_mode.game_mode = mode;
    }

    /// Gets whether this game is a team game or not.
    /// # Returns
    /// * true if team game, false if solo game.
    #[func]
    pub fn get_team_game(&self) -> bool {
        self.game_mode.team_game
    }

    /// Sets whether this is a team game or not.
    /// # Arguments
    /// * `team` - true if team game, false if solo game.
    #[func]
    pub fn set_team_game(&mut self, team: bool) {
        self.game_mode.team_game = team;
    }

    /// Gets whether this is an online multiplayer game or not.
    /// # Returns
    /// * true if online multiplayer, false if not.
    #[func]
    pub fn get_online_multiplayer(&self) -> bool {
        self.general.online_multiplayer
    }

    /// Sets whether this is an online multiplayer game or not.
    /// # Arguments
    /// * `online` - true if online multiplayer, false if not.
    #[func]
    pub fn set_online_multiplayer(&mut self, online: bool) {
        self.general.online_multiplayer = online;
    }
}

#[derive(Clone)]
/// This is a struct for the player settings.
pub struct PlayerSettings {
    /// The player's max health.
    pub max_health: f32,
    /// The player's max run speed.
    pub max_run_speed: f32,
    /// The player's min run speed.
    pub min_run_speed: f32,
    /// The player's max jump force.
    pub max_jump_force: f32,
    /// The player's min jump force.
    pub min_jump_force: f32,
    /// The player's fall gravity.
    pub fall_gravity: f32,
}

/// Default methods for the PlayerSettings
impl Default for PlayerSettings {
    /// Default method for the Player Settings
    /// # Returns
    /// * PlayerSettings with default values for each
    fn default() -> Self {
        Self {
            max_health: 100.0,
            max_run_speed: 200.0,
            min_run_speed: 100.0,
            max_jump_force: 400.0,
            min_jump_force: 200.0,
            fall_gravity: 1000.0,
        }
    }
}

#[derive(Clone)]
/// This is a struct for the environment settings.
pub struct EnvironmentSettings {
    /// The length of the day and night cycle.
    pub day_night_length: f32,
    /// The time it takes to transition from day to night.
    pub day_night_transition_time: f32,
    /// The gravity of the environment.
    pub gravity: f32,
    /// The static friction of the environment.
    pub static_friction: f32,
    /// The kinetic friction of the environment.
    pub kinetic_friction: f32,
    /// The drag coefficient of the environment.
    pub air_density: f32,
}

/// Default methods for the Environment Settings
impl Default for EnvironmentSettings {
    /// Default method for the Environment Settings
    /// # Returns
    /// * EnvironmentSettings with default values
    fn default() -> Self {
        Self {
            day_night_length: 10.0,
            day_night_transition_time: 1.0,
            gravity: 980.0,
            static_friction: 1.0,
            kinetic_friction: 1.0,
            air_density: 1.0,
        }
    }
}

#[derive(Clone)]
/// This is a struct for the map settings.
pub struct MapSettings {
    /// The maps in the game.
    pub maps: Vec<String>,
    /// The selected map.
    pub selected_map: String,
}

///  Default methods for the Map Settings
impl Default for MapSettings {
    /// Default method for the Map Settings
    /// # Returns
    /// * MapSettings with default values
    fn default() -> Self {
        Self {
            maps: Vec::new(),
            selected_map: "MapOne".to_string(),
        }
    }
}

#[derive(Clone)]
/// This is a struct for the general settings.
pub struct GeneralSettings {
    /// Whether the debug mode is enabled.
    pub debug_mode: bool,
    /// Whether the game is an online multiplayer game.
    pub online_multiplayer: bool,
}

///  Default methods for the General Settings
impl Default for GeneralSettings {
    /// Default method for the General Settings
    /// # Returns
    /// * GeneralSettings with default values
    fn default() -> Self {
        Self {
            debug_mode: true,
            online_multiplayer: false,
        }
    }
}

/// Represents the game mode and whether this game is team or solo based.
#[derive(Clone)]
pub struct GameMode {
    /// What game mode the game will be.
    pub game_mode: String,
    /// Whether the game will be a team or solo game.
    pub team_game: bool,
}

/// Default method for the GameMode struct
impl Default for GameMode {
    /// Gets the default values for the GameMode struct
    ///
    /// # Returns
    /// * (Self) - Default representation of GameMode.
    fn default() -> Self {
        Self {
            game_mode: "Last Player Standing".to_string(),
            team_game: false,
        }
    }
}
