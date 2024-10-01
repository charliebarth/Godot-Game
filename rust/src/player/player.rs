use godot::classes::AnimatedSprite2D;
use godot::classes::CharacterBody2D;
use godot::classes::ICharacterBody2D;
use godot::classes::ProjectSettings;
use godot::prelude::*;

use super::input_manager::InputManager;
use super::metal_manager::MetalManager;
use super::metal_reserve_bar_manager::MetalReserveBarManager;
use super::player_states::idle::Idle;
use super::traits::player_state::PlayerState;

const MAX_HEALTH: u8 = 100;
const MIN_HEALTH: u8 = 0;
const DEFAULT_RUN_SPEED: f32 = 200.0;
const DEFAULT_JUMP_FORCE: f32 = 450.0;
const MAX_RUN_SPEED: f32 = 300.0;
const MIN_RUN_SPEED: f32 = 100.0;
const MAX_JUMP_FORCE: f32 = 550.0;
const MIN_JUMP_FORCE: f32 = 300.0;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>,
    direction: f32,
    gravity: f64,
    health: u8,
    delta: f64,
    current_state: Box<dyn PlayerState>,
    previous_state: Box<dyn PlayerState>,
    anim_finished: bool,
    run_speed: f32,
    jump_force: f32,
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
            previous_state: Box::new(Idle),
            direction: 1.0,
            health: 100,
            delta: 0.0,
            gravity,
            anim_finished: false,
            run_speed: DEFAULT_RUN_SPEED,
            jump_force: DEFAULT_JUMP_FORCE,
        }
    }

    fn ready(&mut self) {
        // Assign starting metals to the player based on the game mode
        // TODO: Change this so that a dynamic game mode can be selected
        self.get_metal_manager()
            .bind_mut()
            .assign_starting_metals("last_player_standing");

        // Start the player in the idle state
        self.set_state(Box::new(Idle));
    }

    fn physics_process(&mut self, delta: f64) {
        self.set_delta(delta);

        let mut base_vel = self.base_mut().get_velocity();
        let sprite = self.get_sprite();

        // Check if the player's sprite is currently playing an animation
        if !sprite.is_playing() {
            self.set_anim_finished();
        }

        // Apply gravity to the player if they are not on the floor
        if !self.base().is_on_floor() {
            base_vel.y += (self.gravity * self.delta) as f32;
        } else {
            base_vel.y = 0.0;
        }

        self.base_mut().set_velocity(base_vel);

        // Reset the speed scale of the player's sprite to avoid old animation speeds from affecting the new animation
        // Also reset the run and jump force of the player to their default values
        let mut sprite: Gd<AnimatedSprite2D> = self.get_sprite();
        sprite.set_speed_scale(1.0);
        self.set_run_speed(DEFAULT_RUN_SPEED);
        self.set_jump_force(DEFAULT_JUMP_FORCE);

        // Update all metals held by the player
        self.get_metal_manager().bind_mut().update(self);

        // Update the current state of the player
        let current_state = self.get_current_state();
        current_state.update(self);
        self.update_animation();

        // Make the player move and slide based on their velocity
        self.base_mut().move_and_slide();
    }
}

impl Player {
    /// Set the current state of the player and trigger the enter method of the new state
    /// This method also sets the previous state of the player to the current state
    /// The enter method of the new state is triggered to allow for any initial and/orone-time logic to be executed
    ///
    /// # Arguments
    /// * `new_state` - The new state to set the player to
    pub fn set_state(&mut self, new_state: Box<dyn PlayerState>) {
        self.previous_state = self.get_current_state();
        self.current_state = new_state;
        self.get_current_state().enter(self);
    }

    /// Get the current state of the player
    ///
    /// # Returns
    /// * `Box<dyn PlayerState>` - The current state of the player
    pub fn get_current_state(&self) -> Box<dyn PlayerState> {
        self.current_state.clone()
    }

    /// Set the delta time of the player
    ///
    /// # Arguments
    /// * `delta` - The delta time to set
    fn set_delta(&mut self, delta: f64) {
        self.delta = delta;
    }

    /// Get the delta time of the player
    ///
    /// # Returns
    /// * `f64` - The delta time of the player
    pub fn get_delta(&self) -> f64 {
        self.delta
    }

    /// Get the health of the player
    ///
    /// # Returns
    /// * `u8` - The health of the player
    pub fn get_health(&self) -> u8 {
        self.health
    }

    /// Get the direction the player is facing
    ///
    /// # Returns
    /// * `f32` - The direction the player is facing
    pub fn get_dir(&self) -> f32 {
        self.direction
    }

    /// Set the direction the player is facing
    /// All values less than 0 are set to -1.0 or facing left
    /// All values greater than 0 are set to 1.0 or facing right
    ///
    /// # Arguments
    /// * `direction` - The direction to set the player to
    pub fn set_dir(&mut self, direction: f32) {
        if direction < 0.0 {
            self.direction = -1.0;
        } else if direction > 0.0 {
            self.direction = 1.0;
        }
    }

