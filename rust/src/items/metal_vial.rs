/// Represents a Metal Vial. 
/// 
/// Author : Trinity Pittman
/// Version : 10/03/2024

use godot::prelude::*;
use godot::classes::{Area2D, IArea2D};

use crate::player::player::Player;


/// Represents a Metal Vial  
#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct MetalVial {
    base: Base<Area2D>,
    metals: Option<Vec<StringName>>, 
    amt: f64
}

#[godot_api]
impl IArea2D for MetalVial {

    /// Constructor for a Metal Vial 
    fn init(base: Base<Area2D>) -> Self {
        Self {
            base,
            metals: None,
            amt: 10.
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
            player.bind_mut().adjust_metals(self.get_metals(), self.amt); // Dereference and call the method
            self.base_mut().queue_free();     // Remove the vial from the scene 
        } else {
            godot_print!("Something other than player entered the coin.");
        }            
    }

    fn get_metals(&mut self) -> Vec<StringName>{
        if self.metals.is_none() {
            self.metals = Some(Vec::new());
        }
        self.metals.as_ref().expect("Metals not found").clone()
    }

    fn set_metals(&mut self, metals: Vec<StringName>) {
        self.metals = Some(metals);
    }

    #[func]
    fn is_metal(&self) -> bool {
        true // Metal vials are made of metal
    }
}