use godot::{classes::InputEvent, prelude::*};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct InputManager {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for InputManager {
    fn init(base: Base<Node2D>) -> Self {
        Self { base }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        godot_print!("Input event: {}", event.as_text());
        godot_print!("Is pressed: {}", event.is_pressed());
        godot_print!("Is released: {}", event.is_released());
        // is_release is useful for buttons but not triggers or sticks but those have a value
        // triggers return true for is pressed when over half pull and false when below half pull
        // this might be tied to something like deadzone in settings but I'm not sure yet
    }
}
