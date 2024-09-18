/// Represents a Metal Bar that contains the amount of reserves for a particular metal type. 
/// 
/// Author : Trinity Pittman
/// Version : 09/18/2024

use godot::prelude::*;
use godot::classes::{ITextureProgressBar, TextureProgressBar};

const MAX_RESERVE: u8 = 100;
const MIN_RESERVE: u8 = 0;


#[derive(GodotClass)]
#[class(base=TextureProgressBar)]
pub struct MetalBar {
    base: Base<TextureProgressBar>,
    /// The amount of Metal reserved in the bar 
    reserves: u8,
}


#[godot_api]
impl ITextureProgressBar for MetalBar {

    fn init(base: Base<TextureProgressBar>) -> Self {

        Self {
            base,
            reserves: 0,
        }
    }

    fn ready(&mut self){
        // do i need to set textures?
    }


}

impl MetalBar {

    pub fn get_reserves(&mut self) -> u8 {
        self.reserves
    }

    pub fn set_value(&mut self, reserves: u8){
        self.base.value = reserves; //ngl i have no idea how to make this work 
    }


    pub fn adjust_reserves(&mut self, reserve: i8) {

        let new_reserve = if reserve < 0 {
            self.reserves.wrapping_sub(-reserve as u8)
        } else {
            self.reserves.saturating_add(reserve as u8)
        };

        self.reserves = new_reserve.clamp(MIN_RESERVE, MAX_RESERVE);
        self.set_value(self.reserves);
    }
}