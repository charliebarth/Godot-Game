//! player.rs
//!
//! The Player class is a character that can move and jump around the game world.
//! The player has a health bar, a coin counter, and can interact with other players and
//! metal objects in multiple ways, like dealing damage, getting damaged, and more.
//! The player can also be in different states, such as idle, running, jumping, and falling.
//!
//! Author: Michael Imerman, Trinity Pittman, Charles Barth
//! Version: Spring 2025
use std::collections::HashMap;
use std::collections::VecDeque;
use std::time::Instant;

use godot::classes::Camera2D;
use godot::classes::CanvasItem;
use godot::classes::CharacterBody2D;
use godot::classes::ConfigFile;
use godot::classes::Control;
use godot::classes::Engine;
use godot::classes::GpuParticles2D;
use godot::classes::ICharacterBody2D;
use godot::classes::Input;
use godot::classes::Label;
use godot::classes::PointLight2D;
use godot::classes::RayCast2D;
use godot::classes::Sprite2D;
use godot::classes::SubViewport;
use godot::classes::TextureProgressBar;
use godot::classes::{AnimatedSprite2D, Area2D};
use godot::prelude::*;

use crate::game::Game;
use crate::metal_object::MetalObject;
use crate::player::enums::metal_type::MetalType;
use crate::player::player_tin_light::PlayerTinLight;
use crate::settings::Settings;
use crate::ui::metal_reserve_bar_manager::MetalReserveBarManager;

use super::disconnected::Disconnected;
use super::enums::force::Force;
use super::enums::force::ForceModifier;
use super::enums::force::ForceModifierTag;
use super::enums::player_events::PlayerEvents;
use super::enums::player_states::PlayerStates;
use super::enums::timeout_events::TimeoutEvents;
use super::input_manager::InputManager;
use super::metal_line::MetalLine;
use super::metal_manager::MetalManager;
use crate::ui::coin_counter::CoinCounter;

/// The maximum amount of health the player can have
const MAX_HEALTH: f64 = 100.0;
/// The minimum amount of health the player can have
const MIN_HEALTH: f64 = 0.0;
/// The default run speed of the player
const DEFAULT_RUN_SPEED: f32 = 250.0;
/// The default jump force of the player
const DEFAULT_JUMP_FORCE: f32 = 450.0;
/// The maximum run speed of the player
const MAX_RUN_SPEED: f32 = 600.0;
/// The minimum run speed of the player
const MIN_RUN_SPEED: f32 = 0.0;
/// The maximum jump force of the player
const MAX_JUMP_FORCE: f32 = 700.0;
/// The minimum jump force of the player
const MIN_JUMP_FORCE: f32 = 300.0;

// Add an enum to identify different node types
#[derive(Hash, Eq, PartialEq, Clone, Debug, Copy)]
enum CachedNode {
    HealthBar,
    CoinCounter,
    SteelParticles,
    Disconnected,
    Camera,
    PewterParticles,
    TinParticles,
    BronzeParticles,
    SteelLines,
    IronLines,
    LineSelector,
    MetalReserveBarManager,
    InputManager,
    MetalManager,
    Sprite,
    IronParticles,
    CopperParticles,
}

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    /// The base node of the player
    base: Base<CharacterBody2D>,
    /// The direction the player is facing
    direction: f32,
    /// The gravity of the player
    gravity: f64,
    /// The default gravity (Comes from the Settings singleton)
    default_gravity: f64,
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
    timeout_events: HashMap<TimeoutEvents, Instant>,
    /// A queue of forces to be applied to the player
    forces: VecDeque<Force>,
    /// A vec of nearby metal objects that can be used by steel and iron
    metal_objects: Vec<Gd<MetalObject>>,
    /// A vec of nearby players that is used for copper and bronze functionality
    nearby_players: Vec<Gd<Player>>,
    /// A vec of the player's currently actively burning metals
    active_metals: Vec<MetalType>,
    /// The player's current particles
    current_particles: Option<Gd<GpuParticles2D>>,
    /// The mass of the player in kilograms
    mass: f32,
    /// If the player is attacking or not
    is_attacking: bool,
    /// HashMap storing cached node references
    cached_nodes: HashMap<CachedNode, Gd<Node>>,
    /// The settings for the game
    settings: Gd<Settings>,
    /// The number of eliminations the player has
    eliminations: i32,
    /// The previous velocity of the player
    previous_velocity: Vector2,
    /// This is collection of modifier meant to be applied to forces before they're applied
    /// to the player
    force_modifiers: HashMap<ForceModifierTag, ForceModifier>,
    /// The previous serialized state of the player
    previous_serialization: HashMap<String, String>,
    /// The number of physics frames that have passed since the last serialization
    physics_frame_count: i32,
    /// Serialized data from the server to be applied to the player at the start of the next physics frame
    data_to_apply: Option<Dictionary>,
    /// The horizontal force of the player based on input from the input manager
    horizontal_force: f32,
    /// Whether the player is a remote ghost player or not
    remote_player: bool,
}

