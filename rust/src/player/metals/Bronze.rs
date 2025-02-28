use godot::obj::Gd;
use std::ffi::c_void;

use crate::player::enums::metal_type::{BurnType, ButtonState, MetalType};
use crate::player::input_manager::InputManager;
use crate::player::player::Player;
use crate::player::traits::metal::Metal;

/// The Bronze player ability.
/// This ability allows players to view the particles that other players emit when they low burn
/// metals, given that the other player is not burning copper.
pub struct Bronze {
    /// The maximum amount of bronze the player can store.
    capacity: f64,
    /// The current amount of bronze the player has.
    current_reserve: f64,
    /// The rate at which the player burns bronze when using the low burn ability.
    low_burn_rate: f64,
    /// A flag to determine if the player is low burning.
    low_burning: bool,
    /// A flag to determine if the player is burning.
    burning: bool,
    /// A reference to the player.
    player: Gd<Player>,
    /// The type of metal.
    metal_type: MetalType,
    /// The previous amount of bronze the player had.
    previous_reserve: f64,
}

impl Bronze {
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
            previous_reserve,
            low_burn_rate,
            low_burning: false,
            burning: false,
            player,
            metal_type,
        }
    }

    /// Function that updates if the player is low burning bronze
    pub fn cleanup_lowburn(&mut self) {
        self.low_burning = false;
        self.player.bind_mut().get_bronze_particles().set_visible(false);
    }
}


impl Metal for Bronze {
    /// The update function for bronze.
    /// This function checks to see if the input manager has a copper event.
    /// If the event is found, the low burn function is called.
    /// Will also toggle copper particles on and off.
    fn update(&mut self) {
        let mut input_manager = self.player.bind_mut().get_input_manager();
        self.update_burn(&mut input_manager);
        self.update_low_burn(&mut input_manager);
        if self.current_reserve <= 0.0 {
            // self.cleanup_burn();
            self.cleanup_lowburn();
        } else if self.low_burning {
            self.update_reserve(-self.low_burn_rate);
        }

        if self.current_reserve != self.previous_reserve {
            self.player
                .bind_mut()
                .set_metal_reserve_amount(self.metal_type.as_str(), self.current_reserve);
        }
        self.previous_reserve = self.current_reserve;
    }

    /// The burn function for bronze.
    /// It does the same as low_burn because copper has static performance.
    fn burn(&mut self) {
        self.low_burning = true;
        self.player.bind_mut().get_bronze_particles().set_visible(true);
    }

    /// The low burn function for bronze.
    /// Sets the low_burning flag to true and shows the copper particles.
    fn low_burn(&mut self) {
        self.low_burning = true;
        self.player.bind_mut().get_bronze_particles().set_visible(true);
    }

    fn update_reserve(&mut self, amount: f64) {
        todo!()
    }

    fn metal_type(&self) -> MetalType {
        todo!()
    }

    fn update_low_burn(&mut self, input_manager: &mut Gd<InputManager>) {
        todo!()
    }

    fn update_burn(&mut self, input_manager: &mut Gd<InputManager>) {
        self.update_low_burn(input_manager);
    }

}


