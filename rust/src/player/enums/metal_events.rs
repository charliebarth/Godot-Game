#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]

pub enum BurnType {
    Burn,
    LowBurn,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum MetalEvents {
    Pewter(BurnType),
    Steel(BurnType),
    Iron(BurnType),
    SteelLowBurn(BurnType),
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
            "pewter" => Some(MetalEvents::Pewter(BurnType::Burn)),
            "steel" => Some(MetalEvents::Steel(BurnType::Burn)),
            "iron" => Some(MetalEvents::Iron(BurnType::Burn)),
            "steel_lowburn" => Some(MetalEvents::SteelLowBurn(BurnType::Burn)),
            _ => None,
        }
    }

    /// Returns the burn type of the metal event
    pub fn get_burn_type(&self) -> BurnType {
        match self {
            MetalEvents::Pewter(burn_type) => burn_type.clone(),
            MetalEvents::Steel(burn_type) => burn_type.clone(),
            MetalEvents::Iron(burn_type) => burn_type.clone(),
            MetalEvents::SteelLowBurn(burn_type) => burn_type.clone(),
        }
    }

    /// Returns the low burn variant of the metal event
    pub fn get_low_burn_variant(&self) -> MetalEvents {
        match self {
            MetalEvents::Pewter(BurnType::Burn) => MetalEvents::Pewter(BurnType::LowBurn),
            MetalEvents::Pewter(BurnType::LowBurn) => MetalEvents::Pewter(BurnType::LowBurn),
            MetalEvents::Steel(BurnType::Burn) => MetalEvents::Steel(BurnType::LowBurn),
            MetalEvents::Steel(BurnType::LowBurn) => MetalEvents::Steel(BurnType::LowBurn),
            MetalEvents::Iron(BurnType::Burn) => MetalEvents::Iron(BurnType::LowBurn),
            MetalEvents::Iron(BurnType::LowBurn) => MetalEvents::Iron(BurnType::LowBurn),
            MetalEvents::SteelLowBurn(BurnType::Burn) => {
                MetalEvents::SteelLowBurn(BurnType::LowBurn)
            }
            MetalEvents::SteelLowBurn(BurnType::LowBurn) => {
                MetalEvents::SteelLowBurn(BurnType::LowBurn)
            }
        }
    }
}
