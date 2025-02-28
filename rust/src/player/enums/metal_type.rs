#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum BurnType {
    Burn,
    LowBurn,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum ButtonState {
    Pressed,
    Released,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum MetalType {
    Pewter,
    Steel,
    Iron,
    Tin,
    Copper,
    Bronze,
}

impl MetalType {
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
}
// impl MetalEvents {
//     /// Method to convert from string to the corresponding event
//     ///
//     /// # Arguments
//     ///
//     /// * `button` - The string to convert to a metal event
//     ///
//     /// # Returns
//     ///
//     /// * `Some(MetalEvents)` - The corresponding metal event with the burn BurnType
//     /// * `None` - If the string does not match any metal event
//     pub fn from_string(button: &str) -> Option<MetalEvents> {
//         match button {
//             "pewter" => Some(MetalEvents::Pewter),
//             "steel" => Some(MetalEvents::Steel),
//             "iron" => Some(MetalEvents::Iron),
//             "steel_lowburn" => Some(MetalEvents::SteelLowBurn),
//             "pewter_lowburn" => Some(MetalEvents::PewterLowBurn),
//             "iron_lowburn" => Some(MetalEvents::IronLowBurn),
//             _ => None,
//         }
//     }

//     /// Method to get the low burn variant of the event
//     ///
//     /// # Arguments
//     /// * `event` - The event to get the low burn variant of
//     ///
//     /// # Returns
//     /// * `Some(MetalEvents)` - The low burn variant of the event
//     pub fn get_low_burn_variant(event: MetalEvents) -> Option<MetalEvents> {
//         match event {
//             MetalEvents::Pewter => Some(Self::PewterLowBurn),
//             MetalEvents::Iron => Some(MetalEvents::SteelLowBurn),
//             MetalEvents::Steel => Some(MetalEvents::SteelLowBurn),
//             _ => None,
//         }
//     }

//     pub fn as_str(&self) -> &str {
//         match self {
//             MetalEvents::Pewter => "pewter",
//             MetalEvents::Steel => "steel",
//             MetalEvents::Iron => "iron",
//             _ => "unknown",
//         }
//     }
// }
