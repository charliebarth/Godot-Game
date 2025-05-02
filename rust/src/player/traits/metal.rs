/// metal.rs
/// This file defines a trait for metals in the game. Each metal has a reserve, a type, and can be
/// burned or low burned. The trait provides methods to manage the metal's state and reserve, as
/// well as to interact with the player.
///
/// Author: Charles Barth
/// Version: Spring 2025
use godot::obj::GdMut;

use crate::player::{
    enums::metal_type::{BurnType, ButtonState, MetalType},
    player::Player,
};

pub trait Metal {
    /// This function will use the metal/player ability and
    /// grants full benefits but consume the reserve faster than a low burn
    ///
    /// # Arguments
    /// * `player` - The player physics will be applied to
    fn burn(&mut self);

    /// This function will use the metal/player ability but provides fewer or weaker benefits
    /// than a regular burn but consumes the reserve slower
    ///
    /// # Arguments
    /// * `player` - The player physics will be applied to
    fn low_burn(&mut self);

    /// This function will increase the reserve of the metal
    ///
    /// # Arguments
    /// * `amount` - The amount to increase the reserve by
    fn update_reserve(&mut self, amount: f64);

    fn metal_type(&self) -> MetalType;
    /// This function is meant to be called when a metal is pressed or released
    /// It will start or stop the burn or low burn depending on the button state
    ///
    /// # Arguments
    /// * `button_state` - The state of the button
    fn update_burn(&mut self) {
        let mut input_manager_unbound = self.get_player().get_input_manager();
        let mut input_manager = input_manager_unbound.bind_mut();

        let event_present = input_manager.fetch_metal_event((
            self.metal_type(),
            BurnType::Burn,
            ButtonState::Pressed,
        ));
        drop(input_manager);

        let metal_type = self.metal_type().clone();
        // If the metal is not already burning and the burn button is pressed trigger the start of a burn
        if !self.burning() && self.current_reserve() > 0.0 && event_present {
            self.get_player()
                .get_metal_particles(metal_type)
                .set_visible(true);

            self.set_burning(true);

        // If the metal is already burning and the burn button has been released trigger the end of a burn
        } else if self.burning() && (self.current_reserve() <= 0.0 || !event_present) {
            if !self.low_burning() {
                self.get_player()
                    .get_metal_particles(metal_type)
                    .set_visible(false);
            }

            self.set_burning(false);
        }
    }

    /// This function is meant to be called when a metal is pressed or released
    /// It will start or stop the low burn depending on the button state
    ///
    /// # Arguments
    /// * `button_state` - The state of the button
    fn update_low_burn(&mut self) {
        let mut input_manager_unbound = self.get_player().get_input_manager();
        let mut input_manager = input_manager_unbound.bind_mut();

        let event_present = input_manager.fetch_metal_event((
            self.metal_type(),
            BurnType::LowBurn,
            ButtonState::Pressed,
        ));
        drop(input_manager);

        let metal_type = self.metal_type().clone();
        // If the metal is not already burning and the burn button is pressed trigger the start of a burn
        if !self.low_burning() && self.current_reserve() > 0.0 && event_present {
            self.get_player()
                .get_metal_particles(metal_type)
                .set_visible(true);

            self.set_low_burning(true);

        // If the metal is already burning and the burn button has been released trigger the end of a burn
        } else if self.low_burning() && (self.current_reserve() <= 0.0 || !event_present) {
            if !self.burning() {
                self.get_player()
                    .get_metal_particles(metal_type)
                    .set_visible(false);
            }

            self.set_low_burning(false);
        }
    }

    /// This function gets the current reserve of the metal
    fn current_reserve(&self) -> f64;

    /// This function is used to check if the player is burning a metal
    fn burning(&self) -> bool;

    /// This function is used to check if the player is low burning a metal
    fn low_burning(&self) -> bool;

    /// This function will set the low_burning flag to true or false.
    /// If any low burn cleanup logic is unique to a metal it can be triggered in this method.
    ///
    /// # Arguments
    /// * `low_burning` - The state of if low burning is occurring or not
    fn set_low_burning(&mut self, low_burning: bool);

    /// This function will set the burning flag to true or false.
    /// If any burn cleanup logic is unique to a metal it can be triggered in this method.
    ///
    /// # Arguments
    /// * `burning` - The state of if burning is occurring or not
    fn set_burning(&mut self, burning: bool);

    /// This function will get the player
    ///
    /// # Returns
    /// * `GdMut<'_, Player>` - The player
    fn get_player(&mut self) -> GdMut<'_, Player>;

    /// This function will get the previous reserve of the metal
    ///
    /// # Returns
    /// * `f64` - The previous reserve of the metal
    fn previous_reserve(&self) -> f64;

    /// This function will set the previous reserve of the metal
    ///
    /// # Arguments
    /// * `amt` - The amount to set the previous reserve to
    fn set_previous_reserve(&mut self, amt: f64);
}