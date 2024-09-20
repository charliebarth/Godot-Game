/// Represents a Metal Bar that contains the amount of reserves for a particular metal type. 
/// 
/// Author : Trinity Pittman
/// Version : 09/18/2024

use godot::prelude::*;
use godot::classes::{ITextureProgressBar, TextureProgressBar};

const MAX_RESERVE: f64 = 100.0;
const MIN_RESERVE: f64 = 0.0;


#[derive(GodotClass)]
#[class(base=TextureProgressBar)]
pub struct MetalBar {
    base: Base<TextureProgressBar>,
    /// The amount of Metal reserved in the bar 
    reserves: f64,
}


#[godot_api]
impl ITextureProgressBar for MetalBar {

    fn init(base: Base<TextureProgressBar>) -> Self {

        Self {
            base,
            reserves: 0.0,
        }
    }

    fn ready(&mut self){
        // do i need to set textures?
    }


}

impl MetalBar {

    pub fn get_reserves(&mut self) -> f64 {
        self.reserves
    }

    pub fn set_value(&mut self, reserves: f64){
        self.base_mut().set_value(reserves); 
    }


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