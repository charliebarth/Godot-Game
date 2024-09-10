use godot::prelude::*;

mod player {

    pub mod player;
    mod traits {
        pub mod player_state;
    }

    mod enums {
        pub mod player_states;
    }
}

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
