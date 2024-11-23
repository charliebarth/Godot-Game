use godot::classes::InputMap;
use godot::{classes::InputEvent, prelude::*};
use std::collections::HashMap;
use std::time::Instant;

use super::enums::coin_events::CoinEvents;
use super::enums::metal_events::MetalEvents;
use super::enums::player_events::{PlayerEvents, TriggerEvents};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct InputManager {
    base: Base<Node2D>,
    button_press_times: HashMap<PlayerEvents, Instant>,
    player_events: HashMap<PlayerEvents, u32>,
    metal_events: HashMap<MetalEvents, Instant>,
    device_id: i32,
}

#[godot_api]
impl INode2D for InputManager {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            button_press_times: HashMap::new(),
            player_events: HashMap::new(),
            metal_events: HashMap::new(),
            device_id: -1,
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.device_id == -1 || event.get_device() != self.device_id {
            return;
        }

        let button_name = InputManager::event_to_input_name(event.clone());

        if let Some(player_event) = PlayerEvents::from_string(&button_name) {
            self.process_player_events(player_event, event);
        } else if let Some(metal_event) = MetalEvents::from_string(&button_name) {
            self.process_metal_events(metal_event, event);
        } 
    }

    fn physics_process(&mut self, _delta: f64) {
        for timer in self.player_events.values_mut() {
            *timer += 1;
        }

        // Expire events after a certain number of frames (e.g., 60 frames)
        self.player_events
            .retain(|event, timer| *timer < event.timeout());
    }
}

impl InputManager {
    pub fn fetch_player_event(&mut self, event: PlayerEvents) -> bool {
        if let Some(_) = self.player_events.remove(&event) {
            true
        } else {
            false
        }
    }

    // Static method
    pub fn event_to_input_name(event: Gd<InputEvent>) -> String {
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

    fn process_metal_events(&mut self, metal_event: MetalEvents, event: Gd<InputEvent>) {
        if event.is_pressed() {
            // When pressed, always insert the burn variant
            self.metal_events.insert(metal_event, Instant::now());
        } else if event.is_released() && self.metal_events.contains_key(&metal_event) {
            if let Some(press_time) = self.metal_events.get(&metal_event) {
                let duration = press_time.elapsed();

                if duration <= std::time::Duration::from_millis(250) {
                    // the burn type will always be burn because the from_string method only returns burn and that is what is passed into this function
                    // if low burn is already toggled on then when it is tapped again it should be toggled off and thus removed from the map
                    // if low burn is not toggled on then it should be toggled on and added to the map thus we need to replace the burn event with the low burn event
                    if !self
                        .metal_events
                        .contains_key(&metal_event.get_low_burn_variant())
                    {
                        self.metal_events
                            .insert(metal_event.get_low_burn_variant(), Instant::now());
                    } else {
                        self.metal_events
                            .remove(&metal_event.get_low_burn_variant());
                    }
                }
            }

            self.metal_events.remove(&metal_event);
        }
    }

    fn process_player_events(&mut self, player_event: PlayerEvents, event: Gd<InputEvent>) {
        if event.is_pressed() {
            let trigger_event = TriggerEvents::trigger_for_player_event(player_event);

            if trigger_event == TriggerEvents::OnPress {
                self.player_events.insert(player_event, 0);
            } else if trigger_event == TriggerEvents::OnRelease {
                self.button_press_times.insert(player_event, Instant::now());
            }
        } else if event.is_released() {
            if let Some(press_time) = self.button_press_times.get(&player_event) {
                let mut player_event = player_event;

                if player_event == PlayerEvents::Roll {
                    let duration = press_time.elapsed();

                    if duration > std::time::Duration::from_millis(250) {
                        player_event = PlayerEvents::Crouch;
                    }
                }

                self.player_events.insert(player_event, 0);
                self.button_press_times.remove(&player_event);
            }
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

    fn process_coin_events(&mut self, coin_event: CoinEvents, event: Gd<InputEvent>){
        if event.is_action_pressed(StringName::from("throw")) {
            // Check if player has coins to throw

            // Get a coin 

            // Throw a coin
            // self.throw(Vector2::new(0., 0.), Vector2::new(150., -200.));
        }
    }
}
