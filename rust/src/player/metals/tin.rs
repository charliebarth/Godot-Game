/// tin.rs
///
/// This file contains the implementation of the Tin player ability.
/// The Tin ability allows players to see better in the dark.
///
/// Author: Michael Imerman, Charles Barth
/// Version: Spring 2025
use godot::obj::{Gd, GdMut};

use crate::player::enums::metal_type::MetalType;
use crate::player::player::Player;
use crate::player::traits::metal::Metal;

/// The tin player ability.
/// This ability allows the player to see better in the dark.
pub struct Tin {
    // The maximum amount of tin the player can store.
    capacity: f64,
    // The current amount of tin the player has.
    current_reserve: f64,
    // The rate at which the player burns tin.
    burn_rate: f64,
    // The rate at which the player burns tin when using the low burn ability.
    low_burn_rate: f64,
    // A reference to the player.
    player: Gd<Player>,
    // The type of metal.
    metal_type: MetalType,
    // A flag to determine if the player is currently burning tin.
    burning: bool,
    // A flag to determine if the player is currently low burning tin.
    low_burning: bool,
    // The previous amount of tin the player had.
    previous_reserve: f64,
}

impl Tin {
    pub fn new(
        capacity: f64,
        current_reserve: f64,
        burn_rate: f64,
        low_burn_rate: f64,
        player: Gd<Player>,
        metal_type: MetalType,
    ) -> Self {
        Self {
            capacity,
            current_reserve,
            previous_reserve: 0.0,
            burn_rate,
            low_burn_rate,
            player,
            metal_type,
            burning: false,
            low_burning: false,
        }
    }
}

impl Metal for Tin {
    /// The burn function for tin.
    /// This ability will allow players to see easier when the night cycle occurs.
    fn burn(&mut self) {
        return;
    }

    /// The low burn function for tin.
    /// This ability will allow players to see easier when the night cycle occurs, but
    /// not as well as they would if they were burning tin regularly.
    fn low_burn(&mut self) {
        return;
    }

    /// This function will update the total metal reserve for tin.
    ///
    /// # Arguments
    /// * `amount` - The amount to update the reserve by
    fn update_reserve(&mut self, amount: f64) {
        // update the amount of tin the player has
        self.current_reserve += amount;
        // clamp the amount of tin the player has to the capacity
        self.current_reserve = self.current_reserve.clamp(0.0, self.capacity);
    }

    /// This function will get the type of metal.
    ///
    /// # Returns
    /// * `MetalType` - The type of metal.
    fn metal_type(&self) -> MetalType {
        self.metal_type
    }

    /// This function will get the current reserve of the metal.
    ///
    /// # Returns
    /// * `f64` - The current reserve.
    fn current_reserve(&self) -> f64 {
        self.current_reserve
    }

    /// This function will get the burning flag.
    ///
    /// # Returns
    /// * `bool` - The burning flag.
    fn burning(&self) -> bool {
        self.burning
    }

    /// This function will get the low burning flag.
    ///
    /// # Returns
    /// * `bool` - The low burning flag.
    fn low_burning(&self) -> bool {
        self.low_burning
    }

    /// This function will set the burning flag.
    ///
    /// # Arguments
    /// * `burning` - The new value of the burning flag.
    fn set_burning(&mut self, burning: bool) {
        self.burning = burning;

        if self.burning {
            self.player.bind_mut().emit_tin_signal(10.0, 3.0);
            //add to the player's active metals
            let mut player = self.player.bind_mut();
            player.add_active_metal(self.metal_type);
        }

        if !self.burning && !self.low_burning {
            //remove the metal from the player's active metals
            let mut player = self.player.bind_mut();
            player.remove_active_metal(self.metal_type);
        }
    }
    fn set_low_burning(&mut self, low_burning: bool) {
        self.low_burning = low_burning;

        if self.low_burning {
            self.player.bind_mut().emit_tin_signal(5.0, 3.0);
            //add to the player's active metals
            let mut player = self.player.bind_mut();
            player.add_active_metal(self.metal_type);
        }

        if !self.low_burning && !self.burning {
            //remove the metal from the player's active metals
            let mut player = self.player.bind_mut();
            player.remove_active_metal(self.metal_type);
        }
    }

    /// This function will get the player.
    ///
    /// # Returns
    /// * `GdMut<Player>` - The player.
    fn get_player(&mut self) -> GdMut<'_, Player> {
        self.player.bind_mut()
    }

    /// This function will get the previous reserve.
    ///
    /// # Returns
    /// * `f64` - The previous reserve.
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
