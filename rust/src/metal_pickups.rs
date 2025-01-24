use godot::prelude::*;
use godot::classes::{IMarker2D, Marker2D, Timer};

use crate::items::coin::Coin;
use crate::items::metal_vial::MetalVial;

#[derive(GodotClass)]
#[class(base=Marker2D)]
pub struct MetalPickup {
    base: Base<Marker2D>,
    metal_vial: Option<Gd<MetalVial>>,
}

#[godot_api]
impl IMarker2D for MetalPickup {
    fn init(base: Base<Marker2D>) -> Self {
        Self {
            base,
            metal_vial: None,
        }
    }

    fn ready(&mut self) {
        self.make_vial();

        let mut timer: Gd<Timer> = self.base().get_node_as("./Timer");
        timer.set_wait_time(5.0);
        timer.set_one_shot(false);
        timer.start();
        godot_print!("TIMER START");
        godot_print!(
            "Timer exists: {}",
            self.base().has_node("./Timer".into()) // Verifies the timer node exists
        );
        godot_print!("Timer is stopped: {}", timer.is_stopped());
        godot_print!("Time left: {}\nTimer wait: {}", 
            timer.get_time_left(),
            timer.get_wait_time());
        
    }
}

#[godot_api]
impl MetalPickup {
    fn make_vial(&mut self) {
        let metal_scene = load::<PackedScene>("res://scenes/metal_vial.tscn");
        let mut metal = metal_scene.instantiate_as::<MetalVial>().clone();
        metal.set_name("MetalVialPickup".into());
        metal.set_visible(true);
        self.metal_vial = Some(metal);

        // Add metal vial to node tree
        let vial = self.get_metal_vial();
        self.base_mut().add_child(vial);
    }

    fn get_metal_vial(&mut self) -> Gd<MetalVial> {
        self.metal_vial
            .as_ref()
            .expect("Could not find metal vial")
            .clone()
    }

    #[func]
    fn on_timer_timeout(&mut self) {
        godot_print!("TIMER TIMEOUT");
        if self.metal_vial.as_ref().unwrap().is_inside_tree() {
            godot_print!("MetalVial still exists, skipping respawn.");
        } else {
            self.make_vial();
        }
    }
}