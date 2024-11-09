use godot::{
    classes::{IRigidBody2D, RigidBody2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
#[derive(Debug)]
pub struct MetalObject {
    base: Base<RigidBody2D>,
    weight: i32,
}

#[godot_api]
impl IRigidBody2D for MetalObject {
    fn init(base: Base<RigidBody2D>) -> Self {
        Self { base, weight: 10 }
    }
}

#[godot_api]
impl MetalObject {
    #[func]
    fn is_metal(&self) -> bool {
        true
    }
}
