/// input_manager.rs
///
/// This module defines the InputManager class, which is responsible for handling all input events
/// for a given player and device. It will convert button presses into player events and metal events.
/// This allows for extra functionality such as being able to map different events to the same button
/// based on how long the button is held down.
///
/// Author: Charles Barth
/// Version: Spring 2025
use godot::classes::{Engine, Input, InputMap};
use godot::{classes::InputEvent, prelude::*};
use std::collections::{HashMap, HashSet};

use crate::game::Game;
use crate::settings::Settings;

use super::enums::metal_type::{BurnType, ButtonState, MetalType};
use super::enums::player_events::PlayerEvents;
use super::player::Player;

/// The input manager is responsible for handling all input events for a given
/// player and device.
/// It will convert button presses into player events and metal events.
/// This allows us to add extra functionality such as being able to map to
/// different events to the same button
/// based on how long the button is held down.
#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct InputManager {
    /// The base node of the InputManager.
    base: Base<Node2D>,
    /// The player events that have been triggered.
    /// Events will be removed after a certain number of frames or when the
    /// button is released.
    player_events: HashMap<PlayerEvents, i8>,
    /// The metal events that have been triggered.
    /// This will persist until the button is released.
    metal_events: HashSet<(MetalType, BurnType, ButtonState)>,
    /// A hashmap to keep track of whether a button has been released.
    /// This prevents an event from being triggered multiple times while a
    /// button is held down.
    button_released: HashMap<String, bool>,
    /// The device id that the input manager is listening for.
    device_id: i32,
    /// A hashmap to keep track of the left and right movement values.
    left_right: HashMap<&'static str, f32>,
    /// Whether the player is a remote player.
    remote_player: bool,
    /// The recent device id that the input manager is listening for.
    recent_device: i32,
    /// The player id of the player that the input manager is listening for.
    player_id: i32,
}

#[godot_api]
impl INode2D for InputManager {
    /// The Godot constructor for the InputManager class.
    ///
    /// # Arguments
    /// * `base` - The base node of the InputManager.
    ///
    /// # Returns
    /// * `InputManager` - A new instance of the InputManager class.
    fn init(base: Base<Node2D>) -> Self {
        let mut left_right: HashMap<&'static str, f32> = HashMap::new();
        left_right.insert("move_left", 0.0);
        left_right.insert("move_right", 0.0);

        Self {
            base,
            player_events: HashMap::new(),
            metal_events: HashSet::new(),
            button_released: HashMap::new(),
            device_id: -1,
            left_right,
            remote_player: false,
            recent_device: -1,
            player_id: -1,
        }
    }

    /// This is a built in method for Godot that is called when an input event
    /// is detected.
    /// An input event will be converted to either a PlayerEvent or a MetalEvent
    /// and then stored for use in the game.
    fn ready(&mut self) {
        let player = self
            .base()
            .get_parent()
            .unwrap()
            .try_cast::<Player>()
            .unwrap();

        self.remote_player = player.bind().is_remote_player();
        self.player_id = player.bind().get_player_id();
    }

    /// This is a built in method for Godot that is called when an input event is detected.
    /// An input event will be converted to either a PlayerEvent or a MetalEvent and then stored for use in the game.
    ///
    /// # Arguments
    /// * `event` - The input event that was detected.
    fn input(&mut self, event: Gd<InputEvent>) {
        if (self.device_id != -1 && event.get_device() != self.device_id)
            || event.is_echo()
            || self.remote_player
        {
            return;
        }

        let button_name = InputManager::event_to_input_name(event.clone());

        if button_name == "" {
            return;
        }

        self.handle_input(
            button_name.clone(),
            event.is_action_pressed(button_name.as_str()),
            event.is_action_released(button_name.as_str()),
            event.get_action_strength(button_name.as_str()),
        );
    }

    /// This is a built in method for Godot that is called every physics frame.
    /// This is where the player events are updated and expired after a certain
    /// number of frames.
    ///
    /// # Arguments
    /// * `delta` - The time since the last frame.
    fn physics_process(&mut self, _delta: f64) {
        for (event, timer) in self.player_events.iter_mut() {
            if event.timeout() > -1 {
                *timer += 1;
            }
        }

        // Expire events after a certain number of frames (e.g., 60 frames)
        self.player_events
            .retain(|event, timer| event.timeout() == -1 || *timer < event.timeout());

        let settings = Engine::singleton()
            .get_singleton("Settings")
            .expect("settings singleton missing")
            .try_cast::<Settings>()
            .expect("settings is not a Settings");
        let online = settings.bind().get_online_multiplayer();
        if !self.remote_player {
            if self.device_id > -1 {
                self.left_right.insert(
                    "move_left",
                    Input::singleton()
                        .get_action_strength(format!("move_left{}", self.device_id).as_str()),
                );

                self.left_right.insert(
                    "move_right",
                    Input::singleton()
                        .get_action_strength(format!("move_right{}", self.device_id).as_str()),
                );
            } else if self.recent_device != -1 {
                self.left_right.insert(
                    "move_left",
                    Input::singleton()
                        .get_action_strength(format!("move_left{}", self.recent_device).as_str()),
                );

                self.left_right.insert(
                    "move_right",
                    Input::singleton()
                        .get_action_strength(format!("move_right{}", self.recent_device).as_str()),
                );
            }

            if online {
                let mut game = self.base().get_node_as::<Game>("/root/Game");
                game.rpc_id(
                    1,
                    "receive_movement",
                    &[
                        Variant::from(self.player_id),
                        Variant::from(self.get_left_value()),
                        Variant::from(self.get_right_value()),
                    ],
                );
            }
        }
    }
}

