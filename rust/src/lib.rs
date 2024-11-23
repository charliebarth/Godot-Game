use godot::prelude::*;

mod player {

    pub mod input_manager;
    pub mod metal_line;
    pub mod metal_manager;
    pub mod player;

    mod enums {
        pub mod force;
        pub mod metal_events;
        pub mod player_events;
        pub mod player_states;
        pub mod timeout_events;
    }

    mod traits {
        pub mod metal;
        pub mod player_state;
    }

    mod player_states {
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
        pub mod attack;
    }

    mod metals {
        pub mod pewter;
        pub mod steel;
    }
}

pub mod metal_object;
pub mod player_manager;

mod items {
    pub mod coin;
    pub mod metal_vial;
}

mod ui {
    pub mod allomantic_line;
    pub mod allomantic_range;
    pub mod coin_counter;
    pub mod metal_bar;
    pub mod metal_reserve_bar_manager;
}

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
