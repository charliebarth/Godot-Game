use godot::prelude::*;

pub mod metal_reserve_bar_manager;

pub mod metal_bar;



struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
