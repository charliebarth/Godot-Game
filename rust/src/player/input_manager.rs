use godot::classes::InputMap;
use godot::{classes::InputEvent, prelude::*};
use std::collections::HashMap;
use std::time::Instant;

use super::enums::player_events::PlayerEvents;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct InputManager {
    base: Base<Node2D>,
    button_press_times: HashMap<String, Instant>,
    events: HashMap<PlayerEvents, u32>,
}

#[godot_api]
impl INode2D for InputManager {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            button_press_times: HashMap::new(),
            events: HashMap::new(),
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let button_name = self.event_to_input_name(event.clone());

        if event.is_pressed() {
            self.button_press_times
                .insert(button_name.clone(), Instant::now());
        }

        if event.is_released() {
            if let Some(press_time) = self.button_press_times.get(&button_name) {
                if button_name == "b_button" {
                    let duration = press_time.elapsed();
                    godot_print!("duration: {}", duration.as_millis());

                    if duration < std::time::Duration::from_millis(250) {
                        self.events.insert(PlayerEvents::Roll, 0);
                    } else {
                        self.events.insert(PlayerEvents::Crouch, 0);
                    }
                } else if let Some(event) = PlayerEvents::from_string(&button_name) {
                    self.events.insert(event, 0);
                }

                self.button_press_times.remove(&button_name);
            }
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        for timer in self.events.values_mut() {
            *timer += 1;
        }

        // Expire events after a certain number of frames (e.g., 60 frames)
        self.events.retain(|_, timer| *timer < 3);
    }
}

impl InputManager {
    pub fn fetch_event(&mut self, event: PlayerEvents) -> bool {
        if let Some(_) = self.events.remove(&event) {
            true
        } else {
            false
        }
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
                continue;
            }

            if input_map.event_is_action(event.clone(), input.clone()) {
                return input_str;
            }
        }

        "".to_string()
    }
}
