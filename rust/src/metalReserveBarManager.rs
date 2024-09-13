/// UNFINISHED 
/// 
/// Controls the metal bars on screen, stores bars that are not currently on screen and has methods 
/// to add and remove bars from the on screen Vbox by name. 
/// 
/// Author : Trinity Pittman
/// Version : 09/12/2024

use gdnative::prelude::*;


pub struct MetalReserveBarManager {
    /// data structure (like a list) where metals not on screen will be stored 
    unused_metals: Vec<Ref<Node>>,      
}

/// where the methods that belong to MetalReserveBarManager will be stored
#[methods]
impl MetalReserveBarManager {           
    /// basically a constructor, rust needs the _owner: &Control parameter
    fn new(_owner: &Control) -> Self {   
        MetalReserveBarManager {
            /// an empty list is created to store the off screen metals
            unused_metals: Vec::new(),  x 
        }
    }

    #[export]
    fn remove_metal(&mut self, owner: &Control, bar_name: String) {
        /// If the container isn't found in godot, show an error message
        let vbox = unsafe {      
            owner.get_node_as::<VBoxContainer>("MetalReserveBars").expect("MetalReserveBars not found")
        };

        if let Some(node) = owner.get_node(bar_name) {
            self.unused_metals.push(node.claim());  /// Store the removed node in the vector
            owner.remove_child(node);              /// Remove the node from VBox
        } else {
            godot_print!("Bar with name '{}' not found", bar_name);
        }
    }

    #[export]
    fn add_metal(&mut self, owner: &Control, bar_name: String) {
        let index = self.unused_metals.iter().position(|node| {
            if let Some(node) = unsafe { node.assume_safe() } {
                node.name().to_string() == bar_name
            } else {
                false
            }
        });

        if let Some(index) = index {
            let node = self.unused_metals.remove(index);
            owner.add_child(node, false);  /// Re-add the node to VBox
        } else {
            godot_print!("Bar with name '{}' not found in removed nodes", bar_name);
        }
    }
}