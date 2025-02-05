use std::collections::HashMap;
use std::collections::VecDeque;
use std::time::Duration;
use std::time::Instant;

use godot::classes::CanvasItem;
use godot::classes::CharacterBody2D;
use godot::classes::GpuParticles2D;
use godot::classes::ICharacterBody2D;
use godot::classes::PointLight2D;
use godot::classes::ProjectSettings;
use godot::classes::Sprite2D;
use godot::classes::SubViewport;
use godot::classes::TextureProgressBar;
use godot::classes::{AnimatedSprite2D, Area2D};
use godot::prelude::*;

use crate::game::Game;
use crate::items::coin::Coin;
use crate::metal_object::MetalObject;
use crate::ui::metal_reserve_bar_manager::MetalReserveBarManager;

use super::disconnected::Disconnected;
use super::enums::force::Force;
use super::enums::player_events::PlayerEvents;
use super::enums::player_states::PlayerStates;
use super::enums::timeout_events::TimeoutEvents;
use super::input_manager::InputManager;
use super::metal_line::MetalLine;
use super::metal_manager::MetalManager;
use crate::ui::coin_counter::CoinCounter;

const MAX_HEALTH: f64 = 100.0;
const MIN_HEALTH: f64 = 0.0;
const DEFAULT_RUN_SPEED: f32 = 250.0;
const DEFAULT_JUMP_FORCE: f32 = 450.0;
const MAX_RUN_SPEED: f32 = 600.0;
const MIN_RUN_SPEED: f32 = 0.0;
const MAX_JUMP_FORCE: f32 = 700.0;
const MIN_JUMP_FORCE: f32 = 300.0;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    /// The base node of the player
    base: Base<CharacterBody2D>,
    /// The direction the player is facing
    direction: f32,
    /// The gravity of the player
    gravity: f64,
    /// The health of the player
    health: f64,
    /// The amount of time that has passed since the last frame
    delta: f64,
    /// The current state of the player
    current_state: PlayerStates,
    /// The previous state of the player
    previous_state: PlayerStates,
    /// A flag to determine if the player's animation has finished
    anim_finished: bool,
    /// The current maximum run speed of the player
    run_speed: f32,
    /// The current maximum jump force of the player
    jump_force: f32,
    /// The device ID the player should listen for input from
    device_id: i32,
    /// The ID of the player
    player_id: i32,
    /// A HashMap of timeout events that the player is currently tracking
    /// The key is the event and the value is a tuple of the start time and duration of the event
    /// When the time since the start time is greater than the duration, the event is removed from the HashMap
    timeout_events: HashMap<TimeoutEvents, (Instant, Duration)>,
    // TODO: Make all of these cached nodes a HashMap rather than a bunch of individual Option fields
    /// The animated sprite of the player
    sprite: Option<Gd<AnimatedSprite2D>>,
    /// The input manager of the player
    input_manager: Option<Gd<InputManager>>,
    /// The metal manager of the player
    metal_manager: Option<Gd<MetalManager>>,
    /// The metal reserve bar manager of the player
    metal_reserve_bar_manager: Option<Gd<MetalReserveBarManager>>,
    /// The health bar of the player
    health_bar: Option<Gd<TextureProgressBar>>,
    /// The coin counter of the player
    coin_counter: Option<Gd<CoinCounter>>,
    /// The point light of the player
    point_light: Option<Gd<PointLight2D>>,
    /// A vec of the animated sprites of the current player that are shown to other players
    player_vis: Vec<Gd<AnimatedSprite2D>>,
    /// The node that draws lines to nearby metal objects
    metal_line: Option<Gd<MetalLine>>,
    /// A metal line selector node
    /// This done by getting the angle of the node and the angle of metal object and comparing them
    line_selector: Option<Gd<Sprite2D>>,
    /// Particles that are emitted when the player is burning or low burning pewter
    /// This is a visual reminder to the player that they are using pewter
    pewter_particles: Option<Gd<GpuParticles2D>>,
    /// Particles that are emitted when the player is burning or low burning steel
    /// This is a visual reminder to the player that they are using steel
    steel_particles: Option<Gd<GpuParticles2D>>,
    /// This node has a timer that counts down and removes the player from the game if it reaches 0
    disconnected: Option<Gd<Disconnected>>,
    /// A queue of forces to be applied to the player
    forces: VecDeque<Force>,
    /// A vec of nearby metal objects that can be used by steel and iron
    metal_objects: Vec<Gd<MetalObject>>,
    /// The mass of the player in kilograms
    mass: f32,
    is_steel_burning: bool,
    /// If the player is attacking or not
    is_attacking: bool,
}

