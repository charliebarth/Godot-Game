/// UNFINISHED 
/// 
/// Controls the metal bars on screen, stores bars that are not currently on screen and has methods 
/// to add and remove bars from the on screen Vbox by name. 
/// 
/// Author : Trinity Pittman
/// Version : 09/18/2024

use godot::prelude::*;
use godot::classes::{IVBoxContainer, VBoxContainer};  // Import Node and VBoxContainer
pub use crate::metal_bar::MetalBar;

use std::collections::HashMap;


const MAX_BARS_ON_SCREEN: u8 = 4;

#[derive(GodotClass)]
#[class(base=VBoxContainer)]
pub struct MetalReserveBarManager {
    base: Base<VBoxContainer>,
    /// data structure (like a list) where metals not on screen will be stored 
    unused_metals: HashMap<String, Gd<MetalBar>>,      
}




/// where the methods that belong to MetalReserveBarManager will be stored
#[godot_api]
impl IVBoxContainer for  MetalReserveBarManager {    
    // Initialization method for `MetalReserveBarManager`
    fn init(base: Base<VBoxContainer>) -> Self {
        Self {
            base,
            unused_metals: HashMap::new(),
        }
    }       

    fn ready(&mut self) { 
        // Create Metals that are auto added to VBox 

    }

}

impl MetalReserveBarManager{

    pub fn add_remove(&mut self, unbind: Gd<MetalBar>, _bind: String){
        self.base_mut().remove_child(unbind);
        // self.unused_metals.insert(unbind);

        // let metal_bind: Gd<MetalBar> =  self.unused_metals.get(bind);
    }
}