#[godot_api]
impl InputManager {
    /// Returns whether the player is a remote player.
    ///
    /// # Returns
    /// * `bool` - True if the player is a remote player, false otherwise
    #[func]
    pub fn get_is_remote_player(&self) -> bool {
        self.remote_player
    }

    /// Returns the device id of the player that the input manager is listening for.
    ///
    /// # Returns
    /// * `i32` - The device id of the player that the input manager is listening for
    #[func]
    pub fn get_device_id(&self) -> i32 {
        self.device_id
    }

    /// Returns the recent device id that the input manager is listening for.
    ///
    /// # Returns
    /// * `i32` - The recent device id that the input manager is listening for
    #[func]
    pub fn get_recent_device(&self) -> i32 {
        self.recent_device
    }

    /// Sets the recent device id that the input manager is listening for.
    ///
    /// # Arguments
    /// * `device` - The device id to set
    #[func]
    pub fn set_recent_device(&mut self, device: i32) {
        self.recent_device = device;
    }

    /// Sets the left and right movement values.
    ///
    /// # Arguments
    /// * `left` - The left movement value
    /// * `right` - The right movement value
    #[func]
    pub fn set_left_right(&mut self, left: f32, right: f32) {
        self.left_right.insert("move_left", left);
        self.left_right.insert("move_right", right);
    }

    /// Returns the left movement value.
    ///
    /// # Returns
    /// * `f32` - The left movement value
    #[func]
    fn get_left_value(&self) -> f32 {
        *self.left_right.get("move_left").unwrap()
    }

    /// Returns the right movement value.
    ///
    /// # Returns
    /// * `f32` - The right movement value
    #[func]
    fn get_right_value(&self) -> f32 {
        *self.left_right.get("move_right").unwrap()
    }

    /// Handles the input event.
    /// This is what used to be in the input method.
    ///
    /// # Arguments
    /// * `button_name` - The name of the button that was pressed
    /// * `is_pressed` - Whether the button was pressed
    /// * `is_released` - Whether the button was released
    #[func]
    pub fn handle_input(
        &mut self,
        button_name: String,
        is_pressed: bool,
        is_released: bool,
        action_strength: f32,
    ) {
        if !self.button_released.contains_key(&button_name) {
            self.button_released.insert(button_name.clone(), true);
        }

        if button_name.contains("move_") {
            self.left_right.insert("move_left", action_strength);
        } else if button_name.contains("move_right") {
            self.left_right.insert("move_right", action_strength);
        } else if let Some(player_event) = PlayerEvents::from_string(&button_name) {
            self.process_player_events(player_event, is_pressed, is_released, button_name);
        } else if let Some(metal_type) = MetalType::from_string(&button_name) {
            self.process_metal_events(metal_type, is_pressed, is_released, button_name);
        }
    }

    /// Returns the name of the button that was pressed.
    ///
    /// # Arguments
    /// * `event` - The input event that was detected
    ///
    /// # Returns
    #[func]
    pub fn get_button_name(&mut self, event: Gd<InputEvent>) -> String {
        InputManager::event_to_input_name(event)
    }

    /// Returns the left and right movement values combined.
    ///
    /// # Returns
    /// * `f32` - The left and right movement values combined
    #[func]
    pub fn get_left_right_value(&self) -> f32 {
        let move_left = self.left_right.get("move_left").unwrap();
        let move_right = self.left_right.get("move_right").unwrap();
        -*move_left + *move_right
    }

    /// Fetching the events checks if the event is in the hashmap and if it is
    /// it removes it and returns true otherwise it returns false.
    ///
    /// Arguments:
    /// * `event` - The event to fetch
    ///
    /// Returns:
    /// * `bool` - True if the event was in the hashmap and removed,
    ///            false otherwise
    pub fn fetch_player_event(&mut self, event: PlayerEvents) -> bool {
        if let Some(_) = self.player_events.remove(&event) {
            true
        } else {
            false
        }
    }