#[godot_api]
impl ICharacterBody2D for Player {
    /// The Godot contructor for the Player class node
    ///
    /// # Arguments
    /// * `base` - The base node type for the player
    ///
    /// # Returns
    /// * `Player` - The Player node
    fn init(base: Base<CharacterBody2D>) -> Self {
        let path = GString::from("physics/2d/default_gravity");
        let gravity: f64 =
            FromGodot::try_from_variant(&ProjectSettings::singleton().get_setting(&path)).unwrap();

        Self {
            base,
            direction: 1.0,
            health: MAX_HEALTH,
            delta: 0.0,
            gravity,
            current_state: PlayerStates::Jump,
            previous_state: PlayerStates::Fall,
            anim_finished: false,
            run_speed: DEFAULT_RUN_SPEED,
            jump_force: DEFAULT_JUMP_FORCE,
            device_id: 0,
            player_id: 0,
            timeout_events: HashMap::new(),
            sprite: None,
            input_manager: None,
            metal_manager: None,
            metal_reserve_bar_manager: None,
            health_bar: None,
            coin_counter: None,
            point_light: None,
            player_vis: Vec::new(),
            metal_line: None,
            line_selector: None,
            pewter_particles: None,
            steel_particles: None,
            disconnected: None,
            forces: VecDeque::new(),
            metal_objects: Vec::new(),
            mass: 70.0,
            is_steel_burning: false,
            is_attacking: false,
        }
    }

    /// The Godot method called when the player enters the scene tree for the first time
    /// Any one time logic and initialization should be done here
    /// NOTE: This only is called the very first time the instance enters the scene tree
    fn ready(&mut self) {
        // Assign starting metals to the player based on the game mode
        // TODO: Change this so that a dynamic game mode can be selected
        self.get_metal_manager()
            .bind_mut()
            .assign_starting_metals("last_player_standing");

        // Start the player in the idle state
        self.set_state(PlayerStates::Idle);

        // Set the health bar to the player's health
        self.get_health_bar().set_value(self.get_health());
    }

    /// The Godot method called every physics frame
    /// Physics frames happen a static number of times per second as opposed to process frames which happen as often as possible
    ///
    /// # Arguments
    /// * `delta` - The time since the last frame
    fn physics_process(&mut self, delta: f64) {
        if self.health <= 0.0 {
            self.die();
        }

        // If the die button is pressed, the player dies
        // This is used for testing as a quick way to simulate player death
        // This will be either removed or disabled during playtesting
        if self
            .get_input_manager()
            .bind_mut()
            .check_for_player_event(PlayerEvents::Die)
        {
            self.adjust_health(-100.0);
        }

        self.set_delta(delta);

        self.add_force(Force::Gravity {
            acceleration: self.gravity,
        });

        if self.base().is_on_floor() {
            self.add_force(Force::NormalForce { magnitude: -1.0 });
        }

        // Reset the player to their default values such as animation speed, run speed, and jump force
        self.reset_player();

        // Update all metals held by the player
        self.get_metal_manager().bind_mut().update(self);

        // Update the current state of the player
        self.current_state.update_state(self);
        self.set_animation_direction();

        // Check for any timeout events that have expired
        self.expire_timeout_events();

        // Make the player move and slide based on their velocity
        self.apply_forces();
        self.base_mut().move_and_slide();
    }
}

#[godot_api]
impl Player {
    #[func]
    /// A method that makes the player die
    /// This will clean up the player and the viewport they are in
    /// as well as notify the game that the player has died
    pub fn die(&mut self) {
        let mut camera = Camera2D::new_alloc();
        camera.set_name("OverviewCamera");
        camera.set_position(Vector2::new(20.0, -225.0));
        camera.set_zoom(Vector2::new(0.37, 0.37));

        //overview_container.set_canvas_cull_mask(1);
        let mut parent_viewport = self
            .base()
            .get_parent()
            .unwrap()
            .try_cast::<SubViewport>()
            .unwrap();

        parent_viewport.set_canvas_cull_mask(1);
        parent_viewport.add_child(&camera);
        self.base_mut().queue_free();
        self.base()
            .get_node_as::<Game>("/root/Game")
            .bind_mut()
            .remove_player(self.player_id);
    }

