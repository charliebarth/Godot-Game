use godot::builtin::meta::FromGodot;
use godot::builtin::StringName;
use godot::builtin::Vector2;
use godot::engine::AnimatedSprite2D;
use godot::engine::CharacterBody2D;
use godot::engine::ICharacterBody2D;
use godot::engine::Input;
use godot::engine::ProjectSettings;
use godot::prelude::*;

const MAX_JUMP_HEIGHT: f32 = 300.0;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    base: Base<CharacterBody2D>,
    state: PlayerState,
    direction: f32,
    gravity: f64,
    delta: f64,
    animation_duration: i8,
    animation_speed: f32,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        let path = GString::from("physics/2d/default_gravity");
        let gravity: f64 =
            FromGodot::try_from_variant(&ProjectSettings::singleton().get_setting(path)).unwrap();

        Self {
            base,
            state: PlayerState::Idle,
            direction: 1.0,
            gravity,
            delta: 0.0,
            animation_duration: 0,
            animation_speed: 1.0,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.delta = delta;
        let mut base_vel = self.base_mut().get_velocity();

        if !self.base().is_on_floor() {
            base_vel.y += (self.gravity * self.delta) as f32;
        } else {
            base_vel.y = 0.0;
        }
        self.base_mut().set_velocity(base_vel);

        self.update_animation();

        self.base_mut().move_and_slide();
    }
}

impl Player {
    /// Update the player's animation based on the current state.
    /// The player's animation is determined by the state of the player.
    /// Change the player's direction based on the horizontal direction.
    fn update_animation(&mut self) {
        // godot_print!("Player State Before: {:?}", self.state);
        // Go to current state and check to see if the player should transition to a different state.

        // godot_print!("Player State After: {:?}", self.state);

        // Call state method associated with the current state.
        self.enter_state();

        // Apply physics based on the current state.
        self.apply_state_physics();

        let mut sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        self.set_animation_direction(&mut sprite);

        let animation_name = StringName::from(self.state.as_str());
        if sprite.get_animation() != animation_name {
            self.animation_duration = 0;
            sprite.set_animation(animation_name.into());
            sprite.play();
        }

        sprite.set_speed_scale(self.animation_speed);
    }

    fn enter_state(&mut self) {
        match self.state {
            PlayerState::Run => {
                self.run_state();
            }
            PlayerState::Jump => {
                self.jump_state();
            }
            PlayerState::Idle => {
                self.idle_state();
            }
            PlayerState::Fall => {
                self.fall_state();
            }
            PlayerState::Land => {
                self.land_state();
            }
            PlayerState::JumpFallTransition => {
                self.jump_fall_transition_state();
            }
            PlayerState::Crouch => {
                self.crouch_state();
            }
            PlayerState::Slide => {
                self.slide_state();
            }
            PlayerState::Backstep => {
                self.backstep_state();
            }
            PlayerState::Roll => {
                self.roll_state();
            }
            PlayerState::Dead => {
                self.dead_state();
            }
            PlayerState::TurnAround => {
                self.turn_around_state();
            }
            PlayerState::Decelerate => {
                self.decelerate_state();
            }
        }
    }

