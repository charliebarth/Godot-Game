/// Represents a Metal Bar that contains the amount of reserves for a particular metal type. 
/// 
/// Author : Trinity Pittman
/// Version : 09/22/2024

use godot::prelude::*;
use godot::classes::{ITextureProgressBar, ResourceLoader, TextureProgressBar, Texture2D};

/// The maximum number of metal reserves a player can have
const MAX_RESERVE: f64 = 100.0;
/// The minumum number of metal reserves a player can have
const MIN_RESERVE: f64 = 0.0;


#[derive(GodotClass)]
#[class(base=TextureProgressBar)]
/// Struct that reprents a Metal Reserve Bar 
pub struct MetalBar {
    base: Base<TextureProgressBar>,
    /// The amount of Metal reserved in the bar 
    reserves: f64,
}


#[godot_api]
impl ITextureProgressBar for MetalBar {

    /// Constructor for a Metal Bar
    fn init(base: Base<TextureProgressBar>) -> Self {

        Self {
            base,
            reserves: 0.0,
        }
    }

    /// Sets the Metals value to 0.0 at the start of the round 
    fn ready(&mut self){
        self.base_mut().set_value(100.0);
        self.base_mut().set_min(MIN_RESERVE);
        self.base_mut().set_max(MAX_RESERVE);
    }


}

impl MetalBar {

    pub fn set_texture(&mut self, path: &str) {
        
        // Every bar will have the same under texture so we set that first
        let under_path: &str = "res://assets/HealthMetalBars/HealthBar DARK.png";
        let texture_under: Gd<Texture2D> = self.load_texture(under_path);
        godot_print!("Texture: {}", texture_under);
        self.base_mut().set_under_texture(texture_under);

        // The progress texture is dependent on the type of metal and is passed into this function
        let path_str: String = format!("res://assets/HealthMetalBars/metal_bar_prog_{}.png", path);
        godot_print!("{}", path_str);
        let texture_progress: Gd<Texture2D> = self.load_texture(path_str.as_str());
        godot_print!("Texture: {}", texture_progress);
        self.base_mut().set_progress_texture(texture_progress);
        self.base_mut().set_texture_progress_offset(Vector2::new(0.0, 1.0));
        
        
    }

    fn load_texture(&mut self, path: &str) -> Gd<Texture2D>{
        let mut loader: Gd<ResourceLoader> = ResourceLoader::singleton(); 

        let path_g: GString = GString::from(path);   // Change the string to a GString for godot
        let tex: Option<Gd<Resource>> = loader.load(path.into());

        tex.unwrap().cast::<Texture2D>() 
    }

    pub fn set_name(&mut self, name: &str) {
        let name_g: GString = GString::from(name);   // Change the string to a GString for godot
        self.base_mut().set_name(name_g);
    }

    pub fn hide(&mut self){
        self.base_mut().hide();
    }

    /// Getter method for the current number of reserves
    pub fn get_reserves(&mut self) -> f64 {
        self.reserves
    }

    /// Setter method for the reserves
    pub fn set_value(&mut self, reserves: f64){
        self.base_mut().set_value(reserves); 
    }

    /// Adjusts the number of reserves of this metal positively or negatively 
    pub fn adjust_reserves(&mut self, reserve: f64) {

        let new_reserve = if reserve < 0.0 {
            if self.reserves < -reserve {
                MIN_RESERVE
            } else {
                self.reserves + reserve
            }
        } else {
            if self.reserves + reserve > MAX_RESERVE {
                MAX_RESERVE
            } else {
                self.reserves + reserve
            }
        };

        self.reserves = new_reserve.clamp(MIN_RESERVE, MAX_RESERVE);
        self.set_value(self.reserves);
    }
}