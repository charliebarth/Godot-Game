use godot::prelude::*;

mod player {

    pub mod player;

    mod enums {
        pub mod player_state;
    }
}

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
