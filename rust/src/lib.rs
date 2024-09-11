use godot::prelude::*;

mod player {

    pub mod player;

    mod enums {}

    mod traits {
        pub mod player_state;
    }

    mod player_states {
        pub mod idle;
        pub mod jump;
    }
}

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
