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
}

#[godot_api]
impl IObject for Settings {
    fn init(base: Base<Object>) -> Self {
        Self {
            base,
            player: PlayerSettings::default(),
            environment: EnvironmentSettings::default(),
            map: MapSettings::default(),
            general: GeneralSettings::default(),
        }
    }
}

#[godot_api]
impl Settings {
    #[func]
    pub fn get_player_max_health(&self) -> f32 {
        self.player.max_health
    }

    #[func]
    pub fn get_selected_map(&self) -> String {
        self.map.selected_map.clone()
    }

    #[func]
    pub fn get_gravity(&self) -> f32 {
        self.environment.gravity
    }

    #[func]
    pub fn is_debug_mode(&self) -> bool {
        self.general.debug_mode
    }
}

#[derive(Clone)]
pub struct PlayerSettings {
    pub max_health: f32,
    pub max_run_speed: f32,
    pub min_run_speed: f32,
    pub max_jump_force: f32,
    pub min_jump_force: f32,
    pub fall_gravity: f32,
}

impl Default for PlayerSettings {
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
pub struct EnvironmentSettings {
    pub day_night_length: f32,
    pub day_night_transition_time: f32,
    pub gravity: f32,
    pub static_friction: f32,
    pub kinetic_friction: f32,
    pub drag_coefficient: f32,
    pub air_density: f32,
}

impl Default for EnvironmentSettings {
    fn default() -> Self {
        Self {
            day_night_length: 10.0,
            day_night_transition_time: 1.0,
            gravity: 980.0,
            static_friction: 1.0,
            kinetic_friction: 1.0,
            drag_coefficient: 1.0,
            air_density: 1.0,
        }
    }
}

#[derive(Clone)]
pub struct MapSettings {
    pub maps: Vec<String>,
    pub selected_map: String,
}

impl Default for MapSettings {
    fn default() -> Self {
        Self {
            maps: Vec::new(),
            selected_map: "MapTwo".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct GeneralSettings {
    pub debug_mode: bool,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self { debug_mode: true }
    }
}
