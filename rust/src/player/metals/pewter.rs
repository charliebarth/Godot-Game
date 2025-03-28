use godot::obj::{Gd, GdMut};

use crate::player::enums::force::{ForceModifier, ForceModifierTag};
use crate::player::enums::metal_type::MetalType;
use crate::player::player::Player;
use crate::player::traits::metal::Metal;

/// The pewter player ability.
/// CUrrently this gives the player a speed boost and a jump boost.
/// In the future I would like to reduce these boost and add additional mechanics.
/// Such as the ability to wall jump or chain a landing into a high jump.
pub struct Pewter {
    /// The maximum amount of pewter the player can store.
    capacity: f64,
    /// The current amount of pewter the player has.
    current_reserve: f64,
    /// The rate at which the player burns pewter.
    burn_rate: f64,
    /// The rate at which the player burns pewter when using the low burn ability.
    low_burn_rate: f64,
    /// A reference to the player.
    player: Gd<Player>,
    /// The type of metal.
    metal_type: MetalType,
    /// A flag to determine if the player is currently burning pewter.
    burning: bool,
    /// A flag to determine if the player is currently low burning pewter.
    low_burning: bool,
    /// The previous amount of pewter the player had.
    previous_reserve: f64,
}

impl Pewter {
    /// The constructor for the pewter struct.
    ///
    /// # Arguments
    /// * `capacity` - The maximum amount of pewter the player can store.
    /// * `current_reserve` - The current amount of pewter the player has.
    /// * `burn_rate` - The rate at which the player burns pewter.
    /// * `low_burn_rate` - The rate at which the player burns pewter when using the low burn ability.
    ///
    /// # Returns
    /// * `Pewter` - A new instance of the pewter struct.
    pub fn new(
        capacity: f64,
        current_reserve: f64,
        burn_rate: f64,
        low_burn_rate: f64,
        player: Gd<Player>,
        metal_type: MetalType,
    ) -> Self {
        // player
        //     .bind_mut()
        //     .set_metal_reserve_amount(MetalType::Steel.as_str().into(), current_reserve);

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

    fn adjust_force_modifer(&mut self, modifier: ForceModifier) {
        if self.burning || self.low_burning {
            self.player.bind_mut().replace_force_modifier(modifier);
        } else {
            self.player
                .bind_mut()
                .remove_force_modifier(ForceModifierTag::Pewter);
        }
    }
}

impl Metal for Pewter {
    /// The burn function for pewter.
    /// This function gives the player a large speed boost and a large jump boost.
    ///
    /// # Arguments
    /// * `player` - A mutable reference to the player so that the run speed and jump force can be modified.
    fn burn(&mut self) {
        self.update_reserve(-self.burn_rate);
    }

    /// The low burn function for pewter.
    /// This function gives the player a small speed boost and a small jump boost.
    ///
    /// # Arguments
    /// * `player` - A mutable reference to the player so that the run speed and jump force can be modified.
    fn low_burn(&mut self) {
        self.update_reserve(-self.low_burn_rate);
    }

    fn update_reserve(&mut self, amount: f64) {
        self.current_reserve += amount;
        self.current_reserve = self.current_reserve.clamp(0.0, self.capacity);
    }

    fn metal_type(&self) -> MetalType {
        self.metal_type
    }

    fn current_reserve(&self) -> f64 {
        self.current_reserve
    }

    fn burning(&self) -> bool {
        self.burning
    }

    fn low_burning(&self) -> bool {
        self.low_burning
    }

    fn set_burning(&mut self, burning: bool) {
        self.burning = burning;

        if self.burning {
            self.adjust_force_modifer(ForceModifier::Pewter {
                run_boost: 0.9,
                jump_boost: 0.5,
            });
        } else {
            self.adjust_force_modifer(ForceModifier::Pewter {
                run_boost: 0.5,
                jump_boost: 0.2,
            });
        }
    }

    fn set_low_burning(&mut self, low_burning: bool) {
        self.low_burning = low_burning;

        if self.low_burning {
            self.adjust_force_modifer(ForceModifier::Pewter {
                run_boost: 0.5,
                jump_boost: 0.2,
            });
        } else {
            self.adjust_force_modifer(ForceModifier::Pewter {
                run_boost: 0.9,
                jump_boost: 0.5,
            });
        }
    }

    fn get_player(&mut self) -> GdMut<'_, Player> {
        self.player.bind_mut()
    }

    fn previous_reserve(&self) -> f64 {
        self.previous_reserve
    }

    fn set_previous_reserve(&mut self, amt: f64) {
        self.previous_reserve = amt;
    }
}
