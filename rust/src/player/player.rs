use godot::classes::AnimatedSprite2D;
// use godot::builtin::StringName;
// use godot::builtin::Vector2;
// use godot::classes::AnimatedSprite2D;
use godot::classes::CharacterBody2D;
use godot::classes::ICharacterBody2D;
use godot::classes::ProjectSettings;
// use godot::classes::Input;
// use godot::meta::FromGodot;
use godot::prelude::*;

use super::player_states::idle::Idle;
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
            current_state: Box::new(Idle),
            direction: 1.0,
            health: 100,
            delta: 0.0,
            gravity,
            anim_finished: false,
        }
    }

    fn ready(&mut self) {
        self.set_state(Box::new(Idle));

        let method_name: StringName = "set_anim_finished".into();
        let callable: Callable = self.base_mut().callable(method_name);
        let signal: StringName = "animation_finished".into();

        let mut sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        sprite.connect(signal, callable);
        godot_print!("Connected signal");
    }

    fn physics_process(&mut self, delta: f64) {
        self.set_delta(delta);

        let mut base_vel = self.base_mut().get_velocity();

        let sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        if !sprite.is_playing() {
            self.set_anim_finished();
        }

        if !self.base().is_on_floor() {
            base_vel.y += (self.gravity * self.delta) as f32;
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
        godot_print!("{}", new_state.as_str());
        self.current_state = new_state;
        self.get_current_state().enter(self);
        self.update_animation();
        self.anim_finished = false;
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
        godot_print!("Recieved signal");
        self.anim_finished = true;
    }

    pub fn is_anim_finished(&self) -> bool {
        self.anim_finished
    }

    pub fn get_gravity(&self) -> f64 {
        self.gravity
    }

    fn update_animation(&mut self) {
        let mut sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        self.set_animation_direction(&mut sprite);

        let animation_name = StringName::from(self.get_current_state().as_str());
        if sprite.get_animation() != animation_name {
            sprite.set_animation(animation_name.into());
            sprite.play();
        }
    }

    fn set_animation_direction(&mut self, sprite: &mut Gd<AnimatedSprite2D>) {
        let mut scale = sprite.get_scale();
        let mut pos = sprite.get_position();

        if self.direction < 0.0 && scale.x != -1.0 {
            scale.x = -1.0;
            pos.x -= 9.0;
        } else if self.direction > 0.0 && scale.x != 1.0 {
            scale.x = 1.0;
            pos.x += 9.0;
        }

        sprite.set_scale(scale);
        sprite.set_position(pos);
    }
}
