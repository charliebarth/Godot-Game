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

    fn input(&mut self, event: Gd<InputEvent>) {}
}
