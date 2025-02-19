use godot::classes::{IMarker2D, Marker2D, Timer};
use godot::prelude::*;

use crate::game::Game;
use crate::items::metal_vial::MetalVial;

const WAIT_TIME: f64 = 5.;
const OFF_MAP: Vector2 = Vector2::new(-100000., 100000.);

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
        self.get_metal_vial().set_global_position(OFF_MAP);

        let mut timer: Gd<Timer> = self.base().get_node_as("./Timer");
        timer.set_autostart(true);
        timer.set_wait_time(WAIT_TIME);
        timer.set_one_shot(false);
        timer.start();

        godot_print!("TIMER START");
    }
}

#[godot_api]
impl MetalPickup {
    fn make_vial(&mut self) {
        let metal_scene = load::<PackedScene>("res://scenes/metal_vial.tscn");
        let mut metal = metal_scene.instantiate_as::<MetalVial>().clone();
        metal.set_name("MetalVialPickup");
        metal.set_visible(true);

        let mut new_metals = Vec::new();

        let mode = self.find_game_mode();

        if mode == "last_player_standing".to_string() {
            //TODO set what we want the vials to incr
            godot_print!("GAME MODE LAST PLAYER STANDING");
            new_metals.push("pewter");
        } else {
            godot_print!("GAME MODE SMTH ELSE");
            new_metals.push("iron");
            new_metals.push("steel");
        }

        metal.bind_mut().set_metals(new_metals);

        self.metal_vial = Some(metal);

        // Add metal vial to node tree
        let vial = self.get_metal_vial();
        self.base_mut().add_child(&vial);
    }

    fn get_metal_vial(&mut self) -> Gd<MetalVial> {
        self.metal_vial
            .as_ref()
            .expect("Could not find metal vial")
            .clone()
    }

    fn find_game_mode(&mut self) -> String {
        let mode = Game::get_game_mode();

        mode
    }

    #[func]
    fn on_timer_timeout(&mut self) {
        godot_print!("TIMER TIMEOUT");

        if self.get_metal_vial().get_global_position() == OFF_MAP {
            self.get_metal_vial()
                .set_global_position(self.base().get_global_position());
        } else {
            godot_print!("MetalVial still exists, skipping respawn.\n");
        }
    }
}
