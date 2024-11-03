use std::borrow::{Borrow, BorrowMut};

/// Represents an Allomantic Line.
/// 
/// Author : Trinity Pittman
/// Version : 10/24/2024

use godot::prelude::*;
use godot::classes::{CharacterBody2D, Area2D, ILine2D, Line2D};

const OFFSET: Vector2 = Vector2::new(400.0, -320.0);

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

    fn physics_process(&mut self, delta: f64) {
        // Get the player position
        let player_pos = self.player.as_mut().unwrap().get_position() + OFFSET;

        // Get the metal position 
        let target_pos = self.metal.as_mut().unwrap().get_position() + OFFSET;


        // Update the line's points
        // self.base_mut().draw_line(player_pos, target_pos, Color::from_rgb(173.,216.,230.));
        self.base_mut().set_points((&[player_pos, target_pos]).into());
        // self.line.as_mut().unwrap().draw_line(player_pos, target_position);
        
    }

    fn ready(&mut self){
        self.base_mut().set_physics_process(true);
    }
}

impl AllomanticLine {

    pub fn initialize_fields(&mut self, metal: Gd<Area2D>, player: Gd<CharacterBody2D>){
        self.metal = Some(metal);
        self.player = Some(player);
    }

    pub fn setup(&mut self){
        let start = self.metal.as_mut().unwrap().get_position();
        let end = self.player.as_mut().unwrap().get_position();

        // self.base_mut().draw_line(start, end, Color::from_rgb(173.,216.,230.));

        // self.base_mut().add_point(start + OFFSET);
        // self.base_mut().add_point(end + OFFSET);
    }

}