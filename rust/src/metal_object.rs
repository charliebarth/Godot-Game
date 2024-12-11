use godot::{
    classes::{IRigidBody2D, RigidBody2D},
    prelude::*,
};

/// This is a Node for immovable metal objects.
#[derive(GodotClass)]
#[class(base=RigidBody2D)]
#[derive(Debug)]
pub struct MetalObject {
    /// The base node of the MetalObject.
    base: Base<RigidBody2D>,
}

#[godot_api]
impl IRigidBody2D for MetalObject {
    /// The Godot constructor for the MetalObject class.
    ///
    /// # Arguments
    /// * `base` - The base node of the MetalObject.
    ///
    /// # Returns
    /// * `MetalObject` - A new instance of the MetalObject class.
    fn init(base: Base<RigidBody2D>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl MetalObject {
    /// This method is the way to determine if the object is metal.
    ///
    /// # Returns
    /// * `bool` - True if the object is metal.
    #[func]
    pub fn is_metal(&self) -> bool {
        true
    }
}
