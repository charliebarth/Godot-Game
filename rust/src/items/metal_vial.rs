/// Represents a Metal Vial. 
/// 
/// Author : Trinity Pittman
/// Version : 10/03/2024

use godot::prelude::*;
use godot::classes::{Area2D, IArea2D};

use crate::player::player::Player;
use crate::traits::MetalObject;


/// Represents a Metal Vial  
#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct MetalVial {
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for MetalVial {

    /// Constructor for a Metal Vial 
    fn init(base: Base<Area2D>) -> Self {
        Self {
            base,
        }
    }
}

#[godot_api]
impl MetalVial {

    /// When someone enters this metal vial hit box we call the method to add metal to that players  
    /// metal bars. 
    /// 
    /// Args: 
    ///      body (Gd<Node2D>): the Node that enters this metal vial. 
    #[func]
    fn metal_pickup(&mut self, body: Gd<Node2D>) {
        let body_name = body.get_name();
        godot_print!("Metal entered by {body_name}");    // Prints who picked up the coin
        
        if let Ok(mut player) = body.try_cast::<Player>() {
            player.bind_mut().adjust_metals(); // Dereference and call the method
            self.base_mut().queue_free();     // Remove the vial from the scene 
        } else {
            godot_print!("Something other than player entered the coin.");
        }            
    }
}


impl MetalObject for MetalVial {
    fn is_metal(&self) -> bool {
        true // Metal vials are made of metal
    }

    fn new_alloc2() -> Gd<Self> {
        MetalVial::new_alloc()
    }
}