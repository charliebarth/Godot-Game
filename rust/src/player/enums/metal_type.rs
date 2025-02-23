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
}

impl MetalType {
    pub fn from_string(button: &str) -> Option<MetalType> {
        match button {
            "pewter" => Some(MetalType::Pewter),
            "steel" => Some(MetalType::Steel),
            "iron" => Some(MetalType::Iron),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            MetalType::Pewter => "pewter",
            MetalType::Steel => "steel",
            MetalType::Iron => "iron",
        }
    }
}
