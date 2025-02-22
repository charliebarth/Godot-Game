/// This file is the connection point between Rust and Godot
/// It is responsible for defining the Rust library and the classes that Godot can use
use godot::{classes::Engine, prelude::*};
use settings::Settings;
pub mod player {

    pub mod disconnected;
    pub mod input_manager;
    pub mod metal_line;
    pub mod metal_manager;
    pub mod player;
    pub mod player_tin_light;

    pub mod enums {
        pub mod coin_events;
        pub mod force;
        pub mod metal_type;
        pub mod player_events;
        pub mod player_states;
        pub mod timeout_events;
    }

    pub mod traits {
        pub mod metal;
        pub mod player_state;
    }

    pub mod player_states {
        pub mod attack;
        pub mod crouch;
        pub mod crouch_end;
        pub mod crouch_start;
        pub mod fall;
        pub mod idle;
        pub mod jump;
        pub mod land;
        pub mod roll;
        pub mod run;
        pub mod slide;
        pub mod slide_crouch;
        pub mod sprint;
    }

    pub mod metals {
        pub mod iron;
        pub mod pewter;
        pub mod steel;
        pub mod tin;
    }
}

pub mod game;
pub mod main_menu;
pub mod map;
pub mod map_light;
pub mod metal_object;
pub mod metal_pickups;
pub mod player_light;
pub mod settings;
pub mod split_screen;

pub mod items {
    pub mod coin;
    pub mod metal_vial;
}

pub mod ui {
    pub mod coin_counter;
    pub mod metal_bar;
    pub mod metal_reserve_bar_manager;
}

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            // The `&str` identifies your singleton and can be
            // used later to access it.
            Engine::singleton().register_singleton("Settings", &Settings::new_alloc());
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            // Let's keep a variable of our Engine singleton instance,
            // and MyEngineSingleton name.
            let mut engine = Engine::singleton();
            let singleton_name = "Settings";

            // Here, we manually retrieve our singleton(s) that we've registered,
            // so we can unregister them and free them from memory - unregistering
            // singletons isn't handled automatically by the library.
            if let Some(my_singleton) = engine.get_singleton(singleton_name) {
                // Unregistering from Godot, and freeing from memory is required
                // to avoid memory leaks, warnings, and hot reloading problems.
                engine.unregister_singleton(singleton_name);
                my_singleton.free();
            } else {
                // You can either recover, or panic from here.
                godot_error!("Failed to get settings singleton");
            }
        }
    }
}
