/// This file is the connection point between Rust and Godot
/// It is responsible for defining the Rust library and the classes that Godot can use
use godot::prelude::*;

pub mod player {

    pub mod disconnected;
    pub mod input_manager;
    pub mod metal_line;
    pub mod metal_manager;
    pub mod player;

    pub mod enums {
        pub mod coin_events;
        pub mod force;
        pub mod metal_events;
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
        pub mod pewter;
        pub mod steel;
    }
}

pub mod game;
pub mod main_menu;
pub mod metal_object;
pub mod metal_pickups;

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
unsafe impl ExtensionLibrary for MyExtension {}
