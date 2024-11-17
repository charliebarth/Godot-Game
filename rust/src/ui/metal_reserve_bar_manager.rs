use std::borrow::BorrowMut;
use std::collections::HashMap;

/// Controls the metal bars on screen, stores bars that are not currently on screen and has methods 
/// to add and remove bars from the on screen Vbox by name. 
/// 
/// Author : Trinity Pittman
/// Version : 11/17/2024

use godot::prelude::*;
use godot::classes::{IVBoxContainer, InputMap, VBoxContainer};  // Import Node and VBoxContainer
pub use crate::ui::metal_bar::MetalBar;


// The maximum number of bars to display on a players screen at a time 
const MAX_BARS_ON_SCREEN: u8 = 4;

// Change this to account for how many we currently support
const TOTAL_BARS: u8 = 10;   

// Represents the order of supported metals (simply reorder these based on implementation)
const PATHS: [&str; 10] = ["iron", "steel", "pewter", "tin", "bronze", "copper", "duralumin", 
                            "nicrosil", "chromium", "gold"];

#[derive(GodotClass)]
#[class(base=VBoxContainer)]
pub struct MetalReserveBarManager {
    base: Base<VBoxContainer>,
    bars: Option<HashMap<StringName, Gd<MetalBar>>>,  
}

/// Methods that belong to MetalReserveBarManager
#[godot_api]
impl IVBoxContainer for  MetalReserveBarManager {    
    /// Initialization method for `MetalReserveBarManager`
    fn init(base: Base<VBoxContainer>) -> Self {
        Self {
            base,
            bars: None,
        }
    }       

    /// Creates and sets up the bars inside the Metal Reserve Bar Manager 
    fn ready(&mut self) { 
        // Create Metals that are auto added to VBox based on the keybound metals 
        for i in 0..TOTAL_BARS {
            let index = i as usize;
            let mut bar: Gd<MetalBar> = MetalBar::new_alloc();
            bar.hide();     // Default hidden
            self.base_mut().add_child(bar.clone());     // Add to VBox

            if let Some(&metal_name_temp) = PATHS.get(index) {
                let metal_name = StringName::from(metal_name_temp);
                bar.set_name(metal_name.clone().to_string().into()); // Set name
                self.get_metal_bar(metal_name.clone()).bind_mut().set_texture(PATHS[index]);
                self.get_bars().insert(metal_name, bar); // Add to HashMap
            }
            
            godot_print!("BARS CREATED")
        }

        self.setup_keybinds();
    }

}

impl MetalReserveBarManager{

    /// Sets the metals currently on screen based on the keybindings set 
    fn setup_keybinds(&mut self) {
        let mut input_map: Gd<InputMap> = InputMap::singleton();
        let inputs: Array<StringName> = input_map.get_actions();

        let length: usize = inputs.len();
        for i in (0..length).rev() {
            let input: StringName = inputs.get(i).unwrap();

            if PATHS.contains(&input.to_string().as_str())  {
                godot_print!("{}", input);
                let events: Array<Gd<godot::classes::InputEvent>> = 
                                                input_map.action_get_events(input.clone());
                
                let mut max = 0;
                // If something is keybound to the event and not reached max metals, show the bar
                if events.len() > 0 && max != MAX_BARS_ON_SCREEN{    
                    let mut bar: Gd<MetalBar> = self.get_metal_bar(input);
                    bar.show();
                    max = max + 1;
                }
            }
        }
    }

    /// Given the name of a metal, gets the metal bar associated with it
    /// 
    /// Args: 
    ///     name (&str): the name of the metal bar to get 
    /// 
    /// Returns: the metal bar or None if none exists 
    pub fn get_metal_bar(&mut self, name: StringName) -> Gd<MetalBar> {
        self.get_bars().get(&name).expect("Bar not found").clone()
        

        // let children: Array<Gd<Node>> = self.base.to_gd().get_children(); 
        
        // for i in 0..children.len() {
        //     let child : Gd<Node> = children.get(i).expect("");

        //     if let Ok(bar) = child.try_cast::<MetalBar>() {
        //         if bar.get_name() == name {
        //             return Some(bar);
        //         }
        //     }
        // }

        // return None;
    }

    fn get_bars(&mut self) -> HashMap<StringName, Gd<MetalBar>> {
        if self.bars.is_none() {
            self.bars = Some(HashMap::new());
        }
        self.bars.as_ref().expect("bars not found").clone()
    }

    /// Adds metals to all the bars contained within the metal reserve bar manager 
    pub fn add_metals(&mut self, metals: &Vec<StringName>, amt: f64){ // if needed 
        for i in 0..self.get_bars().len() {
            // Get the specific bar 
            let mut bar = self.get_metal_bar(StringName::from(PATHS[i as usize]));
            if metals.contains(&bar.get_name()){ // If its one of the metals
                bar.bind_mut().adjust_reserves(amt);    // add metals
                godot_print!("METALS ADDED to {}", bar.get_name())
            }
        }

        // let children: Array<Gd<Node>> = self.base.to_gd().get_children(); 
        // for i in 0..TOTAL_BARS {
        //     let child : Gd<Node> = children.get(i.into()).expect("");
        //     if let Ok(mut bar) = child.try_cast::<MetalBar>() {
        //         let mut bar_mut = bar.bind_mut();
        //         if increase.contains(&bar_mut.get_name()){
        //             bar_mut.adjust_reserves(amt);
        //             godot_print!("METALS ADDED to {}", bar_mut.get_name())
        //         }
        //     }
        // }
        
    }

    // Adds and removes a metal bar from displaying on the screen 
    pub fn add_remove(&mut self, unbind: Gd<MetalBar>, bind: String){
        godot_print!("KEY REBINDINGS - stubbed")
    }
}