use godot::obj::Gd;

use crate::player::{enums::metal_type::MetalType, input_manager::InputManager};

pub trait Metal {
    fn update(&mut self);
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
    fn update_burn(&mut self, input_manager: &mut Gd<InputManager>);

    /// This function is meant to be called when a metal is pressed or released
    /// It will start or stop the low burn depending on the button state
    ///
    /// # Arguments
    /// * `button_state` - The state of the button
    fn update_low_burn(&mut self, input_manager: &mut Gd<InputManager>);
}
