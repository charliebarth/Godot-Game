use godot::prelude::*;

mod player {

    pub mod player;

    mod enums {}

    mod traits {
        pub mod player_state;
    }

    mod player_states {
        pub mod crouch;
        pub mod fall;
        pub mod idle;
        pub mod jump;
        pub mod jump_fall;
        pub mod land;
        pub mod roll;
        pub mod run;
    }
}

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
