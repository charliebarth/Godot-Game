use std::collections::HashMap;

use super::{
    enums::metal_type::{BurnType, ButtonState, MetalType},
    metals::steel::Steel,
    player::Player,
    traits::metal::Metal,
};
use godot::prelude::*;

/// The metal manager is responsible for managing the metals that the player has access to.
/// It creates the metals and assigns them to the player based on the game mode.
/// It also updates the metals every frame.

pub struct MetalManager {
    /// The metals that the player has access to.
    metals: HashMap<MetalType, Box<dyn Metal>>,
    /// The player that the metal manager is attached to.
    player: Option<Gd<Player>>,
}

// #[godot_api]
// impl INode2D for MetalManager {
//     /// The Godot constructor for the MetalManager class.
//     ///
//     /// # Arguments
//     /// * `base` - The base node of the MetalManager.
//     ///
//     /// # Returns
//     /// * `MetalManager` - A new instance of the MetalManager class.
//     fn init(base: Base<Node2D>) -> Self {
//         Self {
//             base,
//             metals: Vec::new(),
//             input_manager: None,
//             player: None,
//         }
//     }

//     fn ready(&mut self) {
//         let player_node = self.base().get_parent().expect("parent not found");
//         let mut player = player_node.try_cast::<Player>().expect("player not found");

//         let input_manager = player.bind_mut().get_input_manager();

//         self.player = Some(player);
//         self.input_manager = Some(input_manager);
//     }

//     fn process(&mut self, _delta: f64) {
//         // if input manager has low burn then start low burning which should trigger one time logic and then every frame continue low burning
//         // if input manager has burn then start burning which should trigger one time logic and then every frame continue burning

//         // if the input manager has stop low burn then stop low burning which includes any cleanup logic
//         // if the input manager has stop burn then stop burning which includes any cleanup logic

//         let mut input_manager_unbound = self
//             .input_manager
//             .as_ref()
//             .expect("input manager not found")
//             .clone();
//         let mut input_manager = input_manager_unbound.bind_mut();

//         for metal in &mut self.metals {
//             // Start and stop logic for the burn and low burn
//             // fetch_metal_event will return true if the event has been triggered and will remove the event from the input manager
//             if input_manager.fetch_metal_event(
//                 metal.metal_type(),
//                 BurnType::Burn,
//                 ButtonState::Pressed,
//             ) {
//                 metal.as_mut().start_burn();
//             }
//             if input_manager.fetch_metal_event(
//                 metal.metal_type(),
//                 BurnType::Burn,
//                 ButtonState::Released,
//             ) {
//                 metal.as_mut().stop_burn();
//             }
//             if input_manager.fetch_metal_event(
//                 metal.metal_type(),
//                 BurnType::LowBurn,
//                 ButtonState::Pressed,
//             ) {
//                 metal.as_mut().start_low_burn();
//             }
//             if input_manager.fetch_metal_event(
//                 metal.metal_type(),
//                 BurnType::LowBurn,
//                 ButtonState::Released,
//             ) {
//                 metal.as_mut().stop_low_burn();
//             }

//             // Update will trigger any continuous logic for the metal if it is burning or low burning
//             metal.as_mut().update();
//         }
//     }
// }

impl MetalManager {
    /// The constructor for the MetalManager class.
    pub fn new() -> Self {
        Self {
            metals: HashMap::new(),
            player: None,
        }
    }

    pub fn set_player(&mut self, player: Gd<Player>) {
        self.player = Some(player);
    }

    /// Assigns the starting metals to the player based on the game mode.
    /// The match statement will be expanded in the future to include more game modes.
    ///
    /// # Arguments
    /// * `game_mode` - The game mode that the player is playing.
    pub fn assign_starting_metals(&mut self, game_mode: &str) {
        match game_mode {
            "last_player_standing" => self.last_player_standing(),
            _ => {}
        }
    }

    /// Assigns the metals for the last player standing game mode.
    fn last_player_standing(&mut self) {
        // self.metals
        //     .push(Box::new(Pewter::new(100.0, 100.0, 0.05, 0.01)));
        self.metals.insert(
            MetalType::Steel,
            Box::new(Steel::new(self.player.as_ref().unwrap().clone())),
        );
    }

    /// Updates every metal that the player has access to.
    pub fn update_metals(&mut self) {
        for metal in &mut self.metals {
            metal.1.as_mut().update();
        }
    }

    /// Increases the reserve of a specific metal by the given amount.
    ///
    /// # Arguments
    /// * `metal` - The name of the metal to increase the reserve of.
    /// * `amount` - The amount to increase the reserve by.
    pub fn increase_metal_reserve(&mut self, metal: StringName, amount: f64) {
        // for m in &mut self.metals {
        //     if m.as_ref().as_str() == metal.to_string() {
        //         m.as_mut().increase_reserve(amount);
        //     }
        // }
    }

    pub fn update_metal(&mut self, metal_event: (MetalType, BurnType), button_state: ButtonState) {
        let (metal_type, burn_type) = metal_event;
        let metal_option = self.metals.get_mut(&metal_type);
        if metal_option.is_none() {
            return;
        }

        let metal = metal_option.unwrap();
        if burn_type == BurnType::Burn {
            metal.update_burn(button_state);
        } else if burn_type == BurnType::LowBurn {
            metal.update_low_burn(button_state);
        }
    }
}
