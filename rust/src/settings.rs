use godot::{classes::Object, prelude::*};

#[derive(GodotClass)]
#[class(base=Object)]
pub struct Settings {
    base: Base<Object>,
    maps: Vec<String>,
    selected_map: String,
    day_night_length: f32,
    day_night_transition_time: f32,
    player_max_health: f32,
    player_max_run_speed: f32,
    player_min_run_speed: f32,
    player_max_jump_force: f32,
    player_min_jump_force: f32,
    gravity: f32,
    /// Increasing this makes the player land faster and feel heavier/more responsive to control
    /// Decreasing this makes the player spend more time falling and feel lighter/less responsive to control
    player_fall_gravity: f32,
}

#[godot_api]
impl IObject for Settings {
    fn init(base: Base<Object>) -> Self {
        Self {
            base,
            maps: Vec::new(),
            selected_map: "MapTwo".to_string(),
            day_night_length: 10.0,
            day_night_transition_time: 1.0,
            player_max_health: 100.0,
            player_max_run_speed: 200.0,
            player_min_run_speed: 100.0,
            player_max_jump_force: 400.0,
            player_min_jump_force: 200.0,
            gravity: 9.81,
            player_fall_gravity: 1000.0,
        }
    }
}

#[godot_api]
impl Settings {
    #[func]
    pub fn foo(&self) {
        godot_print!("Settings singleton fetched");
    }

    pub fn get_maps(&self) -> Vec<String> {
        self.maps.clone()
    }

    pub fn get_selected_map(&self) -> String {
        self.selected_map.clone()
    }

    pub fn get_gravity(&self) -> f32 {
        self.gravity
    }
}
