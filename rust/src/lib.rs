use godot::prelude::*;

mod player {

    pub mod player;
    mod traits {
        pub mod player_state;
    }

    mod states {
        pub mod jump;
    }
}

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