#[godot_api]
impl ICharacterBody2D for Player {
    /// The Godot contractor for the Player class node
    ///
    /// # Arguments
    /// * `base` - The base node type for the player
    ///
    /// # Returns
    /// * `Player` - The Player node
    fn init(base: Base<CharacterBody2D>) -> Self {
        let settings = Engine::singleton()
            .get_singleton("Settings")
            .expect("settings singleton missing")
            .try_cast::<Settings>()
            .expect("settings is not a Settings");

        let settings_bound = settings.bind();
        let gravity: f64 = settings_bound.get_gravity() as f64;
        drop(settings_bound);

        Self {
            base,
            direction: 1.0,
            health: MAX_HEALTH,
            delta: 0.0,
            default_gravity: gravity,
            gravity,
            current_state: PlayerStates::Jump,
            previous_state: PlayerStates::Fall,
            anim_finished: false,
            run_speed: DEFAULT_RUN_SPEED,
            jump_force: DEFAULT_JUMP_FORCE,
            device_id: 0,
            player_id: 0,
            timeout_events: HashMap::new(),
            forces: VecDeque::new(),
            metal_objects: Vec::new(),
            nearby_players: Vec::new(),
            active_metals: Vec::new(),
            current_particles: None,
            mass: 500.0,
            is_attacking: false,
            cached_nodes: HashMap::new(),
            settings,
            eliminations: 0,
            previous_velocity: Vector2::ZERO,
            force_modifiers: HashMap::new(),
            previous_serialization: HashMap::new(),
            physics_frame_count: 0,
            data_to_apply: None,
            horizontal_force: 0.0,
            remote_player: false,
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

        // Give the metal manager access to the player
        self.get_metal_manager()
            .bind_mut()
            .set_player(self.base().get_node_as::<Player>("."));

        // Connect the tin signal to the player
        let tin_light = self.base().get_node_as::<PlayerTinLight>("PlayerTinLight");
        self.base_mut().connect(
            "tin_activated",
            &Callable::from_object_method(&tin_light, "adjust_tin_light"),
        );
        // Set the UI size
        let mut player_ui = self.base().get_node_as::<Control>("PlayerUI");

        let mut config = ConfigFile::new_gd();
        let err = config.load("user://settings.ini"); // TODO Check Error

        // Get the UI settings
        let size = config
            .get_value("ui", "size")
            .to_string()
            .parse::<f32>()
            .expect("Failed to parse to f32");
        let opacity = config
            .get_value("ui", "opacity")
            .to_string()
            .parse::<f32>()
            .expect("Failed to parse to f32");
        let pos_i = config
            .get_value("ui", "pos")
            .to_string()
            .parse::<f32>()
            .expect("Failed to parse to f32");

        // Set the scale (size) of the UI elements
        player_ui.set_scale(Vector2::new(size, size));

        // Set the opacity of the UI elements
        let mut color = player_ui.get_modulate();
        color.a = opacity;
        player_ui.set_modulate(color);

        // Set the position of the UI elements
        let positions = [
            Vector2::new(-479., -269.), // Top Left
            Vector2::new(-62., -269.),  // Top Center
            Vector2::new(355., -269.),  // Top Right
            Vector2::new(-479., 201.),  // Bottom Left
            Vector2::new(-62., 201.),   // Bottom Center
            Vector2::new(355., 201.),   // Bottom Right
        ];
        let pivot_offset = [
            Vector2::new(0., 0.),    // Top Left
            Vector2::new(62., 0.),   // Top Center
            Vector2::new(124., -0.), // Top Right
            Vector2::new(0., 70.),   // Bottom Left
            Vector2::new(62., 70.),  // Bottom Center
            Vector2::new(124., 70.), // Bottom Right
        ];

        player_ui.set_global_position(self.base().to_local(positions[pos_i as usize]));
        player_ui.set_pivot_offset(pivot_offset[pos_i as usize]);
    }

    /// The Godot method called every physics frame
    /// Physics frames happen a static number of times per second as opposed to process frames which happen as often as possible
    ///
    /// # Arguments
    /// * `delta` - The time since the last frame
    fn physics_process(&mut self, delta: f64) {
        let is_server = self.base().get_multiplayer().unwrap().is_server();
        let online = self.settings.bind().get_online_multiplayer();

        if online && !is_server {
            if let Some(data) = &self.data_to_apply {
                let data = self.dictionary_to_hashmap(data);
                self.forces.clear(); // Clear the forces queue to ensure no data conflicts
                self.deserialize(data);
                self.data_to_apply = None;
            }
        }

        if self.health <= 0.0 {
            self.die();
        }

        // Reset the player to their default values such as animation speed, run speed, and jump force
        self.reset_player();

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

        // Update the current state of the player
        self.current_state.update_state(self);
        self.set_animation_direction();

        // Check for any timeout events that have expired
        self.expire_timeout_events();

        self.add_force(Force::Gravity {
            acceleration: self.gravity,
        });

        if self.base().is_on_floor() {
            self.add_force(Force::NormalForce { magnitude: -1.0 });
        }

        // Make the player move and slide based on their velocity
        self.horizontal_force = self.get_input_manager().bind().get_left_right_value();
        self.apply_forces();
        self.base_mut().move_and_slide();

        if online && is_server {
            self.physics_frame_count += 1;
            if self.physics_frame_count % 3 == 0 {
                let serialization = self.serialize();
                let mut game = self.base().get_node_as::<Game>("/root/Game");
                game.call(
                    "add_serialization",
                    &[self.hashmap_to_dictionary(serialization).to_variant()],
                );

                self.physics_frame_count = 0;
            }
        }
    }
}

#[godot_api]
impl Player {
    /// Sets whether the player is a remote player.
    /// If the player is a remote player, this will remove unnecessary nodes to
    /// increase performance.
    ///
    /// # Arguments
    /// * `remote_player` - Whether the player is a remote player
    pub fn set_remote_player(&mut self, remote_player: bool) {
        self.remote_player = remote_player;

        if self.remote_player {
            self.get_camera().queue_free();
            self.base()
                .get_node_as::<PointLight2D>("PointLight2D")
                .queue_free();
            self.base()
                .get_node_as::<PointLight2D>("PointLight2D2")
                .queue_free();
            self.base()
                .get_node_as::<Sprite2D>("LineSelector")
                .queue_free();
            self.base().get_node_as::<Label>("FpsCounter").queue_free();
        }
    }

