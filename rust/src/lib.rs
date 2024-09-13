// use godot::builtin::StringName;
// use godot::builtin::Vector2;
// use godot::classes::AnimatedSprite2D;
use godot::classes::CharacterBody2D;
use godot::classes::ICharacterBody2D;
// use godot::classes::Input;
// use godot::classes::ProjectSettings;
// use godot::meta::FromGodot;
use godot::prelude::*;

// const MAX_JUMP_HEIGHT: f32 = 300.0;

pub mod metalReserveBarManager;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self { base }
    }
}
