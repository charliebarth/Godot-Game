/// Represents an Allomantic Line.
/// 
/// Author : Trinity Pittman
/// Version : 10/24/2024

use godot::prelude::*;
use godot::classes::{Area2D, CharacterBody2D, ILine2D, Line2D};

const OFFSET: Vector2 = Vector2::new(0., 10.0);

/// Struct that represents an Allomantic Line
#[derive(GodotClass)]
#[class(base=Line2D)]
pub struct AllomanticLine {
    base: Base<Line2D>,
    metal: Option<Gd<Area2D>>, // the metal has to have the MetalObject trait
    player: Option<Gd<CharacterBody2D>>,
    strength: f32,
}


#[godot_api]
impl ILine2D for AllomanticLine {

    /// Constructor for the Allomantic Line
    fn init(base: Base<Line2D>) -> Self {
        Self {
            base,
            metal: None,
            player: None,
            strength: 1.,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.draw();        
    }

    fn ready(&mut self){
        self.base_mut().set_physics_process(true);
    }

}

#[godot_api]
impl AllomanticLine {

    #[func]
    fn draw(&mut self){
        if let (Some(metal), Some(player)) = (self.metal.as_ref(), self.player.as_ref()) {
            let start = metal.get_global_position();
            let end = player.get_global_position();

            let start_local = self.base_mut().to_local(start);
            let end_local = self.base_mut().to_local(end) + OFFSET;

            // godot_print!("Drawing line from {:?} to {:?}", start_local, end_local);

            // Draw the line between the positions
            self.base_mut().clear_points();  // Clear previous points
            self.base_mut().add_point(start_local);
            self.base_mut().add_point(end_local);

            // Optionally set the color and line width
            let blue = 0.8 * self.strength;
            self.base_mut().set_default_color(Color::from_rgb(0.2, 0.4, blue).with_alpha(1.0));
            self.base_mut().set_width(2.0);  
        }
        
    }

    pub fn initialize_fields(&mut self, metal: Gd<Area2D>, player: Gd<CharacterBody2D>){
        self.metal = Some(metal);
        self.player = Some(player);
        // self.physics_process(0.1);

    }

    #[export_name = "get_metal"]
    pub fn get_metal(&self) -> Option<Gd<Area2D>> {
        self.metal.clone()
    }
}