    /// Adjust the health of the player
    ///
    /// # Arguments
    /// * `health` - The amount to adjust the health by
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

    pub fn apply_horizontal_velocity(&mut self, direction: f32, max_speed: f32) {
        let mut base = self.base_mut();
        let mut base_vel = base.get_velocity();
        base_vel.x = max_speed * direction;
        base.set_velocity(base_vel);
    }

    /// Set the animation finished flag to true
    pub fn set_anim_finished(&mut self) {
        self.anim_finished = true;
    }

    /// Check if the player's animation is finished
    ///
    /// # Returns
    /// * `bool` - True if the animation is finished, false otherwise
    pub fn is_anim_finished(&self) -> bool {
        self.anim_finished
    }

    /// Get the gravity of the player
    ///
    /// # Returns
    /// * `f64` - The gravity of the player
    pub fn get_gravity(&self) -> f64 {
        self.gravity
    }

    /// Update the animation of the player
    /// This method sets the animation of the player based on the current state of the player
    /// It also sets the animation direction based on the direction the player is facing
    ///
    /// If animation from the current state is not the one being played, the animation is changed and the animation finished flag is reset
    /// The animation is then played
    fn update_animation(&mut self) {
        let mut sprite = self.get_sprite();

        self.set_animation_direction(&mut sprite);

        let animation_name = StringName::from(self.get_current_state().as_str(self));
        if sprite.get_animation() != animation_name {
            self.anim_finished = false;
            sprite.set_animation(animation_name.into());
            sprite.play();
        }
    }

    /// Set the animation direction of the player
    /// This method sets the direction of the player's sprite based on the direction the player is facing
    /// This also changes the position of the sprite to ensure it is centered in the player's hitbox
    ///
    /// # Arguments
    /// * `sprite` - The sprite to set the animation direction of
    fn set_animation_direction(&mut self, sprite: &mut Gd<AnimatedSprite2D>) {
        let mut scale = sprite.get_scale();
        let mut pos = sprite.get_position();

        if self.direction < 0.0 && scale.x != -1.3 {
            scale.x = -1.3;
            pos.x -= 9.0;
        } else if self.direction > 0.0 && scale.x != 1.3 {
            scale.x = 1.3;
            pos.x += 9.0;
        }

        sprite.set_scale(scale);
        sprite.set_position(pos);
    }

    /// Get the previous state of the player
    ///
    /// # Returns
    /// * `Box<dyn PlayerState>` - The previous state of the player
    pub fn get_previous_state(&self) -> Box<dyn PlayerState> {
        self.previous_state.clone()
    }

    /// Set the previous state of the player
    ///
    /// # Arguments
    /// * `state` - The state to set the previous state to
    pub fn set_previous_state(&mut self, state: Box<dyn PlayerState>) {
        self.previous_state = state;
    }

    /// Get the run speed of the player
    ///
    /// # Returns
    /// * `f32` - The run speed of the player
    pub fn get_run_speed(&self) -> f32 {
        self.run_speed
    }

    /// Get the jump force of the player
    ///
    /// # Returns
    /// * `f32` - The jump force of the player
    pub fn get_jump_force(&self) -> f32 {
        self.jump_force
    }

    /// Set the run speed of the player
    ///
    /// # Arguments
    /// * `speed` - The speed to set the player to
    pub fn set_run_speed(&mut self, speed: f32) {
        self.run_speed = speed.clamp(MIN_RUN_SPEED, MAX_RUN_SPEED);
    }

    /// Set the jump force of the player
    ///
    /// # Arguments
    /// * `force` - The force to set the player to
    pub fn set_jump_force(&mut self, force: f32) {
        self.jump_force = force.clamp(MIN_JUMP_FORCE, MAX_JUMP_FORCE);
    }
}

/// Getters for nodes
impl Player {
    /// Getter for the InputManager node
    ///
    /// # Returns
    /// * `InputManager` - The InputManager node
    pub fn get_input_manager(&self) -> Gd<InputManager> {
        self.base().get_node_as::<InputManager>("InputManager")
    }

    /// Getter for the MetalManager node
    ///
    /// # Returns
    /// * `MetalManager` - The MetalManager node
    pub fn get_metal_manager(&self) -> Gd<MetalManager> {
        self.base().get_node_as::<MetalManager>("MetalManager")
    }

    /// Getter for the AnimatedSprite2D node
    ///
    /// # Returns
    /// * `AnimatedSprite2D` - The AnimatedSprite2D node
    pub fn get_sprite(&self) -> Gd<AnimatedSprite2D> {
        self.base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D")
    }

    /// Getter for the MetalReserveBarManager node
    ///
    /// # Returns
    /// * `MetalReserveBarManager` - The MetalReserveBarManager node
    pub fn get_metal_reserve_bar_manager(&self) -> Gd<MetalReserveBarManager> {
        self.base()
            .get_node_as::<MetalReserveBarManager>("MetalReserveBarManager")
    }
}
