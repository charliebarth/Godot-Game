use godot::classes::InputMap;
use godot::{classes::InputEvent, prelude::*};
use std::collections::{HashMap, HashSet};

use super::enums::coin_events::CoinEvents;
use super::enums::metal_events::MetalEvents;
use super::enums::player_events::PlayerEvents;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct InputManager {
    base: Base<Node2D>,
    player_events: HashMap<PlayerEvents, i8>,
    metal_events: HashSet<MetalEvents>,
    button_released: HashMap<String, bool>,
    device_id: i32,
}

#[godot_api]
impl INode2D for InputManager {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            player_events: HashMap::new(),
            metal_events: HashSet::new(),
            button_released: HashMap::new(),
            device_id: -1,
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.device_id == -1 || event.get_device() != self.device_id {
            return;
        }

        let button_name = self.event_to_input_name(event.clone());

        if !self.button_released.contains_key(&button_name) {
            self.button_released.insert(button_name.clone(), true);
        }

        if let Some(player_event) = PlayerEvents::from_string(&button_name) {
            self.process_player_events(player_event, event, button_name);
        } else if let Some(metal_event) = MetalEvents::from_string(&button_name) {
            self.process_metal_events(metal_event, event, button_name);
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        for (event, timer) in self.player_events.iter_mut() {
            if event.timeout() > -1 {
                *timer += 1;
            }
        }

        // Expire events after a certain number of frames (e.g., 60 frames)
        self.player_events
            .retain(|event, timer| event.timeout() == -1 || *timer < event.timeout());
    }
}

impl InputManager {
    /// Fetching the events checks if the event is in the hashmap and if it is it removes it and returns true otherwise it returns false.
    ///
    /// Arguments:
    /// * `event` - The event to fetch
    ///
    /// Returns:
    /// * `bool` - True if the event was in the hashmap and removed, false otherwise
    pub fn fetch_player_event(&mut self, event: PlayerEvents) -> bool {
        if let Some(_) = self.player_events.remove(&event) {
            true
        } else {
            false
        }
    }

    /// Checks if the event is in the hashmap but does not remove it.
    ///
    /// Arguments:
    /// * `event` - The event to check for
    ///
    /// Returns:
    /// * `bool` - True if the event is in the hashmap, false otherwise
    pub fn check_for_player_event(&self, event: PlayerEvents) -> bool {
        self.player_events.contains_key(&event)
    }

    fn event_to_input_name(&self, event: Gd<InputEvent>) -> String {
        let mut input_map = InputMap::singleton();
        let inputs = input_map.get_actions();

        let length = inputs.len();
        for i in (0..length).rev() {
            let input = inputs.get(i).unwrap();
            let input_str = input.to_string();

            // Skip inputs that start with "ui_"
            if input_str.starts_with("ui_") {
                break;
            }

            if input_map.event_is_action(event.clone(), input.clone()) {
                return input_str;
            }
        }

        "".to_string()
    }

    fn process_metal_events(
        &mut self,
        metal_event: MetalEvents,
        event: Gd<InputEvent>,
        button_name: String,
    ) {
        if event.is_pressed() {
            if self.player_events.contains_key(&PlayerEvents::LowBurn) {
                if let Some(low_burn_variant) = MetalEvents::get_low_burn_variant(metal_event) {
                    if self.metal_events.contains(&low_burn_variant) {
                        self.button_released.insert(button_name, true);
                        self.metal_events.remove(&low_burn_variant);
                    } else {
                        self.button_released.insert(button_name, false);
                        self.metal_events.insert(low_burn_variant);
                    }
                }
            } else {
                self.button_released.insert(button_name, false);
                self.metal_events.insert(metal_event);
            }
        } else if event.is_released() {
            self.button_released.insert(button_name, true);
            self.metal_events.remove(&metal_event);
        }
    }

    fn process_player_events(
        &mut self,
        player_event: PlayerEvents,
        event: Gd<InputEvent>,
        button_name: String,
    ) {
        if event.is_pressed()
            && !self.player_events.contains_key(&player_event)
            && *self.button_released.get(&button_name).unwrap()
        {
            self.button_released.insert(button_name, false);
            self.player_events.insert(player_event, 0);
        } else if event.is_released() {
            self.button_released.insert(button_name, true);
            self.player_events.remove(&player_event);
        }
    }

    pub fn fetch_metal_event(&mut self, metal_event: MetalEvents) -> bool {
        if let Some(_) = self.metal_events.get(&metal_event) {
            true
        } else {
            false
        }
    }

    pub fn set_device_id(&mut self, device_id: i32) {
        self.device_id = device_id;
    }

    fn process_coin_events(&mut self, coin_event: CoinEvents, event: Gd<InputEvent>) {
        if event.is_action_pressed(StringName::from("throw")) {
            // Check if player has coins to throw

            // Get a coin

            // Throw a coin
            // self.throw(Vector2::new(0., 0.), Vector2::new(150., -200.));
        }
    }
}
