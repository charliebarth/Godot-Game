/// Represents an Allomantic Line.
/// 
/// Author : Trinity Pittman
/// Version : 10/27/2024

use godot::prelude::*;
use godot::classes::{Area2D, IArea2D};

use crate::items::coin::Coin;
use crate::traits::MetalObject;

/// Struct that represents an Allomantic Range
#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct AllomanticRange {
    base: Base<Area2D>,
}


#[godot_api]
impl IArea2D for AllomanticRange {

    /// Constructor for the Allomantic Range
    fn init(base: Base<Area2D>) -> Self {

        Self {
            base,
        }
    }

    fn ready(&mut self){
    }
}

#[godot_api]
impl AllomanticRange {

    /// When something enters this hitbox we...
    /// 
    /// Args: 
    ///      body (Gd<Node2D>): the Node that enters this hitbox  
    #[func]
    fn enter_range(&mut self, body: Gd<Area2D>) {
        let body_name = body.get_name();
        godot_print!("Alomantic range entered by {body_name}");   

        // TODO - this is not to my liking in forms of loosley coupled. 
        if let Ok(metal_object) = body.try_cast::<Coin>() {
            if metal_object.bind().is_metal() {
                godot_print!("IS METAL: {}", body_name);
                // Additional logic for metal objects
            }
        } else {
            godot_print!("Something other than a metal object entered the allomantic range.");
        }
    
        // if body.has_method(StringName::from("is_metal")) {
        //     godot_print!("IS METAL")
            
        // } else {
        //     godot_print!("Something other than a metal object entered the allomantic range.");
        // }
    }       
}
