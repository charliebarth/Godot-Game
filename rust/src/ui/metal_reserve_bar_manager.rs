/// UNFINISHED 
/// 
/// Controls the metal bars on screen, stores bars that are not currently on screen and has methods 
/// to add and remove bars from the on screen Vbox by name. 
/// 
/// Author : Trinity Pittman
/// Version : 10/10/2024

use godot::prelude::*;
use godot::classes::{IVBoxContainer, InputMap, VBoxContainer};  // Import Node and VBoxContainer
pub use crate::ui::metal_bar::MetalBar;

use std::collections::HashMap;

// The maximum number of bars to display on a players screen at a time 
const MAX_BARS_ON_SCREEN: u8 = 4;
// Change this to account for how many we currently support
const TOTAL_BARS: u8 = 10;   
// Represents the order of supported metals (simply reorder these based on implementation)
const PATHS: [&str; 10] = ["iron", "steel", "pewter", "tin", "bronze", "copper", "duralumin", "nicrosil", "chromium", "gold"];

#[derive(GodotClass)]
#[class(base=VBoxContainer)]
pub struct MetalReserveBarManager {
    base: Base<VBoxContainer>,
    /// data structure (like a list) where metals not on screen will be stored 
    unused_metals: HashMap<String, Gd<MetalBar>>,      
}

/// Methods that belong to MetalReserveBarManager
#[godot_api]
impl IVBoxContainer for  MetalReserveBarManager {    
    /// Initialization method for `MetalReserveBarManager`
    fn init(base: Base<VBoxContainer>) -> Self {
        Self {
            base,
            unused_metals: HashMap::new(),
        }
    }       

    /// Creates and sets up the bars inside the Metal Reserve Bar Manager 
    fn ready(&mut self) { 
        // Create Metals that are auto added to VBox based on the keybound metals 
        for i in 0..TOTAL_BARS {
            let mut bar = MetalBar::new_alloc();
            bar.set_visible(true);
            self.base_mut().add_child(bar);
            
            godot_print!("BARS CREATED")
        }
        self.setup_metals();
        self.setup_keybinds();
    }

}

impl MetalReserveBarManager{
    fn setup_keybinds(&mut self) {
        let mut input_map = InputMap::singleton();
        let inputs = input_map.get_actions();

        let length = inputs.len();
        for i in (0..length).rev() {
            let input = inputs.get(i).unwrap();
            let input_str = input.to_string();

            if PATHS.contains(&input_str.as_str())  {
                godot_print!("{}", input_str);
                let events: Array<Gd<godot::classes::InputEvent>> = input_map.action_get_events(
                                                                StringName::from(input_str));
                
                if events.len() == 0 {
                    let bar: Option<Gd<MetalBar>> = self.get_metal_bar(input.to_string().as_str());
                    bar.unwrap().hide();
                }
            }
        }
    }

    pub fn get_metal_bar(&mut self, name: &str) -> Option<Gd<MetalBar>> {
        let children: Array<Gd<Node>> = self.base.to_gd().get_children(); 
        
        for i in 0..children.len() {
            let child : Gd<Node> = children.get(i).expect("");

            if let Ok(bar) = child.try_cast::<MetalBar>() {
                if bar.get_name() == StringName::from(name) {
                    return Some(bar);
                }
            }
        }

        return None;
    }

    /// Sets the name and texture of every Metal Bar 
    fn setup_metals(&mut self) {
        // Get all the children of the player 
        let children: Array<Gd<Node>> = self.base.to_gd().get_children(); 
        

        for i in 0..children.len() {

            let child : Gd<Node> = children.get(i).expect("");
            if let Ok(mut bar) = child.try_cast::<MetalBar>() {
                    let mut bar_mut = bar.bind_mut();
                    bar_mut.set_name(PATHS[i]);
                    bar_mut.set_texture(PATHS[i]);
                    
                } else {
                    godot_print!("Failed to cast node to MetalBar");
                }
    
        }
        godot_print!("BARS SETUP")
    }

    /// Adds metals to all the bars contained within the metal reserve bar manager 
    pub fn add_metals(&mut self){
        let children: Array<Gd<Node>> = self.base.to_gd().get_children(); 
        for i in 0..TOTAL_BARS {
            let child : Gd<Node> = children.get(i.into()).expect("");
            if let Ok(mut bar) = child.try_cast::<MetalBar>() {
                let mut bar_mut = bar.bind_mut();
                bar_mut.adjust_reserves(10.0);
            }
        }
        godot_print!("METALS ADDED")
    }

    // Adds and removes a metal bar from displaying on the screen 
    pub fn add_remove(&mut self, unbind: Gd<MetalBar>, bind: String){
        godot_print!("STUBBED")
    }
}