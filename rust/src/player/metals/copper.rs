//! copper.rs
//!
//! This file contains the implementation of the Copper player ability.
//! The Copper ability allows players to mask their particles from other players that are burning
//! Bronze.
//!
//! Author: Michael Imerman, Charles Barth
//! Version: Spring 2025
use godot::obj::{Gd, GdMut};

use crate::player::enums::metal_type::MetalType;
use crate::player::player::Player;
use crate::player::traits::metal::Metal;

/// The Copper player ability.
/// This ability allows players to mask their particles from other players.
/// This ability will counter Bronze, which allows players to see other players' particles.
pub struct Copper {
    /// The maximum amount of copper the player can store.
    capacity: f64,
    /// The current amount of copper the player has.
    current_reserve: f64,
    /// The rate at which the player burns copper when using the low burn ability.
    low_burn_rate: f64,
    /// A flag to determine if the player is low burning.
    low_burning: bool,
    /// A flag to determine if the player is burning.
    burning: bool,
    /// A reference to the player.
    player: Gd<Player>,
    /// The type of metal.
    metal_type: MetalType,
    /// The previous amount of copper the player had.
    previous_reserve: f64,
}

impl Copper {
    /// Creates an instance of the Copper 
    /// # Arguments
    /// * `capacity` - The maxiumum amount of copper the player can store 
    /// * `current_reserve` - The current amount of copper the player has
    /// * `low_burn_rate` - The rate at which the player burns copper when using
    ///                     the low burn ability
    /// * `player` - A reference to the player
    /// * `metal_type` - The type of metal 
    /// 
    /// # Returns
    /// * An instance of Copper class 
    pub fn new(
        capacity: f64,
        current_reserve: f64,
        low_burn_rate: f64,
        player: Gd<Player>,
        metal_type: MetalType,
    ) -> Self {
        Self {
            capacity,
            current_reserve,
            previous_reserve: 0.0,
            low_burn_rate,
            low_burning: false,
            burning: false,
            player,
            metal_type,
        }
    }
}

/// Metal methods for Copper
impl Metal for Copper {
    /// The burn function for copper.
    /// It does the same as low_burn because copper has static performance.
    fn burn(&mut self) {
        return;
    }

    /// The low burn function for copper.
    /// Sets the low_burning flag to true. No particles are shown for copper.
    fn low_burn(&mut self) {
        return;
    }

    /// This function will update the total metal reserve for copper.
    ///
    /// # Arguments
    /// * `amount` - The amount to update the reserve by.
    fn update_reserve(&mut self, amount: f64) {
        // update the amount of tin the player has
        self.current_reserve += amount;
        // clamp the amount of tin the player has to the capacity
        self.current_reserve = self.current_reserve.clamp(0.0, self.capacity);
    }

    /// This function will return the type of metal.
    ///
    /// # Returns
    /// The type of metal.
    fn metal_type(&self) -> MetalType {
        self.metal_type
    }

    /// This function will get the current reserve.
    ///
    /// # Returns
    /// The current reserve.
    fn current_reserve(&self) -> f64 {
        self.current_reserve
    }

    /// This function will get the burning flag.
    ///
    /// # Returns
    /// The burning flag.
    fn burning(&self) -> bool {
        self.burning
    }

    /// This function will get the low burning flag.
    ///
    /// # Returns
    /// The low burning flag.
    fn low_burning(&self) -> bool {
        self.low_burning
    }

    /// This function will set the burning flag.
    ///
    /// # Arguments
    /// * `burning` - The new value of the burning flag.
    fn set_burning(&mut self, burning: bool) {
        self.burning = burning;

        // check if the player is burning or low burning and add to active metals if they are
        if self.burning || self.low_burning {
            // add the metal to the player's active metals
            let mut player = self.player.bind_mut();
            player.add_active_metal(self.metal_type);
        }

        if !self.burning && !self.low_burning {
            // remove the metal from the player's active metals
            let mut player = self.player.bind_mut();
            player.remove_active_metal(self.metal_type);
        }
    }

    /// This function will set the low burning flag.
    ///
    /// # Arguments
    /// * `low_burning` - The new value of the low burning flag.
    fn set_low_burning(&mut self, low_burning: bool) {
        self.low_burning = low_burning;

        // check if the player is burning or low burning and add to active metals if they are
        if self.burning || self.low_burning {
            // add the metal to the player's active metals
            let mut player = self.player.bind_mut();
            player.add_active_metal(self.metal_type);
        }

        if !self.low_burning && !self.burning {
            // remove the metal from the player's active metals
            let mut player = self.player.bind_mut();
            player.remove_active_metal(self.metal_type);
        }
    }

    /// This function will get the player.
    ///
    /// # Returns
    /// The player.
    fn get_player(&mut self) -> GdMut<'_, Player> {
        self.player.bind_mut()
    }

    /// This function will get the previous reserve.
    ///
    /// # Returns
    /// The previous reserve.
    fn previous_reserve(&self) -> f64 {
        self.previous_reserve
    }

    /// This function will set the previous reserve.
    ///
    /// # Arguments
    /// * `amt` - The amount to set the previous reserve to.
    fn set_previous_reserve(&mut self, amt: f64) {
        self.previous_reserve = amt;
    }
}
