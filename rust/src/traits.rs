use godot::{classes::Area2D, obj::{Gd, NewAlloc}};

// Define the MetalObject trait 
pub trait MetalObject {
    fn is_metal(&self) -> bool;
    fn new_alloc2() -> Gd<Self>
    where 
        Self: godot::prelude::GodotClass;
}