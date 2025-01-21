use godot::prelude::*;
use godot::classes::{IMarker2D, Marker2D};

use crate::items::metal_vial::MetalVial;

#[derive(GodotClass)]
#[class(base=Marker2D)]
pub struct MetalPickup {
    base: Base<Marker2D>,
}

#[godot_api]
impl IMarker2D for MetalPickup {
    fn init(base: Base<Marker2D>) -> Self {
        Self {
            base,
        }
    }
    fn ready(&mut self) {
        let metal_scene = load::<PackedScene>("res://scenes/metal_vial.tscn");
        let mut metal = metal_scene.instantiate_as::<MetalVial>().clone();
        metal.set_name("MetalVialPickup".into());
        metal.set_visible(true);
        metal.set_global_position(self.base().get_global_position());
        godot_print!("POS {}: {}", metal.get_name(), metal.get_position());
    }
}