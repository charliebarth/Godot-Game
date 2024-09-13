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
const MAX_HEALTH: u8 = 100;
const MIN_HEALTH: u8 = 0;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>,
    direction: f32,
    gravity: f64,
    health: u8,
    delta: f64,
    current_state: Box<dyn PlayerState>,
    anim_finished: bool,
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
            health: 100,
            delta: 0.0,
            gravity,
            anim_finished: false,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.set_delta(delta);
        self.get_current_state().update(self);
        self.base_mut().move_and_slide();
    }
}

impl Player {
    pub fn set_state(&mut self, new_state: Box<dyn PlayerState>) {
        self.current_state = new_state;
        self.get_current_state().enter(self);
        self.anim_finished = false;
        // TODO: Player animation
    }

    pub fn get_current_state(&self) -> Box<dyn PlayerState> {
        self.current_state.clone()
    }

    fn set_delta(&mut self, delta: f64) {
        self.delta = delta;
    }

    pub fn get_delta(&self) -> f64 {
        self.delta
    }

    pub fn get_health(&self) -> u8 {
        self.health
    }

    pub fn get_dir(&self) -> f32 {
        self.direction
    }

    pub fn set_dir(&mut self, direction: f32) {
        if direction < 0.0 {
            self.direction = -1.0;
        } else if direction > 0.0 {
            self.direction = 1.0;
        }
    }

    pub fn adjust_health(&mut self, health: i8) {
        // Adjust health positively or negatively
        let new_health = if health < 0 {
            // Subtract health, but ensure we handle underflow
            self.health.wrapping_sub(-health as u8) // `-health` converts to positive
        } else {
            // Add health, but ensure no overflow
            self.health.saturating_add(health as u8)
        };

        // Clamp health between MIN_HEALTH and MAX_HEALTH
        self.health = new_health.clamp(MIN_HEALTH, MAX_HEALTH);
    }

    /// Represents the direction the player is trying to move
    /// Returns 1 when the move right button is pressed, -1 when the move left button is pressed, and 0 if neither is pressed
    // TODO: Rename
    pub fn get_horizontal_movement(&mut self) -> f32 {
        let move_left = StringName::from("move_left");
        let move_right = StringName::from("move_right");
        Input::singleton().get_axis(move_left, move_right)
    }

    pub fn apply_horizontal_velocity(&mut self, direction: f32, speed: f32) {
        let mut base = self.base_mut();
        let mut base_vel = base.get_velocity();
        base_vel.x = speed * direction;
        base.set_velocity(base_vel);
    }

    pub fn set_anim_finished(&mut self) {
        self.anim_finished = true;
    }

    pub fn is_anim_finished(&self) -> bool {
        self.anim_finished
    }

    pub fn get_gravity(&self) -> f64 {
        self.gravity
    }
}
