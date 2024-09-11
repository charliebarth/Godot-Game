// use godot::builtin::StringName;
// use godot::builtin::Vector2;
// use godot::classes::AnimatedSprite2D;
use godot::classes::CharacterBody2D;
use godot::classes::ICharacterBody2D;
use godot::classes::ProjectSettings;
// use godot::classes::Input;
// use godot::meta::FromGodot;
use godot::prelude::*;

use super::player_states::jump::Jump;
use super::traits::player_state::PlayerState;

// const MAX_JUMP_HEIGHT: f32 = 300.0;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>,
    direction: f32,
    gravity: f64,
    current_state: Box<dyn PlayerState>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        let path = GString::from("physics/2d/default_gravity");
        let gravity: f64 =
            FromGodot::try_from_variant(&ProjectSettings::singleton().get_setting(path)).unwrap();

        Self {
            base,
            current_state: Box::new(Jump),
            direction: 1.0,
            gravity,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let mut base_vel = self.base_mut().get_velocity();

        if !self.base().is_on_floor() {
            base_vel.y += (self.gravity * delta) as f32;
        } else {
            base_vel.y = 0.0;
        }

        self.base_mut().set_velocity(base_vel);

        self.get_current_state().update(self);

        self.base_mut().move_and_slide();
    }
}

impl Player {
    pub fn set_state(&mut self, new_state: Box<dyn PlayerState>) {
        self.current_state = new_state;
        self.get_current_state().enter(self);
    }

    pub fn get_current_state(&self) -> Box<dyn PlayerState> {
        self.current_state.clone()
    }
}
