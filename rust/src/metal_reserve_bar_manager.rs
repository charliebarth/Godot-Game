/// UNFINISHED 
/// 
/// Controls the metal bars on screen, stores bars that are not currently on screen and has methods 
/// to add and remove bars from the on screen Vbox by name. 
/// 
/// Author : Trinity Pittman
/// Version : 09/18/2024

use godot::prelude::*;
use godot::classes::{IVBoxContainer, Node, VBoxContainer};  // Import Node and VBoxContainer
use std::cell::Ref;

const MAX_BARS_ON_SCREEN: u8 = 4;

#[derive(GodotClass)]
#[class(base=VBoxContainer)]
pub struct MetalReserveBarManager {
    base: Base<VBoxContainer>,
    /// data structure (like a list) where metals not on screen will be stored 
    unused_metals: Vec<Node>,      
}



/// where the methods that belong to MetalReserveBarManager will be stored
#[godot_api]
impl IVBoxContainer for  MetalReserveBarManager {    
    // Initialization method for `MetalReserveBarManager`
    fn init(base: Base<VBoxContainer>) -> Self {
        Self {
            base,
            unused_metals: Vec::new(),
        }
    }       

    fn ready(&mut self) { 

    }

}

impl MetalReserveBarManager{

    pub fn remove_metal(&mut self, owner: Ref<VBoxContainer>, bar_name: &str) {
        // If the metal bar isn't found in Godot, show an error message
        if let Some(node) = owner.get_node_as(bar_name) {
            self.unused_metals.push(node.claim());  // Store the removed node in the vector
            owner.remove_child(node);              // Remove the node from VBox
        } else {
            godot_print!("Bar with name '{}' not found", bar_name);
        }
    }

    pub fn add_metal(&mut self, owner: &VBoxContainer, bar_name: &str) {
        let index = self.unused_metals.iter().position(|node| {
            if let Some(node) = unsafe { node.assume_safe() } {
                node.name().to_string() == bar_name
            } else {
                false
            }
        });

        if let Some(index) = index {
            let node = self.unused_metals.remove(index);
            owner.add_child(node);  // Re-add the node to VBox
        } else {
            godot_print!("Bar with name '{}' not found in removed nodes", bar_name);
        }
    }
}

