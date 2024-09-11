// use godot::builtin::StringName;
// use godot::builtin::Vector2;
// use godot::classes::AnimatedSprite2D;
use godot::classes::CharacterBody2D;
use godot::classes::ICharacterBody2D;
// use godot::classes::Input;
// use godot::classes::ProjectSettings;
// use godot::meta::FromGodot;
use godot::prelude::*;

use super::enums::player_state::PlayerState;

// const MAX_JUMP_HEIGHT: f32 = 300.0;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>,
    cur_state: PlayerState,
    last_state: PlayerState,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            base,
            cur_state: PlayerState::Idle,
            last_state: PlayerState::Jump,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let cur_state = self.get_current_state();
        let last_state = self.get_last_state();

        if cur_state != last_state {
            cur_state.enter(self)
        } else {
            cur_state.update(self, delta)
        }
    }
}

impl Player {
    fn set_current_state(&mut self, state: PlayerState) {
        self.cur_state = state;
    }

    fn get_current_state(&self) -> PlayerState {
        self.cur_state
    }

    fn set_last_state(&mut self, state: PlayerState) {
        self.last_state = state;
    }

    fn get_last_state(&self) -> PlayerState {
        self.last_state
    }
}
