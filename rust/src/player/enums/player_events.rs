//! player_events.rs
//!
//! This file defines the `PlayerEvents` enum and its associated methods.
//!
//! Author: Charles Barth, Michael Imerman
//! Version: Spring 2025

use godot::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
/// This enum is used to determine whether an event registered by the
/// InputManager should be triggered on press or release.
pub enum TriggerEvents {
    /// The event should be triggered on press.
    OnPress,
    /// The event should be triggered on release.
    OnRelease,
}

/// Methods for TriggerEvents
impl TriggerEvents {
    /// Returns the corresponding trigger event for a given player event.
    /// This is used to determine whether a player event should be recorded on
    /// press or release.
    ///
    /// # Arguments
    /// * `event` - The player event to get the corresponding trigger event for.
    ///
    /// # Returns
    /// * `TriggerEvents` - The corresponding trigger event for the player event.
    pub fn trigger_for_player_event(event: PlayerEvents) -> TriggerEvents {
        match event {
            PlayerEvents::Jump => TriggerEvents::OnPress,
            PlayerEvents::Crouch => TriggerEvents::OnRelease,
            PlayerEvents::Roll => TriggerEvents::OnRelease,
            PlayerEvents::Sprint => TriggerEvents::OnPress,
            PlayerEvents::Attack => TriggerEvents::OnPress,
            PlayerEvents::LowBurn => TriggerEvents::OnPress,
            PlayerEvents::Die => TriggerEvents::OnPress,
            PlayerEvents::MetalWheel => TriggerEvents::OnPress,
            PlayerEvents::Throw => TriggerEvents::OnPress,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, GodotConvert, Var, Export)]
#[godot(via = GString)]
/// This enum defines the valid player events and is used to identify events in
/// the InputManager.
pub enum PlayerEvents {
    /// The player has jumped.
    Jump,
    /// The player has crouched.
    Crouch,
    /// The player has rolled.
    Roll,
    /// The player has sprinted.
    Sprint,
    /// The player has attacked.
    Attack,
    /// This is a modifier key on the controller.
    /// If button is held/the event is registered and a metal event is
    /// triggered, the metal event will be triggered with the LowBurn variant.
    LowBurn,
    /// The player has died or should die.
    Die,
    /// The player has opened the Metal Selector Wheel
    MetalWheel,
    /// The player has thrown a coin.
    Throw,
}

/// Methods for the PlayerEvents
impl PlayerEvents {
    /// Converts a string to the corresponding player event.
    ///
    /// # Arguments
    /// * `button` - The string to convert to a player event.
    ///
    /// # Returns
    /// * `Some(PlayerEvents)` - The corresponding player event.
    pub fn from_string(button: &str) -> Option<PlayerEvents> {
        match button {
            "jump" => Some(PlayerEvents::Jump),
            "sprint" => Some(PlayerEvents::Sprint),
            "roll" => Some(PlayerEvents::Roll),
            "attack" => Some(PlayerEvents::Attack),
            "low_burn" => Some(PlayerEvents::LowBurn),
            "die" => Some(PlayerEvents::Die),
            "metal_selector" => Some(PlayerEvents::MetalWheel),
            "throw" => Some(PlayerEvents::Throw),
            _ => None,
        }
    }

    /// This determines the number of frames a player event should be held in a
    /// vec of registered events before expiring and being removed.
    /// -1 means the event should persist until the button is released
    ///
    /// # Returns
    /// * `i8` - The number of frames the event should be held in a vec of
    ///          registered events before expiring and being removed.
    pub fn timeout(&self) -> i8 {
        match self {
            PlayerEvents::Jump => 12,
            PlayerEvents::Crouch => 10,
            PlayerEvents::Roll => 10,
            PlayerEvents::Sprint => 10,
            PlayerEvents::Attack => 10,
            PlayerEvents::LowBurn => -1,
            PlayerEvents::Die => 10,
            PlayerEvents::MetalWheel => -1,
            PlayerEvents::Throw => -1,
        }
    }
}
