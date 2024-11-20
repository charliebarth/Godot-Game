#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum MetalEvents {
    Pewter,
    Steel,
    Iron,
    SteelLowBurn,
    PewterLowBurn,
}

impl MetalEvents {
    /// Method to convert from string to the corresponding event
    ///
    /// # Arguments
    ///
    /// * `button` - The string to convert to a metal event
    ///
    /// # Returns
    ///
    /// * `Some(MetalEvents)` - The corresponding metal event with the burn BurnType
    /// * `None` - If the string does not match any metal event
    pub fn from_string(button: &str) -> Option<MetalEvents> {
        match button {
            "pewter" => Some(MetalEvents::Pewter),
            "steel" => Some(MetalEvents::Steel),
            "iron" => Some(MetalEvents::Iron),
            "steel_lowburn" => Some(MetalEvents::SteelLowBurn),
            _ => None,
        }
    }

    pub fn get_low_burn_variant(event: MetalEvents) -> Option<MetalEvents> {
        match event {
            MetalEvents::Pewter => Some(Self::PewterLowBurn),
            MetalEvents::Iron => Some(MetalEvents::SteelLowBurn),
            MetalEvents::Steel => Some(MetalEvents::SteelLowBurn),
            _ => None,
        }
    }
}
