use godot::obj::Gd;
use std::ffi::c_void;
//use std::os::unix::raw::gid_t;

use crate::player::enums::metal_type::{BurnType, ButtonState, MetalType};
use crate::player::input_manager::InputManager;
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

    /// Function that updates if the player is burning tin
    fn cleanup_burn(&mut self) {
        self.burning = false;
    }

    /// Function that update if the player is low burning tin
    fn cleanup_lowburn(&mut self) {
        self.low_burning = false;
        self.player.bind_mut().get_tin_particles().set_visible(false);
    }
}

impl Metal for Tin {
    /// The update function for tin.
    /// This function checks to see if the input manager has a tin event.
    /// If the event is found then the burn function is called.
    /// If the low burn variant is found then the low burn function is called.
    /// This will also toggle the tin particles on and off.
    fn update(&mut self) {
        let mut input_manager = self.player.bind_mut().get_input_manager();
        self.update_burn(&mut input_manager);
        self.update_low_burn(&mut input_manager);
        if self.current_reserve <= 0.0 {
            self.cleanup_burn();
            self.cleanup_lowburn();
        } else if self.burning {
            self.update_reserve(-self.burn_rate);
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

    /// The burn function for tin.
    /// This ability will allow players to see easier when the night cycle occurs.
    fn burn(&mut self) {
        // Emit the signal to do a regular burn
        self.player.bind_mut().emit_tin_signal(10.0, 3.0);
    }

    /// The low burn function for tin.
    /// This ability will allow players to see easier when the night cycle occurs, but
    /// not as well as they would if they were burning tin regularly.
    fn low_burn(&mut self) {
        self.player.bind_mut().get_tin_particles().set_visible(true);
        // Emit the signal to do a low burn
        self.player.bind_mut().emit_tin_signal(5.0, 3.0);
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

    fn metal_type(&self) -> MetalType {
        self.metal_type
    }

    /// Updates the value for if the player is low buring tin.
    ///
    /// # Arguments
    /// * `input_manager` - The input manager to check for the event of a pressed button
    fn update_low_burn(&mut self, input_manager: &mut Gd<InputManager>) {
        let mut input_manager = input_manager.bind_mut();
        let burn_type = BurnType::LowBurn;

        if !self.low_burning
            && input_manager.fetch_metal_event((self.metal_type, burn_type, ButtonState::Pressed))
        {
            self.low_burn();
            self.low_burning = true;
        } else if self.low_burning
            && input_manager.fetch_metal_event((self.metal_type, burn_type, ButtonState::Released))
        {
            self.cleanup_lowburn();
        }
    }

    /// Updates the value for if the player is burning tin.
    ///
    /// # Arguments
    /// * `input_manager` - The input manager that checks for events
    fn update_burn(&mut self, input_manager: &mut Gd<InputManager>) {
        let mut input_manager = input_manager.bind_mut();
        let burn_type = BurnType::Burn;

        if !self.burning
            && input_manager.fetch_metal_event((self.metal_type, burn_type, ButtonState::Pressed))
        {
            self.burn();
            self.burning = true;
        } else if self.burning
            && input_manager.fetch_metal_event((self.metal_type, burn_type, ButtonState::Released))
        {
            self.cleanup_burn();
        }
    }
}