    /// Returns whether the player is a remote player.
    ///
    /// # Returns
    /// * `bool` - Whether the player is a remote player
    #[func]
    pub fn is_remote_player(&self) -> bool {
        self.remote_player
    }

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
        let mut game = self.base().get_node_as::<Game>("/root/Game").clone();
        game.call_deferred(
            "remove_player",
            &[
                Variant::from(self.player_id),
                Variant::from(self.eliminations),
            ],
        );
    }

    /// Makes a given player visible to the current player
    ///
    /// # Arguments
    /// * `player_id` - The ID of the player to make visible
    #[func]
    pub fn make_player_visible(&mut self, player_id: i32) {
        let mut player_sprite = self.get_sprite();
        let current_layer = player_sprite.get_visibility_layer();
        player_sprite.set_visibility_layer(current_layer | 1 << (player_id * 2));
    }

    /// Makes a given player invisible to the current player
    ///
    /// # Arguments
    /// * `player_id` - The ID of the player to make invisible
    #[func]
    pub fn make_player_invisible(&mut self, player_id: i32) {
        let mut player_sprite = self.get_sprite();
        let current_layer = player_sprite.get_visibility_layer();
        player_sprite.set_visibility_layer(current_layer & !(1 << (player_id * 2)));
    }

    #[func]
    /// Set the zoom for the player
    ///
    /// # Arguments
    /// * `zoom` - The zoom to set the player to
    pub fn set_zoom(&mut self, zoom: Vector2) {
        if self.remote_player {
            return;
        }

        self.get_camera().set_zoom(zoom);
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

    #[func]
    /// Get the health of the player
    ///
    /// # Returns
    /// * `f64` - The health of the player
    pub fn get_health(&self) -> f64 {
        self.health
    }

    #[func]
    /// Get the eliminations of the player
    ///
    /// # Returns
    /// * `i32` - The eliminations of the player
    pub fn get_eliminations(&self) -> i32 {
        self.eliminations
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
    pub fn adjust_health(&mut self, mut adjustment: f64) {
        if adjustment.signum() == -1.0 && self.is_burning_metal(MetalType::Pewter) {
            if adjustment <= MAX_HEALTH * 0.05 {
                adjustment = 0.0;
            } else if adjustment <= MAX_HEALTH * 0.25 {
                adjustment = adjustment * 0.5;
            } else if adjustment <= MAX_HEALTH * 0.50 {
                adjustment = adjustment * 0.7;
            } else {
                adjustment = adjustment * 0.9;
            }
        }

        // Adjust health by the specified amount
        self.health += adjustment;

        // Clamp health between MIN_HEALTH and MAX_HEALTH
        self.health = self.health.clamp(MIN_HEALTH, MAX_HEALTH);

        // Update the health bar of the player
        self.get_health_bar().set_value(self.get_health());
    }

    #[func]
    /// Adjust the eliminations of the player
    ///
    /// # Arguments
    /// * `attacker_id` - The id of the player who eliminated this player
    pub fn increment_eliminations(&mut self, attacker_id: i32) {
        self.eliminations += 1;
        // update the eliminations counter for a player in game
        self.base()
            .get_node_as::<Game>("/root/Game")
            .bind_mut()
            .update_eliminations(attacker_id);
    }

    /// Adjusts the coins in this players coin_counter positively or negatively.
    ///
    /// # Arguments
    /// * `pos_neg` (i8) - if -1, remove_coin    if +1, add_coin
    pub fn adjust_coins(&mut self, pos_neg: i8, coin: Gd<MetalObject>) {
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
    pub fn adjust_metals(&mut self, metals: Vec<&str>, amt: f64) {
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
    pub fn set_metal_reserve_amount(&mut self, metal: &str, amt: f64) {
        self.get_metal_reserve_bar_manager()
            .bind_mut()
            .set_metal_amount(metal, amt);
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

    /// Gets the default gravity of the game
    /// This value originally comes from the Settings singleton and this is just a cached version
    ///
    /// # Returns
    /// * `f64` - The default gravity of the game
    pub fn get_default_gravity(&self) -> f64 {
        self.default_gravity
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
    }

    /// Set the animation direction of the player
    /// This method sets the direction of the player's sprite based on the direction the player is facing
    /// This also changes the position of the sprite to ensure it is centered in the player's hitbox
    fn set_animation_direction(&mut self) {
        let mut sprite = self.get_sprite();
        let mut scale = sprite.get_scale();

        if self.direction < 0.0 && scale.x != -1.3 {
            scale.x = -1.3;
        } else if self.direction > 0.0 && scale.x != 1.3 {
            scale.x = 1.3;
        }

        sprite.set_scale(scale);
    }

    /// Sets the speed of the player's animations
    ///
    /// # Arguments
    /// * `speed` - The speed to set the player's animations to
    pub fn set_animation_speed(&mut self, speed: f32) {
        let mut sprite = self.get_sprite();
        sprite.set_speed_scale(speed);
    }

    /// Checks to see if a metal object is in range of the player
    ///
    /// # Arguments
    /// * `metal_object` - The metal object to check
    ///
    /// # Returns
    /// * `bool` - True if the metal object is in range, false otherwise
    pub fn is_metal_object_in_range(&self, metal_object: &Gd<MetalObject>) -> bool {
        self.metal_objects.contains(&metal_object)
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
    #[func]
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
        self.timeout_events.insert(event, Instant::now());
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
        self.timeout_events.retain(|event, start_time| {
            let time_elapsed = Instant::now().duration_since(*start_time);
            time_elapsed <= event.get_duration()
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
                let light_mask = node.get_light_mask();
                node.set_light_mask(light_mask | 1 << layer_num);
            }
        }

        let mut sprite = self.get_sprite();
        let sprite_layer = sprite.get_visibility_layer();
        sprite.set_visibility_layer(sprite_layer | 1);

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
        self.set_gravity(self.default_gravity);
    }

    /// Adds a force to the player's forces queue
    ///
    /// # Arguments
    /// * `force` - The force to add to the player's forces queue
    pub fn add_force(&mut self, force: Force) {
        self.forces.push_back(force);
    }

    /// Adds a face modifier to the player's hashmap
    /// These modifiers are not automatically removed by the player.
    ///
    /// If the modifier already exists in the set a default merging of the current modifier and
    /// the provided modifier will be attempted. This behavior comes from the combine_modifiers function
    /// in the ForceModifier implementation
    ///
    ///  Note: Modifiers MUST BE REMOVED MANUALLY
    ///
    ///
    /// # Arguments
    /// * `modifier_value` - The force modifier to add the player's hashmap
    pub fn add_force_modifer(&mut self, mut modifier: ForceModifier) {
        let tag = modifier.tag();

        if let Some(current_modifier) = self.force_modifiers.get(&tag) {
            modifier = current_modifier.combine_modifiers(modifier);
        }

        self.force_modifiers.insert(tag, modifier);
    }

    /// Removes a force modifier from the player's hashmap
    ///
    /// # Arguments
    /// * `modifier` - The force modifier to remove from the player's hashmap
    pub fn remove_force_modifier(&mut self, modifier: ForceModifierTag) {
        self.force_modifiers.remove(&modifier);
    }

    /// Replaces a force modifier in the player's hashmap
    ///
    /// # Arguments
    /// * `modifier` - The force modifier to replace in the player's hashmap
    pub fn replace_force_modifier(&mut self, modifier: ForceModifier) {
        self.force_modifiers.insert(modifier.tag(), modifier);
    }

    /// This iterates through the forces queue and applies each force to the player
    fn apply_forces(&mut self) {
        let len_forces = self.forces.len();
        for _ in 0..len_forces {
            let force = self.forces.pop_front().unwrap();
            self.apply_force(force);
        }

        let base_velocity = self.base().get_velocity();
        if base_velocity != Vector2::ZERO {
            self.previous_velocity = base_velocity;
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
                let mut multiplier = 1.0;
                match self.force_modifiers.get(&ForceModifierTag::Pewter) {
                    Some(ForceModifier::Pewter { jump_boost, .. }) => {
                        multiplier += *jump_boost;
                    }
                    _ => {}
                }

                base_velocity.y += acceleration * multiplier as f32;
            }
            Force::Run { acceleration } => {
                let max_run_speed = self.get_run_speed();
                let mut multiplier = 1.0;
                match self.force_modifiers.get(&ForceModifierTag::Pewter) {
                    Some(ForceModifier::Pewter { run_boost, .. }) => {
                        multiplier += *run_boost;
                    }
                    _ => {}
                }

                if base_velocity.x.signum() != max_run_speed.signum()
                    || base_velocity.x.abs() < max_run_speed
                {
                    base_velocity.x += acceleration * (self.delta * multiplier) as f32;
                }

                if self.horizontal_force.abs() != 1.0
                    || acceleration.signum() != base_velocity.x.signum()
                {
                    base_velocity.x = base_velocity.x.clamp(-max_run_speed, max_run_speed);
                }
            }
            Force::AirRun { acceleration } => {
                let max_run_speed = self.get_run_speed();
                let mut multiplier = 1.0;
                match self.force_modifiers.get(&ForceModifierTag::Pewter) {
                    Some(ForceModifier::Pewter { run_boost, .. }) => {
                        multiplier += *run_boost;
                    }
                    _ => {}
                }

                if base_velocity.x.signum() != max_run_speed.signum()
                    || base_velocity.x.abs() < max_run_speed
                {
                    base_velocity.x += acceleration * (self.delta * multiplier) as f32;
                }

                if self.horizontal_force.abs() != 1.0
                    || acceleration.signum() != base_velocity.x.signum()
                {
                    base_velocity.x = base_velocity.x.clamp(-max_run_speed, max_run_speed);
                }
            }
            Force::Stop {
                horizontal,
                vertical,
            } => {
                base_velocity.x = if horizontal { 0.0 } else { base_velocity.x };
                base_velocity.y = if vertical { 0.0 } else { base_velocity.y };
                self.previous_velocity = Vector2::ZERO;
            }
            Force::SteelPush {
                x_acceleration,
                y_acceleration,
            } => {
                base_velocity.x = x_acceleration;
                base_velocity.y = y_acceleration;
            }
            _ => {}
        }

        self.base_mut().set_velocity(base_velocity);
    }

    /// This is called when an object impacts the player.
    /// It will calculate if the player should be damaged and if they should be moved.
    /// It returns an impact force which is how much force is returned to the object.
    ///
    /// # Arguments
    /// * `impact_force` - The force of the impact which is roughly calulated using the speed of the object and its weight
    ///
    /// # Returns
    /// * `Force` - A Force::Impact which is how much energy/force is returned to the object,
    /// again roughly calculated using the speed of the player and their weight.
    // pub fn impact(&mut self, impact_force: Force) -> Force {
    //     Force::NormalForce { magnitude: -1.0 }
    // }
    pub fn impact(&mut self, body_mass: f32, body_velocity: Vector2) -> Vector2 {
        let player_velocity = self.previous_velocity;

        // Compute new velocities using momentum equations
        let mut new_player_velocity = ((self.mass - body_mass) * player_velocity
            + 2.0 * body_mass * body_velocity)
            / (self.mass + body_mass);

        let mut new_body_velocity = ((body_mass - self.mass) * body_velocity
            + 5.0 * self.mass * player_velocity)
            / (self.mass + body_mass);

        // Prevent slowing down in the same direction for the player
        if new_player_velocity.dot(player_velocity) > 0.0 {
            if new_player_velocity.length() < player_velocity.length() {
                new_player_velocity = player_velocity;
            }
        }

        // Prevent slowing down in the same direction for the body
        if new_body_velocity.dot(body_velocity) > 0.0 {
            if new_body_velocity.length() < body_velocity.length() {
                new_body_velocity = body_velocity;
            }
        }

        self.base_mut().set_velocity(new_player_velocity);

        // Return the new velocity for the body
        new_body_velocity
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
        if !metal.is_freeze_enabled() {
            self.metal_objects.push(metal);
        }
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

    #[func]
    /// Adds a player to the player's vec of nearby players
    ///
    /// # Arguments
    /// * `player` - The player to add to the current player's vector of nearby players
    fn add_nearby_player(&mut self, player: Gd<Player>) {
        self.nearby_players.push(player);
    }

    #[func]
    /// Removes a player from the player's vec of nearby players
    ///
    /// # Arguments
    /// * `player` - The player to remove from the current player's vector of nearby players
    fn remove_nearby_player(&mut self, player: Gd<Player>) {
        if let Some(pos) = self.nearby_players.iter().position(|x| *x == player) {
            let mut player = self.nearby_players.remove(pos);
            let visibility_mask = 1 << player.bind().get_player_id() * 2;
            //player.bind_mut().hide_particles(visibility_mask);
        }
    }

    /// Gets the vec of all nearby players
    ///
    /// # Returns
    /// * `Vec<Gd<Player>>` - The vec of all nearby players
    pub fn get_nearby_players(&mut self) -> &mut Vec<Gd<Player>> {
        &mut self.nearby_players
    }

    /// Reveals the particles of the player if the player is not burning copper
    ///
    /// # Arguments
    /// * `visibility_layer` - The visibility layer to set for the particles
    pub fn reveal_particles(&mut self, visibility_layer: u32) {
        let current_layer = 1 << (self.player_id * 2);
        if !self.is_burning_metal(MetalType::Copper) {
            for metal in MetalType::iter() {
                let mut particles = self.get_metal_particles(metal);
                if particles.is_visible_in_tree() {
                    particles.set_visibility_layer(current_layer | visibility_layer);
                }
            }
        }
    }

    /// Hides the particles (that indicate a player is burning a certain metal) of the player
    /// from other players.
    ///
    /// # Arguments
    /// * `visibility_layer` - A given player's visibility layer
    pub fn hide_particles(&mut self, visibility_layer: u32) {
        godot_print!("Hiding particles for player: {}", self.player_id);
        let mask_to_clear = !(visibility_layer); // Bitwise NOT to clear the target bit
        for metal in MetalType::iter() {
            let mut particles = self.get_metal_particles(metal);
            let current_mask = particles.get_visibility_layer();
            particles.set_visibility_layer(current_mask & mask_to_clear);
        }
    }

    /// Adds a metal to the active burning metals
    ///
    /// # Arguments
    /// * `metal` - The metal to add to the active burning metals
    pub fn add_active_metal(&mut self, metal: MetalType) {
        if !self.active_metals.contains(&metal) {
            self.active_metals.push(metal);
            if metal == MetalType::Copper {
                // get the visibility mask of the current player
                let visibility_mask = 1 << self.player_id * 2;
                self.hide_particles(visibility_mask);
            }
        }
    }

    /// Removes a metal from the active burning metals
    ///
    /// # Arguments
    /// * `metal` - The metal to remove from the active burning metals
    pub fn remove_active_metal(&mut self, metal: MetalType) {
        if let Some(pos) = self.active_metals.iter().position(|x| *x == metal) {
            self.active_metals.remove(pos);
        }
    }

    /// Checks if the player is burning a specific metal
    ///
    /// # Arguments
    /// * `metal` - The metal to check if the player is burning
    ///
    /// # Returns
    /// * `bool` - True if the player is burning the metal, false otherwise
    pub fn is_burning_metal(&mut self, metal: MetalType) -> bool {
        let particles = self.get_metal_particles(metal);
        particles.is_visible_in_tree()
    }

    /// Returns a boolean indicating if the player is burning a specific metal from a string
    ///
    /// # Arguments
    /// * `metal` - The metal to check if the player is burning
    ///
    /// # Returns
    /// * `bool` - True if the player is burning the metal, false otherwise
    #[func]
    pub fn is_burning_metal_from_string(&mut self, metal: String) -> bool {
        if let Some(metal) = MetalType::from_string(&metal) {
            let particles = self.get_metal_particles(metal);
            return particles.is_visible_in_tree();
        } else {
            return false;
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

    /// Enable the hitbox of the player when they are attacking
    ///
    /// # Arguments
    /// * `owner` - A reference to the node for the hitbox of the player
    pub fn enable_hitbox(&mut self) {
        self.is_attacking = true;

        // Get the hitbox of the player
        let mut right_hitbox = self.base().get_node_as::<Area2D>("RightHitbox");
        let mut left_hitbox = self.base().get_node_as::<Area2D>("LeftHitbox");

        // Enable the hitbox of the player depending on the direction they are facing
        if self.direction > 0.0 {
            right_hitbox.set_monitoring(true);
            right_hitbox.set_collision_layer(1 << 2);
            left_hitbox.set_monitoring(false);
            left_hitbox.set_collision_layer(1 << 3);
        } else {
            left_hitbox.set_monitoring(true);
            left_hitbox.set_collision_layer(1 << 2);
            right_hitbox.set_monitoring(false);
            right_hitbox.set_collision_layer(1 << 3);
        }
    }

    /// Disable the hitbox of the player when they are not attacking
    ///
    /// # Arguments
    /// * `owner` - A reference to the node for the hitbox of the player
    pub fn disable_hitbox(&mut self) {
        self.is_attacking = false;
        // Get the hitbox of the player
        let mut right_hitbox = self.base().get_node_as::<Area2D>("RightHitbox");
        let mut left_hitbox = self.base().get_node_as::<Area2D>("LeftHitbox");

        // Disable the hitboxes of the player
        right_hitbox.set_monitoring(false);
        right_hitbox.set_collision_layer(1 << 3);
        left_hitbox.set_monitoring(false);
        left_hitbox.set_collision_layer(1 << 3);
    }

    /// Emit a signal to adjust the light for the player when they use tin
    ///
    /// # Arguments
    /// * `light_level` - The target light level.
    /// * `transition_time` - The time it takes to transition to the target light level.
    pub fn emit_tin_signal(&mut self, light_level: f32, transition_time: f64) {
        self.base_mut().emit_signal(
            "tin_activated",
            &[Variant::from(light_level), Variant::from(transition_time)],
        );
    }

    /// A signal that is emitted by the player when it's id is changed
    /// Children of the player can listen for the signal and then change their visibility layer based on the new id
    #[signal]
    fn id_changed();

    /// A signal that is emitted by the player when it is using tin
    #[signal]
    pub fn tin_activated(light_level: f32, transition_time: f64);

    /// If passed true, the player turns on its timer to count down before the player is removed from the game
    /// If passed false, the player turns off its timer meaning it is no longer disconnected
    ///
    /// # Arguments
    /// * `disconnected` - A boolean that determines if the player is disconnected or not
    pub fn set_disconnected(&mut self, disconnected: bool) {
        let mut disconnected_node = self.get_disconnected();
        disconnected_node.set_visible(disconnected);
    }

    /// Getter for the InputManager node
    /// This effectively caches the InputManager node so that it does not have to be found every time it is needed
    ///
    /// # Returns
    /// * `InputManager` - The InputManager node
    #[func]
    pub fn get_input_manager(&mut self) -> Gd<InputManager> {
        self.get_cached_node(CachedNode::InputManager, "InputManager")
    }

    /// Serializes the player's state
    /// This serialized data will only include data that has changed since the last serialization.
    /// It will be sent to all the clients via RPC so they remain in sync.
    ///
    /// # Returns
    /// * `HashMap<String, String>` - The serialized state of the player
    pub fn serialize(&mut self) -> HashMap<String, String> {
        let mut serialization = HashMap::new();

        // Basic player state
        serialization.insert("player_id".to_string(), self.player_id.to_string());

        if self
            .previous_serialization
            .get("health")
            .map_or(true, |v| v != &self.health.to_string())
        {
            serialization.insert("health".to_string(), self.health.to_string());
        }
        if self
            .previous_serialization
            .get("direction")
            .map_or(true, |v| v != &self.direction.to_string())
        {
            serialization.insert("direction".to_string(), self.direction.to_string());
        }
        if self
            .previous_serialization
            .get("run_speed")
            .map_or(true, |v| v != &self.run_speed.to_string())
        {
            serialization.insert("run_speed".to_string(), self.run_speed.to_string());
        }
        if self
            .previous_serialization
            .get("jump_force")
            .map_or(true, |v| v != &self.jump_force.to_string())
        {
            serialization.insert("jump_force".to_string(), self.jump_force.to_string());
        }
        if self
            .previous_serialization
            .get("mass")
            .map_or(true, |v| v != &self.mass.to_string())
        {
            serialization.insert("mass".to_string(), self.mass.to_string());
        }
        if self
            .previous_serialization
            .get("is_attacking")
            .map_or(true, |v| v != &self.is_attacking.to_string())
        {
            serialization.insert("is_attacking".to_string(), self.is_attacking.to_string());
        }
        if self
            .previous_serialization
            .get("device_id")
            .map_or(true, |v| v != &self.device_id.to_string())
        {
            serialization.insert("device_id".to_string(), self.device_id.to_string());
        }

        // Current state
        if self
            .previous_serialization
            .get("current_state")
            .map_or(true, |v| v != &self.current_state.serialize())
        {
            serialization.insert("current_state".to_string(), self.current_state.serialize());
        }
        if self
            .previous_serialization
            .get("previous_state")
            .map_or(true, |v| v != &self.previous_state.serialize())
        {
            serialization.insert(
                "previous_state".to_string(),
                self.previous_state.serialize(),
            );
        }

        // Position and velocity
        let base = self.base();
        let position = base.get_position();
        let velocity = base.get_velocity();

        if self
            .previous_serialization
            .get("position_x")
            .map_or(true, |v| v != &position.x.to_string())
        {
            serialization.insert("position_x".to_string(), position.x.to_string());
        }
        if self
            .previous_serialization
            .get("position_y")
            .map_or(true, |v| v != &position.y.to_string())
        {
            serialization.insert("position_y".to_string(), position.y.to_string());
        }
        if self
            .previous_serialization
            .get("velocity_x")
            .map_or(true, |v| v != &velocity.x.to_string())
        {
            serialization.insert("velocity_x".to_string(), velocity.x.to_string());
        }
        if self
            .previous_serialization
            .get("velocity_y")
            .map_or(true, |v| v != &velocity.y.to_string())
        {
            serialization.insert("velocity_y".to_string(), velocity.y.to_string());
        }

        // Metal objects count
        let metal_objects_count = self.metal_objects.len().to_string();
        if self
            .previous_serialization
            .get("metal_objects_count")
            .map_or(true, |v| v != &metal_objects_count)
        {
            serialization.insert("metal_objects_count".to_string(), metal_objects_count);
        }

        // Timeout events
        for (event, start_time) in &self.timeout_events {
            let event_key = event.serialize();
            let elapsed = start_time.elapsed().as_secs().to_string();
            if self
                .previous_serialization
                .get(&event_key)
                .map_or(true, |v| v != &elapsed)
            {
                serialization.insert(event_key, elapsed);
            }
        }

        // Update the previous serialization with the new values
        for (key, value) in &serialization {
            self.previous_serialization
                .insert(key.clone(), value.clone());
        }

        serialization
    }

    /// Converts a HashMap to a Dictionary
    ///
    /// # Arguments
    /// * `data` - The HashMap to convert to a Dictionary
    ///
    /// # Returns
    /// * `Dictionary` - The Dictionary containing the player data
    pub fn hashmap_to_dictionary(&self, data: HashMap<String, String>) -> Dictionary {
        let mut dictionary = Dictionary::new();
        for (key, value) in &data {
            dictionary.set(key.as_str(), value.as_str());
        }
        dictionary
    }

    /// Converts a Dictionary to a HashMap
    ///
    /// # Arguments
    /// * `data` - The Dictionary to convert to a HashMap
    ///
    /// # Returns
    /// * `HashMap<String, String>` - The HashMap containing the player data
    pub fn dictionary_to_hashmap(&self, data: &Dictionary) -> HashMap<String, String> {
        let mut hashmap = HashMap::new();
        for (key, value) in data.iter_shared() {
            hashmap.insert(key.to_string(), value.to_string());
        }
        hashmap
    }

    /// Adds server data to the player
    /// This method should only be called on the client side to update player state from server data
    ///
    /// # Arguments
    /// * `data` - The Dictionary containing the player data to apply
    pub fn add_server_data(&mut self, data: Dictionary) {
        self.data_to_apply = Some(data);
    }

    /// Deserializes player data from a HashMap and applies it to the player
    /// This method should only be called on the client side to update player state from server data
    ///
    /// # Arguments
    /// * `data` - The HashMap containing the player data to apply
    pub fn deserialize(&mut self, data: HashMap<String, String>) {
        // Basic player state
        if let Some(health) = data.get("health").and_then(|v| v.parse::<f64>().ok()) {
            self.health = health;
            self.get_health_bar().set_value(health);
        }
        if let Some(direction) = data.get("direction").and_then(|v| v.parse::<f32>().ok()) {
            self.direction = direction;
        }
        if let Some(run_speed) = data.get("run_speed").and_then(|v| v.parse::<f32>().ok()) {
            self.run_speed = run_speed;
        }
        if let Some(jump_force) = data.get("jump_force").and_then(|v| v.parse::<f32>().ok()) {
            self.jump_force = jump_force;
        }
        if let Some(mass) = data.get("mass").and_then(|v| v.parse::<f32>().ok()) {
            self.mass = mass;
        }
        if let Some(is_attacking) = data
            .get("is_attacking")
            .and_then(|v| v.parse::<bool>().ok())
        {
            self.is_attacking = is_attacking;
            if is_attacking {
                self.enable_hitbox();
            } else {
                self.disable_hitbox();
            }
        }
        if let Some(player_id) = data.get("player_id").and_then(|v| v.parse::<i32>().ok()) {
            self.player_id = player_id;
        }
        if let Some(device_id) = data.get("device_id").and_then(|v| v.parse::<i32>().ok()) {
            self.device_id = device_id;
        }

        // Current state
        if let Some(current_state) = data.get("current_state") {
            if let Some(state) = PlayerStates::deserialize(current_state) {
                self.current_state = state;
            }
        }
        if let Some(previous_state) = data.get("previous_state") {
            if let Some(state) = PlayerStates::deserialize(previous_state) {
                self.previous_state = state;
            }
        }

        // Position and velocity
        let mut base = self.base_mut();
        let mut position = base.get_position();
        let mut velocity = base.get_velocity();

        if let Some(x) = data.get("position_x").and_then(|v| v.parse::<f32>().ok()) {
            position.x = x;
        }
        if let Some(y) = data.get("position_y").and_then(|v| v.parse::<f32>().ok()) {
            position.y = y;
        }
        if let Some(x) = data.get("velocity_x").and_then(|v| v.parse::<f32>().ok()) {
            velocity.x = x;
        }
        if let Some(y) = data.get("velocity_y").and_then(|v| v.parse::<f32>().ok()) {
            velocity.y = y;
        }

        base.set_position(position);
        base.set_velocity(velocity);
        drop(base);

        // Update animation based on new state
        self.set_animation_direction();
    }
}
/// Getters for nodes
impl Player {
    /// This enum is used to cache nodes in the player
    fn get_cached_node<T: GodotClass + Inherits<Node>>(
        &mut self,
        cache_key: CachedNode,
        path: &str,
    ) -> Gd<T> {
        if !self.cached_nodes.contains_key(&cache_key) {
            let node = self.base().get_node_as::<T>(path);
            self.cached_nodes.insert(cache_key, node.upcast());
        }

        self.cached_nodes
            .get(&cache_key)
            .expect(&format!("{:?} node not found", cache_key))
            .clone()
            .cast::<T>()
    }

    /// Getter for the MetalManager node
    /// This effectively caches the MetalManager node so that it does not have to be found every time it is needed
    ///
    /// # Returns
    /// * `MetalManager` - The MetalManager node
    pub fn get_metal_manager(&mut self) -> Gd<MetalManager> {
        self.get_cached_node(CachedNode::MetalManager, "MetalManager")
    }

    /// Getter for the AnimatedSprite2D node
    /// This effectively caches the AnimatedSprite2D node so that it does not have to be found every time it is needed
    ///
    /// # Returns
    /// * `AnimatedSprite2D` - The AnimatedSprite2D node
    pub fn get_sprite(&mut self) -> Gd<AnimatedSprite2D> {
        self.get_cached_node(CachedNode::Sprite, "PlayerAnimation")
    }

    /// Getter for the MetalReserveBarManager node
    /// This effectively caches the MetalReserveBarManager node so that it does not have to be found every time it is needed
    ///
    /// # Returns
    /// * `MetalReserveBarManager` - The MetalReserveBarManager node
    pub fn get_metal_reserve_bar_manager(&mut self) -> Gd<MetalReserveBarManager> {
        self.get_cached_node(
            CachedNode::MetalReserveBarManager,
            "PlayerUI/MetalReserveBarManager",
        )
    }

    /// Getter for the HealthBar node
    /// This effectively caches the HealthBar node so that it does not have to be found every time it is needed
    ///
    /// # Returns
    /// * `TextureProgressBar` - The TextureProgressBar node used to display the player's health
    pub fn get_health_bar(&mut self) -> Gd<TextureProgressBar> {
        self.get_cached_node(CachedNode::HealthBar, "PlayerUI/HealthBar")
    }

    /// Getter for CoinCounter node
    /// This effectively caches the CoinCounter node so that it does not have to be found every time it is needed
    ///
    /// # Returns
    /// *  `CoinCounter` - The CoinCounter node used to show player coins.
    pub fn get_coin_counter(&mut self) -> Gd<CoinCounter> {
        self.get_cached_node(
            CachedNode::CoinCounter,
            "PlayerUI/Coin_Counter_Panel/CoinCounter",
        )
    }

    /// Getter for the MetalLine node
    /// This effectively caches the MetalLine node so that it does not have to be found every time it is needed
    ///
    /// # Returns
    /// * `MetalLine` - The MetalLine node
    pub fn get_metal_line(&mut self, metal_type: MetalType) -> Gd<MetalLine> {
        let search_key = if metal_type == MetalType::Iron {
            (CachedNode::IronLines, "IronLines")
        } else {
            (CachedNode::SteelLines, "SteelLines")
        };

        self.get_cached_node(search_key.0, search_key.1)
    }

    /// Getter for the LineSelector node
    /// This effectively caches the LineSelector node so that it does not have to be found every time it is needed
    ///
    /// # Returns
    /// * `Sprite2D` - The LineSelector node
    pub fn get_line_selector(&mut self) -> Gd<Sprite2D> {
        self.get_cached_node(CachedNode::LineSelector, "LineSelector")
    }

    /// Getter for the PewterParticles node
    /// This effectively caches the PewterParticles node so that it does not have to be found every time it is needed
    ///
    /// # Returns
    /// * `GpuParticles2D` - The PewterParticles node
    pub fn get_pewter_particles(&mut self) -> Gd<GpuParticles2D> {
        self.get_cached_node(CachedNode::PewterParticles, "PewterParticles")
    }

    /// Getter for the SteelParticles node
    /// This effectively caches the SteelParticles node so that it does not have to be found every time it is needed
    ///
    /// # Returns
    /// * `GpuParticles2D` - The SteelParticles node
    pub fn get_steel_particles(&mut self) -> Gd<GpuParticles2D> {
        self.get_cached_node(CachedNode::SteelParticles, "SteelParticles")
    }

    /// Getter for the TinParticles node
    /// This effectively caches the TinParticles node so that it does not have to be found every time it is needed
    ///
    /// # Returns
    /// * `GpuParticles2D` - The TinParticles node
    pub fn get_tin_particles(&mut self) -> Gd<GpuParticles2D> {
        self.get_cached_node(CachedNode::TinParticles, "TinParticles")
    }

    /// Getter for the BronzeParticles node
    /// This effectively caches the BronzeParticles node so that it does not have to be found every time it is needed
    ///
    /// # Returns
    /// * `GpuParticles2D` - The BronzeParticles node
    pub fn get_bronze_particles(&mut self) -> Gd<GpuParticles2D> {
        self.get_cached_node(CachedNode::BronzeParticles, "BronzeParticles")
    }

    /// Getter for the IronParticles node
    /// This effectively caches the IronParticles node so that it does not have to be found every
    /// time it is needed.
    ///
    /// * `GpuParticles2D` - The IronParticles node
    pub fn get_iron_particles(&mut self) -> Gd<GpuParticles2D> {
        self.get_cached_node(CachedNode::IronParticles, "IronParticles")
    }

    /// Getter for the CopperParticles node
    /// This effectively caches the CopperParticles node so that it does not have to be found every
    /// time it is needed.
    ///
    /// # Returns
    /// * `GpuParticles2D` - The CopperParticles node
    pub fn get_copper_particles(&mut self) -> Gd<GpuParticles2D> {
        self.get_cached_node(CachedNode::CopperParticles, "CopperParticles")
    }

    /// Getter for the Disconnected node
    /// This effectively caches the Disconnected node so that it does not have to be found every
    /// time it is needed.
    ///
    /// # Returns
    /// * `Disconnected` - The Disconnected node
    pub fn get_disconnected(&mut self) -> Gd<Disconnected> {
        self.get_cached_node(CachedNode::Disconnected, "Disconnected")
    }

    /// Getter for the Camera2D node
    /// This effectively caches the Camera2D node so that it does not have to be found every
    /// time it is needed.
    ///
    /// # Returns
    /// * `Camera2D` - The Camera2D node
    pub fn get_camera(&mut self) -> Gd<Camera2D> {
        self.get_cached_node(CachedNode::Camera, "Camera2D")
    }

    /// Getter for the MetalParticles node
    ///
    /// # Arguments
    /// * `metal_type` - The type of metal to get the particles for
    ///
    /// # Returns
    /// * `GpuParticles2D` - The particles for the given metal type
    pub fn get_metal_particles(&mut self, metal_type: MetalType) -> Gd<GpuParticles2D> {
        match metal_type {
            MetalType::Pewter => self.get_pewter_particles(),
            MetalType::Steel => self.get_steel_particles(),
            MetalType::Iron => self.get_iron_particles(),
            MetalType::Bronze => self.get_bronze_particles(),
            MetalType::Copper => self.get_copper_particles(),
            MetalType::Tin => self.get_tin_particles(),
        }
    }
}