    /// Set the current state of the player and triggers the enter method of the new state
    /// This method also sets the previous state of the player to the current state
    /// The enter method of the new state is triggered to allow for any initial and/or one-time logic to be executed
    ///
    /// # Arguments
    /// * `new_state` - The new state to set the player to
    pub fn set_state(&mut self, new_state: PlayerStates) {
        if self.current_state == new_state {
            return;
        }

        self.update_animation(new_state.as_str().into());

        self.previous_state = self.current_state;
        self.current_state = new_state;

        self.current_state.enter_state(self);
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
    /// * `f64` - The health of the player
    pub fn get_health(&self) -> f64 {
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

    #[func]
    /// Adjust the health of the player
    /// Health is clamped between MIN_HEALTH and MAX_HEALTH
    ///
    /// # Arguments
    /// * `adjustment` - The amount to adjust the health by
    pub fn adjust_health(&mut self, adjustment: f64) {
        // Adjust health by the specified amount
        self.health += adjustment;

        // Clamp health between MIN_HEALTH and MAX_HEALTH
        self.health = self.health.clamp(MIN_HEALTH, MAX_HEALTH);

        // Update the health bar of the player
        self.get_health_bar().set_value(self.get_health());
    }

    /// Adjusts the coins in this players coin_counter positively or negatively.
    ///
    /// # Arguments
    /// * `pos_neg` (i8) - if -1, remove_coin    if +1, add_coin
    pub fn adjust_coins(&mut self, pos_neg: i8, coin: &mut Coin) {
        if pos_neg == -1 {
            // Dereference and call the method
            self.get_coin_counter().bind_mut().remove_coin();
        } else {
            self.get_coin_counter().bind_mut().add_coin(coin);
        }
    }

    /// Adjusts specific metals in this players metal bar manager to some amount.
    ///
    /// # Arguments
    /// * `metals` (`Vec<StringName>`) - the metals to effect
    /// * `amt` (f64) - the new amount to set the metals to
    pub fn adjust_metals(&mut self, metals: Vec<StringName>, amt: f64) {
        for metal in metals {
            self.get_metal_manager()
                .bind_mut()
                .increase_metal_reserve(metal, amt);
        }
    }

    /// Updates a specific metal reserve bar to a new amount
    ///
    /// # Arguments
    /// * `metal` - The metal to update
    /// * `amt` - The new amount to set the metal to
    pub fn set_metal_reserve_amount(&mut self, metal: StringName, amt: f64) {
        self.get_metal_reserve_bar_manager()
            .bind_mut()
            .set_metal_amount(metal, amt);
    }

    /// Represents the direction the player is trying to move
    /// Returns 1 when the move right button is pressed, -1 when the move left button is pressed, and 0 if neither is pressed
    ///
    /// # Returns
    /// * `f32` - The direction the player is trying to move as well as the magnitude of the movement
    pub fn get_horizontal_movement(&mut self) -> f32 {
        let move_left = StringName::from(format!("move_left{}", self.device_id));
        let move_right = StringName::from(format!("move_right{}", self.device_id));
        Input::singleton().get_axis(&move_left, &move_right)
    }

    /// Sets the player's velocity to the speed passed * the magnitude of the direction passed
    /// NOTE: This is deprecated and will be removed. This has been replaced with the player's forces queue and
    /// the Force enum
    ///
    /// # Arguments
    /// * `direction` - The direction and magnitude to move the player
    /// * `max_speed` - The speed to move the player (ignoring direction)
    pub fn apply_horizontal_velocity(&mut self, direction: f32, max_speed: f32) {
        let mut base = self.base_mut();
        let mut base_vel = base.get_velocity();
        base_vel.x = max_speed * direction;
        base.set_velocity(base_vel);
    }

    #[func]
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

    /// Set the gravity of the player
    ///
    /// # Arguments
    /// * `gravity` - The gravity to set
    pub fn set_gravity(&mut self, gravity: f64) {
        self.gravity = gravity;
    }

    /// Update the animation of the player
    /// This method sets the animation of the player based on the current state of the player
    /// It also sets the animation direction based on the direction the player is facing
    ///
    /// If animation from the current state is not the one being played, the animation is changed and the animation finished flag is reset
    /// The animation is then played
    ///
    /// # Arguments
    /// * `animation_name` - The name of the animation to set
    fn update_animation(&mut self, animation_name: StringName) {
        self.set_animation_direction();

        let mut sprite = self.get_sprite();
        self.anim_finished = false;
        sprite.set_animation(&animation_name);
        sprite.play();

        for player_vis in self.player_vis.iter_mut() {
            player_vis.set_animation(&animation_name);
            player_vis.play();
        }
    }

    /// Set the animation direction of the player
    /// This method sets the direction of the player's sprite based on the direction the player is facing
    /// This also changes the position of the sprite to ensure it is centered in the player's hitbox
    fn set_animation_direction(&mut self) {
        let mut sprite = self.get_sprite();
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

        for player_vis in self.player_vis.iter_mut() {
            player_vis.set_scale(scale);
            player_vis.set_position(pos);
        }
    }

    /// Sets the speed of the player's animations
    ///
    /// # Arguments
    /// * `speed` - The speed to set the player's animations to
    pub fn set_animation_speed(&mut self, speed: f32) {
        let mut sprite = self.get_sprite();
        sprite.set_speed_scale(speed);

        for player_vis in self.player_vis.iter_mut() {
            player_vis.set_speed_scale(speed);
        }
    }

    /// Get the previous state of the player
    ///
    /// # Returns
    /// * `PlayerStates` - The previous state of the player
    pub fn get_previous_state(&self) -> PlayerStates {
        self.previous_state
    }

    /// Get the previous state of the player as a string
    ///
    /// # Returns
    /// * `String` - The previous state of the player as a string
    #[func]
    pub fn get_previous_state_str(&self) -> String {
        self.previous_state.as_str().into()
    }

    /// Get the current state of the player as a string
    ///
    /// # Returns
    /// * `String` - The current state of the player as a string
    #[func]
    pub fn get_current_state_str(&self) -> String {
        self.current_state.as_str().into()
    }

    /// A sliding upper limit for the player's run speed
    /// This is changed based on how far the joystick is pressed
    ///
    /// # Returns
    /// * `f32` - The current maximum run speed of the player
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

    /// Set the device ID of the player
    ///
    /// # Arguments
    /// * `device_id` - The device ID to set
    pub fn set_device_id(&mut self, device_id: i32) {
        self.device_id = device_id;
        let mut input_manager_unbound = self.get_input_manager();
        input_manager_unbound.bind_mut().set_device_id(device_id);
    }

    /// Get the device ID of the player
    ///
    /// # Returns
    /// * `i32` - The device ID of the player
    pub fn get_device_id(&self) -> i32 {
        self.device_id
    }

    /// Add a timeout event to the player
    /// This method adds a timeout event to the player and sets the duration of the event using the event's get_duration method
    /// The event is then inserted into the timeout_events HashMap with the current time as the start time
    ///
    /// # Arguments
    /// * `event` - The event to add
    pub fn add_timeout_event(&mut self, event: TimeoutEvents) {
        let duration = event.get_duration();
        self.timeout_events
            .insert(event, (Instant::now(), duration));
    }

    /// Check if the player is able to jump
    /// This method checks if the player is on the floor or if they are within the coyote time window
    /// If either condition is met, the player is able to jump and the method returns true
    /// Otherwise, the player is not able to jump and the method returns false
    ///
    /// # Returns
    /// * `bool` - True if the player is able to jump, false otherwise
    pub fn jump_available(&self) -> bool {
        if self.base().is_on_floor() {
            return true;
        }

        if let Some(_) = self.timeout_events.get(&TimeoutEvents::CoyoteTime) {
            return true;
        }

        false
    }

    /// Check if any timeout events have expired and remove them from the timeout_events HashMap
    fn expire_timeout_events(&mut self) {
        self.timeout_events.retain(|_event, time_tuple| {
            let time_elapsed = Instant::now().duration_since(time_tuple.0);
            time_elapsed <= time_tuple.1
        });
    }

    /// Set the player ID of the player
    /// This ID is assigned to the player when they join the game and is set by the PlayerManager
    /// This method also sets the visibility and light layers for all of the player's children
    /// It then emits the id_changed signal so any children with specific layer logic can update themselves
    ///
    /// # Arguments
    /// * `player_id` - The player ID to set
    pub fn set_player_id(&mut self, player_id: i32) {
        self.player_id = player_id;

        for child in self.base_mut().get_children().iter_shared() {
            if let Ok(mut node) = child.try_cast::<CanvasItem>() {
                let layer_num = player_id * 2;
                node.set_visibility_layer(1 << layer_num);
                node.set_light_mask(1023);
            }
        }

        self.base_mut().emit_signal("id_changed", &[]);
    }

    #[func]
    /// Get the player ID of the player
    /// This ID will be the same as the player number in the game so if this player was the first player to join, their ID would be 1
    ///
    /// # Returns
    /// * `i32` - The player ID of the player
    pub fn get_player_id(&self) -> i32 {
        self.player_id
    }

    /// Reset the player to their default values
    /// This method resets the speed scale of the player's sprite to 1.0
    /// It also resets the run and jump force of the player to their default values
    fn reset_player(&mut self) {
        let mut sprite: Gd<AnimatedSprite2D> = self.get_sprite();
        sprite.set_speed_scale(1.0);
        self.set_run_speed(DEFAULT_RUN_SPEED);
        self.set_jump_force(DEFAULT_JUMP_FORCE);

        for player_vis in self.player_vis.iter_mut() {
            player_vis.set_speed_scale(1.0);
        }
    }

    /// Adds a force to the player's forces queue
    ///
    /// # Arguments
    /// * `force` - The force to add to the player's forces queue
    pub fn add_force(&mut self, force: Force) {
        self.forces.push_back(force);
    }

    /// This iterates through the forces queue and applies each force to the player
    fn apply_forces(&mut self) {
        let len_forces = self.forces.len();
        for _ in 0..len_forces {
            let force = self.forces.pop_front().unwrap();
            self.apply_force(force);
        }
    }

    /// This method takes a force and then applies it to the player
    /// using the appropriate logic for the force
    /// NOTE: This will most likely have sub methods added for the logic of applying each force so
    /// that this method is cleaner
    ///
    /// # Arguments
    /// * `force` - The force to apply to the player
    fn apply_force(&mut self, force: Force) {
        let mut base_velocity = self.base().get_velocity();

        match force {
            Force::Gravity { acceleration } => {
                base_velocity.y += (acceleration * self.delta) as f32;
            }
            Force::NormalForce { magnitude } => {
                base_velocity.y += (self.gravity * magnitude * self.delta) as f32;
            }
            Force::Jump { acceleration } => {
                base_velocity.y += acceleration;
            }
            Force::Run { acceleration } => {
                let max_run_speed = self.get_run_speed();
                if base_velocity.x.abs() < max_run_speed && acceleration != 0.0 {
                    base_velocity.x += acceleration * self.delta as f32;
                } else if acceleration == 0.0 {
                    base_velocity.x = 0.0;
                }

                base_velocity.x = base_velocity.x.clamp(-max_run_speed, max_run_speed);
            }
            Force::AirRun { acceleration } => {
                let max_run_speed = self.get_run_speed();
                if base_velocity.x.abs() < max_run_speed && acceleration != 0.0 {
                    base_velocity.x += acceleration * self.delta as f32;
                } else if acceleration == 0.0 {
                    base_velocity.x = 0.0;
                }

                base_velocity.x = base_velocity.x.clamp(-max_run_speed, max_run_speed);
            }
            Force::Stop {
                horizontal,
                vertical,
            } => {
                base_velocity.x = if horizontal { 0.0 } else { base_velocity.x };
                base_velocity.y = if vertical { 0.0 } else { base_velocity.y };
            }
            Force::SteelPush {
                x_velocity,
                y_velocity,
            } => {
                base_velocity.x = x_velocity;
                base_velocity.y = y_velocity;
            }
        }

        self.base_mut().set_velocity(base_velocity);
    }

    /// The permanent minimum run speed of the player
    ///
    /// # Returns
    /// * `f32` - The minimum run speed of the player
    pub fn get_min_run_speed(&self) -> f32 {
        MIN_RUN_SPEED
    }

    #[func]
    /// Adds a metal object to the player's list of nearby metal objects
    ///
    /// # Arguments
    /// * `metal` - The metal object to add to the player's list of nearby metal objects
    fn add_metal_object(&mut self, metal: Gd<MetalObject>) {
        self.metal_objects.push(metal);
    }

    #[func]
    /// Removes a metal object from the player's list of nearby metal objects
    ///
    /// # Arguments
    /// * `metal` - The metal object to remove from the player's list of nearby metal objects
    fn remove_metal_object(&mut self, metal: Gd<MetalObject>) {
        if let Some(pos) = self.metal_objects.iter().position(|x| *x == metal) {
            self.metal_objects.remove(pos);
        }
    }

    /// Gets the vec of all nearby metal objects
    ///
    /// # Returns
    /// * `Vec<Gd<MetalObject>>` - The vec of all nearby metal objects
    pub fn get_metal_objects(&self) -> &Vec<Gd<MetalObject>> {
        &self.metal_objects
    }

    /// Gets the mass of the player in kilograms
    ///
    /// # Returns
    /// * `f32` - The mass of the player in kilograms
    pub fn get_mass(&self) -> f32 {
        self.mass
    }

    /// Checks if the player is burning steel
    /// This is used by the fall state so that the player can be "falling" while
    /// burning steel while ignoring transitions to new states
    ///
    /// # Returns
    /// * `bool` - True if the player is burning steel, false otherwise
    pub fn get_is_steel_burning(&self) -> bool {
        self.is_steel_burning
    }

    /// Sets if the player is burning steel
    ///
    /// # Arguments
    /// * `is_steel_burning` - A boolean that determines if the player is burning steel
    pub fn set_is_steel_burning(&mut self, is_steel_burning: bool) {
        self.is_steel_burning = is_steel_burning;
    }

    /// Get the angle of the metal object closest to the angle of the LineSelector if the angle is within the range of the LineSelector
    ///
    /// # Returns
    /// * `Option<f64>` - The angle of the nearest metal object if there is one within the range of the LineSelector
    pub fn get_nearest_metal_object(&mut self) -> Option<f64> {
        let mut nearest_metal_object: Option<f64> = None;
        let max_angle: f64 = 5.0_f64.to_radians(); // Define max angle difference in radians

        let line_selector_position = self.get_line_selector().get_global_position();
        let player_position = self.base().get_global_position();

        // Calculate angle of the line selector relative to the player
        let line_selector_angle = (line_selector_position.y as f64 - player_position.y as f64)
            .atan2(line_selector_position.x as f64 - player_position.x as f64);

        let mut current_shortest_angle_diff: f64 = f64::MAX;
        for metal_object in self.get_metal_objects().iter() {
            let metal_object_position = metal_object.get_global_position();

            // Calculate angle of the metal object relative to the player
            let metal_object_angle = (metal_object_position.y as f64 - player_position.y as f64)
                .atan2(metal_object_position.x as f64 - player_position.x as f64);

            // Calculate wrapped angle difference
            let angle_diff = ((metal_object_angle - line_selector_angle + std::f64::consts::PI)
                % (2.0 * std::f64::consts::PI))
                - std::f64::consts::PI;

            if angle_diff.abs() < max_angle && angle_diff.abs() < current_shortest_angle_diff {
                current_shortest_angle_diff = angle_diff.abs();
                nearest_metal_object = Some(metal_object_angle);
            }
        }

        nearest_metal_object
    }

    /// Enable the hitbox of the player when they are attacking
    ///
    /// # Arguments
    /// * `owner` - A reference to the node for the hitbox of the player
    pub fn enable_hitbox(&mut self) {
        self.is_attacking = true;
        // Get the hitbox of the player
        let mut hitbox = self.base().get_node_as::<Area2D>("Hitbox");
        // Enable the hitbox of the player
        hitbox.set_monitoring(true);
        hitbox.set_collision_layer(1 << 2);
    }

    /// Disable the hitbox of the player when they are not attacking
    ///
    /// # Arguments
    /// * `owner` - A reference to the node for the hitbox of the player
    pub fn disable_hitbox(&mut self) {
        self.is_attacking = false;
        // Get the hitbox of the player
        let mut hitbox = self.base().get_node_as::<Area2D>("Hitbox");
        // Disable the hitbox of the player
        hitbox.set_monitoring(false);
        hitbox.set_collision_layer(1 << 3);
    }

    /// A signal that is emmited by the player when it's id is changed
    /// Children of the player can listen for the signal and then change their visibility layer based on the new id
    #[signal]
    pub fn id_changed();

    /// If passed true, the player turns on its timer to count down before the player is removed from the game
    /// If passed false, the player turns off its timer meaning it is no longer disconnected
    ///
    /// # Arguments
    /// * `disconnected` - A boolean that determines if the player is disconnected or not
    pub fn set_disconnected(&mut self, disconnected: bool) {
        let mut disconnected_node = self.get_disconnected();
        disconnected_node.set_visible(disconnected);
    }
}
/// Getters for nodes
impl Player {
    /// Getter for the InputManager node
    /// This effectively caches the InputManager node so that it does not have to be found every time it is needed
    ///
    /// # Returns
    /// * `InputManager` - The InputManager node
    pub fn get_input_manager(&mut self) -> Gd<InputManager> {
        if self.input_manager.is_none() {
            self.input_manager = Some(self.base().get_node_as::<InputManager>("InputManager"));
        }

        self.input_manager
            .as_ref()
            .expect("InputManager node not found")
            .clone()
    }

    /// Getter for the MetalManager node
    /// This effectively caches the MetalManager node so that it does not have to be found every time it is needed
    ///
    /// # Returns
    /// * `MetalManager` - The MetalManager node
    pub fn get_metal_manager(&mut self) -> Gd<MetalManager> {
        if self.metal_manager.is_none() {
            self.metal_manager = Some(self.base().get_node_as::<MetalManager>("MetalManager"));
        }

        self.metal_manager
            .as_ref()
            .expect("MetalManager node not found")
            .clone()
    }

    /// Getter for the AnimatedSprite2D node
    /// This effectively caches the AnimatedSprite2D node so that it does not have to be found every time it is needed
    ///
    /// # Returns
    /// * `AnimatedSprite2D` - The AnimatedSprite2D node
    pub fn get_sprite(&mut self) -> Gd<AnimatedSprite2D> {
        if self.sprite.is_none() {
            self.sprite = Some(self.base().get_node_as::<AnimatedSprite2D>("OwnerVis"));

            let player_vis_one = self.base().get_node_as::<AnimatedSprite2D>("Player1Vis");
            let player_vis_two = self.base().get_node_as::<AnimatedSprite2D>("Player2Vis");
            let player_vis_three = self.base().get_node_as::<AnimatedSprite2D>("Player3Vis");

            self.player_vis.push(player_vis_one);
            self.player_vis.push(player_vis_two);
            self.player_vis.push(player_vis_three);
        }

        self.sprite
            .as_ref()
            .expect("OwnerVis node not found")
            .clone()
    }

    /// Getter for the MetalReserveBarManager node
    /// This effectively caches the MetalReserveBarManager node so that it does not have to be found every time it is needed
    ///
    /// # Returns
    /// * `MetalReserveBarManager` - The MetalReserveBarManager node
    pub fn get_metal_reserve_bar_manager(&mut self) -> Gd<MetalReserveBarManager> {
        if self.metal_reserve_bar_manager.is_none() {
            self.metal_reserve_bar_manager = Some(
                self.base()
                    .get_node_as::<MetalReserveBarManager>("MetalReserveBarManager"),
            );
        }

        self.metal_reserve_bar_manager
            .as_ref()
            .expect("MetalReserveBarManager node not found")
            .clone()
    }

    /// Getter for the HealthBar node
    /// This effectively caches the HealthBar node so that it does not have to be found every time it is needed
    ///
    /// # Returns
    /// * `TextureProgressBar` - The TextureProgressBar node used to display the player's health
    pub fn get_health_bar(&mut self) -> Gd<TextureProgressBar> {
        if self.health_bar.is_none() {
            self.health_bar = Some(self.base().get_node_as::<TextureProgressBar>("HealthBar"));
        }

        self.health_bar
            .as_ref()
            .expect("HealthBar node not found")
            .clone()
    }

    /// Getter for CoinCounter node
    /// This effectively caches the CoinCounter node so that it does not have to be found every time it is needed
    ///
    /// # Returns
    /// *  `CoinCounter` - The CoinCounter node used to show player coins.
    pub fn get_coin_counter(&mut self) -> Gd<CoinCounter> {
        if self.coin_counter.is_none() {
            self.coin_counter = Some(
                self.base()
                    .get_node_as::<CoinCounter>("Coin_Counter_Panel/CoinCounter"),
            );
        }
        self.coin_counter
            .as_ref()
            .expect("Could not find CoinCounter")
            .clone()
    }

    /// Getter for the PointLight2D node
    /// This effectively caches the PointLight2D node so that it does not have to be found every time it is needed
    ///
    /// # Returns
    /// * `PointLight2D` - The PointLight2D node
    pub fn get_point_light(&mut self) -> Gd<PointLight2D> {
        if self.point_light.is_none() {
            self.point_light = Some(self.base().get_node_as::<PointLight2D>("PointLight2D"));
        }

        self.point_light
            .as_ref()
            .expect("PointLight2D node not found")
            .clone()
    }

    /// Getter for the MetalLine node
    /// This effectively caches the MetalLine node so that it does not have to be found every time it is needed
    ///
    /// # Returns
    /// * `MetalLine` - The MetalLine node
    pub fn get_metal_line(&mut self) -> Gd<MetalLine> {
        if self.metal_line.is_none() {
            self.metal_line = Some(self.base().get_node_as::<MetalLine>("MetalLine"));
        }

        self.metal_line
            .as_ref()
            .expect("MetalLine node not found")
            .clone()
    }

    /// Getter for the LineSelector node
    /// This effectively caches the LineSelector node so that it does not have to be found every time it is needed
    ///
    /// # Returns
    /// * `Sprite2D` - The LineSelector node
    pub fn get_line_selector(&mut self) -> Gd<Sprite2D> {
        if self.line_selector.is_none() {
            self.line_selector = Some(self.base().get_node_as::<Sprite2D>("LineSelector"));
        }

        self.line_selector
            .as_ref()
            .expect("LineSelector node not found")
            .clone()
    }

    /// Getter for the PewterParticles node
    /// This effectively caches the PewterParticles node so that it does not have to be found every time it is needed
    ///
    /// # Returns
    /// * `GpuParticles2D` - The PewterParticles node
    pub fn get_pewter_particles(&mut self) -> Gd<GpuParticles2D> {
        if self.pewter_particles.is_none() {
            self.pewter_particles =
                Some(self.base().get_node_as::<GpuParticles2D>("PewterParticles"));
        }

        self.pewter_particles
            .as_ref()
            .expect("PewterParticles node not found")
            .clone()
    }

    /// Getter for the SteelParticles node
    /// This effectively caches the SteelParticles node so that it does not have to be found every time it is needed
    ///
    /// # Returns
    /// * `GpuParticles2D` - The SteelParticles node
    pub fn get_steel_particles(&mut self) -> Gd<GpuParticles2D> {
        if self.steel_particles.is_none() {
            self.steel_particles =
                Some(self.base().get_node_as::<GpuParticles2D>("SteelParticles"));
        }

        self.steel_particles
            .as_ref()
            .expect("SteelParticles node not found")
            .clone()
    }

    /// Getter for the Disconnected node
    /// This effectively caches the Disconnected node so that it does not have to be found every time it is needed
    ///
    /// # Returns
    /// * `Disconnected` - The Disconnected node
    pub fn get_disconnected(&mut self) -> Gd<Disconnected> {
        if self.disconnected.is_none() {
            self.disconnected = Some(self.base().get_node_as::<Disconnected>("Disconnected"));
        }

        self.disconnected
            .as_ref()
            .expect("Disconnected node not found")
            .clone()
    }
}
