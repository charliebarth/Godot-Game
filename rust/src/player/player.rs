// use godot::builtin::StringName;
// use godot::builtin::Vector2;
// use godot::classes::AnimatedSprite2D;
use godot::classes::CharacterBody2D;
use godot::classes::ICharacterBody2D;
// use godot::classes::Input;
// use godot::classes::ProjectSettings;
// use godot::meta::FromGodot;
use godot::prelude::*;

use super::enums::player_states::PlayerStates;

// const MAX_JUMP_HEIGHT: f32 = 300.0;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    base: Base<CharacterBody2D>,
    cur_state: PlayerStates,
    last_state: PlayerStates,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            base,
            cur_state: PlayerStates::Idle,
            last_state: PlayerStates::Fall,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.cur_state.trigger(&self.last_state)
    }
}
