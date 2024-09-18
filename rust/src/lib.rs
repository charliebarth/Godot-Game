use godot::prelude::*;

mod player {

    pub mod input_manager;
    pub mod player;

    mod enums {
        pub mod player_events;
    }

    mod traits {
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
        pub mod sprint;
    }
}

pub mod metal_reserve_bar_manager;

pub mod metal_bar;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
