#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum MetalEvents {
    Pewter,
}

impl MetalEvents {
    // Method to convert from string to the corresponding event
    pub fn from_string(button: &str) -> Option<MetalEvents> {
        match button {
            "pewter" => Some(MetalEvents::Pewter),
            _ => None,
        }
    }
}