    fn apply_state_physics(&mut self) {
        match self.state {
            PlayerState::Jump => {
                self.attempt_jump();
            }
            PlayerState::Run => {
                self.attempt_run();
            }
            PlayerState::Fall => {}
            PlayerState::Idle => {
                self.attempt_idle();
            }
            PlayerState::Backstep => {}
            PlayerState::Crouch => {}
            PlayerState::Dead => {}
            PlayerState::JumpFallTransition => {}
            PlayerState::Land => {}
            PlayerState::Roll => {}
            PlayerState::Slide => {}
            PlayerState::TurnAround => {
                self.attempt_turn_around();
            }
            PlayerState::Decelerate => {
                self.attempt_decelerate();
            }
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

    fn set_direction(&mut self, direction: f32) {
        if direction < 0.0 {
            self.direction = -1.0;
        } else if direction > 0.0 {
            self.direction = 1.0;
        }
    }

    fn get_horizontal_movement(&mut self) -> f32 {
        let move_left = StringName::from("move_left");
        let move_right = StringName::from("move_right");
        Input::singleton().get_axis(move_left, move_right)
    }

    fn apply_horizontal_velocity(&mut self, direction: f32, speed: f32) {
        let mut base = self.base_mut();
        let mut base_vel = base.get_velocity();
        base_vel.x = speed * direction;
        base.set_velocity(base_vel);
    }

    /// Attempt to move the player based on the horizontal direction.
    ///
    /// # Returns
    ///
    /// The horizontal direction of the player.
    fn attempt_run(&mut self) {
        let horizontal_dir = self.get_horizontal_movement();

        if horizontal_dir == 0.0 {
            return;
        }

        self.set_direction(horizontal_dir);
        self.apply_horizontal_velocity(horizontal_dir, 125.0);
    }

    fn attempt_turn_around(&mut self) {
        let horizontal_dir = self.get_horizontal_movement();

        if horizontal_dir == 0.0 {
            return;
        }

        self.apply_horizontal_velocity(horizontal_dir, 100.0);
    }

    /// Attempt to make the player jump. If the player is on the floor and the jump
    /// action is pressed, the player will jump.
    fn attempt_jump(&mut self) {
        let dir = self.get_horizontal_movement();
        self.apply_horizontal_velocity(dir, 125.0);

        let mut base = self.base_mut();
        if !base.is_on_floor() {
            return;
        }

        let jump_force = base.get_velocity().y + -MAX_JUMP_HEIGHT;
        let jump_vel = Vector2::new(base.get_velocity().x, base.get_velocity().y + jump_force);
        base.set_velocity(jump_vel);
    }

    /// Attempt to make the player idle. If the player is not moving, the player will
    /// idle.
    fn attempt_idle(&mut self) {
        let mut base = self.base_mut();
        let base_vel = base.get_velocity();
        let idle_vel = Vector2::new(0.0, base_vel.y);
        base.set_velocity(idle_vel);
    }

    fn attempt_decelerate(&mut self) {
        // Placeholder for deceleration which will be based on duration of deceleration animation.
        // This will also only played when the player has a momentum above a certain threshold.
        self.apply_horizontal_velocity(
            self.direction,
            125.0 - (30 * self.animation_duration) as f32,
        );
    }
}

/// The methods for each player state. These methods are used to determine if the player
/// should transition to a different state.
impl Player {
    /// Already jumping and in the air. Determine if player should be transitioned
    /// to a different state. Also apply physics.
    fn jump_state(&mut self) {
        let y_vel = self.base_mut().get_velocity().y;

        if y_vel > -10.0 && y_vel < 10.0 {
            self.state = PlayerState::JumpFallTransition;
        } else if y_vel >= 10.0 {
            self.state = PlayerState::Fall;
        } else if self.base_mut().is_on_floor() {
            self.state = PlayerState::Land;
        }
    }

    fn run_state(&mut self) {
        let horizontal_dir = self.get_horizontal_movement();

        if horizontal_dir == 0.0 {
            self.state = PlayerState::Decelerate;
            self.animation_duration = 0;
        } else if Input::singleton().is_action_just_pressed(StringName::from("jump"))
            && self.base().is_on_floor()
        {
            self.state = PlayerState::Jump;
        } else if !self.base().is_on_floor() {
            self.state = PlayerState::Fall;
        } else if horizontal_dir != self.direction {
            self.state = PlayerState::TurnAround;
            self.animation_duration = 0;
        } else if Input::singleton().is_action_just_pressed(StringName::from("roll")) {
            self.state = PlayerState::Roll;
        }
    }

    fn idle_state(&mut self) {
        let horizontal_dir = self.get_horizontal_movement();

        if horizontal_dir != 0.0 {
            self.state = PlayerState::Run;
        } else if Input::singleton().is_action_just_pressed(StringName::from("jump"))
            && self.base().is_on_floor()
        {
            self.state = PlayerState::Jump;
        } else if !self.base().is_on_floor() {
            self.state = PlayerState::Fall;
        } else if Input::singleton().is_action_just_pressed(StringName::from("roll")) {
            self.state = PlayerState::Roll;
        }
    }

    fn land_state(&mut self) {
        if self.base().is_on_floor() {
            self.state = PlayerState::Idle;
        } else {
            self.state = PlayerState::Fall;
        }
    }

    fn fall_state(&mut self) {
        if self.base().is_on_floor() {
            self.state = PlayerState::Land;
        }
    }

    fn jump_fall_transition_state(&mut self) {
        const TRANSITION_DURATION: i8 = 4;

        if self.base().is_on_floor() {
            self.state = PlayerState::Land;
        } else if self.animation_duration >= TRANSITION_DURATION {
            self.state = PlayerState::Fall;
        }

        self.animation_duration += 1;
    }

    fn crouch_state(&mut self) {
        // if Input::singleton().is_action_just_pressed(StringName::from("crouch")) {
        //     self.state = PlayerState::Crouch;
        // }
    }

    fn slide_state(&mut self) {
        // if Input::singleton().is_action_just_pressed(StringName::from("slide")) {
        //     self.state = PlayerState::Slide;
        // }
    }

    fn backstep_state(&mut self) {
        // if Input::singleton().is_action_just_pressed(StringName::from("backstep")) {
        //     self.state = PlayerState::Backstep;
        // }
    }

    fn roll_state(&mut self) {
        if !self.base().is_on_floor() {
            self.state = PlayerState::Fall
        }
    }

    fn dead_state(&mut self) {
        // if Input::singleton().is_action_just_pressed(StringName::from("dead")) {
        //     self.state = PlayerState::Dead;
        // }
    }

    fn turn_around_state(&mut self) {
        const TURN_AROUND_DURATION: i8 = 6;
        if self.animation_duration >= TURN_AROUND_DURATION {
            self.state = PlayerState::Run;
        }

        self.animation_duration += 1;
    }

    fn decelerate_state(&mut self) {
        const DECELERATE_DURATION: i8 = 4;
        let horizontal_dir = self.get_horizontal_movement();

        if horizontal_dir != self.direction && horizontal_dir != 0.0 {
            self.state = PlayerState::TurnAround;
        } else if horizontal_dir != 0.0 && horizontal_dir == self.direction {
            self.state = PlayerState::Run;
        } else if horizontal_dir == 0.0 && self.animation_duration >= DECELERATE_DURATION {
            self.state = PlayerState::Idle;
        } else if Input::singleton().is_action_just_pressed(StringName::from("jump"))
            && self.base().is_on_floor()
        {
            self.state = PlayerState::Jump;
        } else if !self.base().is_on_floor() {
            self.state = PlayerState::Fall;
        }

        self.animation_duration += 1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlayerState {
    Idle,
    Run,
    Jump,
    Fall,
    Land,
    Crouch,
    Slide,
    Backstep,
    Roll,
    Dead,
    JumpFallTransition,
    TurnAround,
    Decelerate,
    // Add attacks
}

/// A Finite State Machine for the player character.
/// It is used to determine the current state of the player which is used to
/// apply physics and animations.
impl PlayerState {
    fn as_str(&self) -> &str {
        match self {
            PlayerState::Idle => "idle",
            PlayerState::Run => "run",
            PlayerState::Jump => "jump",
            PlayerState::Fall => "fall",
            PlayerState::Land => "land",
            PlayerState::Crouch => "crouch",
            PlayerState::Slide => "slide",
            PlayerState::Backstep => "backstep",
            PlayerState::Roll => "roll",
            PlayerState::Dead => "dead",
            PlayerState::JumpFallTransition => "jump_fall_transition",
            PlayerState::TurnAround => "turn_around",
            PlayerState::Decelerate => "run", // Need to make decelerate animation
        }
    }
}