    // #[func]
    // pub fn fetch_player_event_from_str(&mut self, event: String) -> bool {
    //     if let Some(_) = self.player_events.remove(&PlayerEvents::from_string(&event).expect("Couldn't parse into a player event.")) {
    //         true
    //     } else {
    //         false
    //     }
    // }

    /// Checks if the event is in the hashmap but does not remove it.
    ///
    /// # Arguments
    /// * `event` - The event to check for
    ///
    /// # Returns
    /// * `bool` - True if the event is in the hashmap, false otherwise
    #[func]
    pub fn check_for_player_event(&self, event: PlayerEvents) -> bool {
        self.player_events.contains_key(&event)
    }

    #[func]
    pub fn str_to_player_event(&self, event: String) -> PlayerEvents {
        PlayerEvents::from_string(&event).expect("Couldn't parse into a player event.")
    }

    /// Takes an InputEvent and returns the name of the input event.
    ///  
    /// # Arguments
    /// * `event` (`Gd<InputEvent>`) - the input event to convert to string
    ///                                representation
    /// # Returns
    /// * `String`` - the string representation of the event
    pub fn event_to_input_name(event: Gd<InputEvent>) -> String {
        let mut input_map = InputMap::singleton();
        let inputs = input_map.get_actions();

        let length = inputs.len();
        for i in (0..length).rev() {
            let input = inputs.get(i).unwrap();
            let input_str = input.to_string();

            // Skip inputs that start with "ui_"
            if input_str.starts_with("ui_") {
                continue;
            }

            if input_map.event_is_action(&event, &input) {
                return input_str;
            }
        }

        "".to_string()
    }

    /// This function takes a MetalEvent and determines if it should be stored,
    /// removed, or toggled.
    ///
    /// Arguments:
    /// * `metal_event` - The MetalEvent to process
    /// * `event` - The input event that was detected
    /// * `button_name` - The name of the button that was pressed
    fn process_metal_events(
        &mut self,
        metal_type: MetalType,
        is_pressed: bool,
        is_released: bool,
        button_name: String,
    ) {
        let burn_type = if self.player_events.contains_key(&PlayerEvents::LowBurn) {
            BurnType::LowBurn
        } else {
            BurnType::Burn
        };

        // If the button is pressed
        if is_pressed {
            // If the player is holding down the low burn button then this is a low burn event
            if burn_type == BurnType::LowBurn {
                // If the low burn event is already in the set then remove it to
                // stop the low burn
                if self
                    .metal_events
                    .contains(&(metal_type, burn_type, ButtonState::Pressed))
                {
                    self.metal_events
                        .remove(&(metal_type, burn_type, ButtonState::Pressed));

                // If the low burn event is not in the set then add it to start the low burn
                } else {
                    self.metal_events
                        .insert((metal_type, burn_type, ButtonState::Pressed));
                }

            // If the player is not holding down the low burn button then this is a burn event
            } else {
                self.metal_events
                    .insert((metal_type, burn_type, ButtonState::Pressed));
            }

        // If the button is released
        } else if burn_type != BurnType::LowBurn && is_released {
            self.metal_events
                .remove(&(metal_type, burn_type, ButtonState::Pressed));
        }
    }

    /// Determines if a specific metal event has been triggered.
    /// Pressed events are left in the set so they can be replaced with a
    /// released event when the button is released.
    /// Released events are removed from the set when fetched.
    ///
    /// Arguments:
    /// * `metal_event` - The metal event to check for
    ///
    /// Returns:
    /// * `bool` - True if the metal event has been triggered, false otherwise
    pub fn fetch_metal_event(&mut self, metal_event: (MetalType, BurnType, ButtonState)) -> bool {
        let button_state = metal_event.2;

        if button_state == ButtonState::Pressed {
            self.metal_events.contains(&metal_event)
        } else {
            self.metal_events.remove(&metal_event)
        }
    }

    /// This function takes a PlayerEvent and determines if it should be stored
    /// or removed.
    /// It also keeps track of whether a button has been released to prevent an
    /// event from being triggered multiple times while a button is held down.
    ///
    /// Arguments:
    /// * `player_event` - The PlayerEvent to process
    /// * `event` - The input event that was detected
    /// * `button_name` - The name of the button that was pressed
    fn process_player_events(
        &mut self,
        player_event: PlayerEvents,
        is_pressed: bool,
        is_released: bool,
        button_name: String,
    ) {
        if is_pressed
            && !self.player_events.contains_key(&player_event)
            && *self.button_released.get(&button_name).unwrap()
        {
            self.button_released.insert(button_name, false);
            self.player_events.insert(player_event, 0);
        } else if is_released {
            self.button_released.insert(button_name, true);
            self.player_events.remove(&player_event);
        }
    }

    /// Sets the device id that the input manager is listening for.
    ///
    /// Arguments:
    /// * `device_id` - The device id to set
    pub fn set_device_id(&mut self, device_id: i32) {
        self.device_id = device_id;
    }
}
