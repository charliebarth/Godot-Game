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
