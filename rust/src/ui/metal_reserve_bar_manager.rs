/// UNFINISHED 
/// 
/// Controls the metal bars on screen, stores bars that are not currently on screen and has methods 
/// to add and remove bars from the on screen Vbox by name. 
/// 
/// Author : Trinity Pittman
/// Version : 09/22/2024

use godot::prelude::*;
use godot::classes::{ITextureProgressBar, IVBoxContainer, TextureProgressBar, VBoxContainer};  // Import Node and VBoxContainer
pub use crate::ui::metal_bar::MetalBar;

use std::collections::HashMap;

const MAX_BARS_ON_SCREEN: u8 = 4;
const TOTAL_BARS: u8 = 10; 

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
        // Create Metals that are auto added to VBox based on the keybound metals 
        for i in 0..TOTAL_BARS {
            let mut bar = MetalBar::new_alloc();
            bar.set_visible(true);
            self.base_mut().add_child(bar);
            
            godot_print!("BARS CREATED")
        }
        self.setup_metals();
    }

}

impl MetalReserveBarManager{

    fn setup_metals(&mut self) {
        // Get all the children of the player 
        let children: Array<Gd<Node>> = self.base.to_gd().get_children(); 
        let paths: [&str; 10] = ["iron", "steel", "pewter", "tin", "bronze", "copper", "duralumin", "nicrosil", "chromium", "gold"];

        for i in 0..children.len() {

            let child : Gd<Node> = children.get(i).expect("");
            if let Ok(mut bar) = child.try_cast::<MetalBar>() {
                    let mut bar_mut = bar.bind_mut();
                    bar_mut.set_name(paths[i]);
                    bar_mut.set_texture(paths[i]);
                    
                    godot_print!("SETUP")
                } else {
                    godot_print!("Failed to cast node to MetalBar");
                }
    
        }
        godot_print!("BARS SETUP")
    }

    pub fn add_metals(&mut self){
        godot_print!("METALS ADDED")
    }

    pub fn add_remove(&mut self, unbind: Gd<MetalBar>, _bind: String){
        self.base_mut().remove_child(unbind);
        // self.unused_metals.insert(unbind);

        // let metal_bind: Gd<MetalBar> =  self.unused_metals.get(bind);
    }
}