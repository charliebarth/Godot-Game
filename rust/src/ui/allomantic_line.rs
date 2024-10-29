/// Represents an Allomantic Line.
/// 
/// Author : Trinity Pittman
/// Version : 10/24/2024

use godot::prelude::*;
use godot::classes::{CharacterBody2D, Area2D, ILine2D, Line2D};

use crate::traits::MetalObject;

/// Struct that represents an Allomantic Line
#[derive(GodotClass)]
#[class(base=Line2D)]
pub struct AllomanticLine {
    base: Base<Line2D>,
    metal: Option<Gd<Area2D>>, // the metal has to have the MetalObject trait
    player: Option<Gd<CharacterBody2D>>,
    strength: f64,
}


#[godot_api]
impl ILine2D for AllomanticLine {

    /// Constructor for the Allomantic Line
    fn init(base: Base<Line2D>) -> Self {

        Self {
            base,
            metal: None,
            player: None,
            strength: 1.0,
        }
    }

    

    fn ready(&mut self){
    }
}

impl AllomanticLine {

    pub fn draw(&mut self, metal: Gd<Area2D>, player: Gd<CharacterBody2D>){
        let start = metal.get_global_position();
        let end = player.get_global_position();
        self.base_mut().add_point(start);
        self.base_mut().add_point(end);
    }

}