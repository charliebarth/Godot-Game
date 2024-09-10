use godot::prelude::*;

mod player {

    pub mod player;
    mod traits {
        pub mod state;
    }

    mod enums {
        pub mod player_states;
    }

    mod states {
        pub mod jump;
    }
}

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
