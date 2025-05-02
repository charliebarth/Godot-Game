// metal_type.rs
//
// This file defines the `MetalType` enum and its associated methods.
//
// Author: Charles Barth, Michael Imerman
// Version: Spring 2025
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum BurnType {
    /// A player is using a metal with its more intense affect
    Burn,
    /// A player is using a metal with its less intense affect
    LowBurn,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum ButtonState {
    /// The button is currently pressed
    Pressed,
    /// The button is currently released
    Released,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum MetalType {
    /// Pewter: A type of metal that is used to increase the player's speed and jump height.
    Pewter,
    /// Steel: A type of metal that is used to push on metal objects.
    Steel,
    /// Iron: A type of metal that is used to pull on metal objects.
    Iron,
    /// Tin: A type of metal that is used to increase the player's visibility in dark areas.
    Tin,
    /// Copper: A type of metal that is used to hide a player's particles.
    Copper,
    /// Bronze: A type of metal that is used to see another player's particles.
    Bronze,
}

impl MetalType {
    /// Converts a string to the corresponding metal type.
    ///
    /// # Arguments
    /// * `button` - The string to convert to a metal type.
    pub fn from_string(button: &str) -> Option<MetalType> {
        match button {
            "pewter" => Some(MetalType::Pewter),
            "steel" => Some(MetalType::Steel),
            "iron" => Some(MetalType::Iron),
            "tin" => Some(MetalType::Tin),
            "copper" => Some(MetalType::Copper),
            "bronze" => Some(MetalType::Bronze),
            _ => None,
        }
    }

    /// Converts a metal type to a string.
    ///
    /// # Returns
    /// * `&str` - The string representation of the metal type.
    pub fn as_str(&self) -> &str {
        match self {
            MetalType::Pewter => "pewter",
            MetalType::Steel => "steel",
            MetalType::Iron => "iron",
            MetalType::Tin => "tin",
            MetalType::Copper => "copper",
            MetalType::Bronze => "bronze",
        }
    }

    /// Converts the metals into an iterable array.
    ///
    /// # Returns
    /// * `impl Iterator<Item = MetalType>` - An iterator over the metal types.
    pub fn iter() -> impl Iterator<Item = MetalType> {
        [
            MetalType::Pewter,
            MetalType::Steel,
            MetalType::Iron,
            MetalType::Tin,
            MetalType::Copper,
            MetalType::Bronze,
        ]
        .into_iter()
    }
}