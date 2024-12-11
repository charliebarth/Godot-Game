/// An enum that represents the different forces that can be applied to the player
/// Forces are used to move the player around the map
#[derive(PartialEq)]
pub enum Force {
    /// Gravity
    /// This force is always applied to the player but will be countered by the normal force when the player is on the floor
    Gravity { acceleration: f64 },
    /// Jump force applied when the player jumps
    Jump { acceleration: f32 },
    /// Normal force applied when the player is on the floor
    /// The normal force is a force that is proportional to gravity and counteracts it's effect
    NormalForce { magnitude: f64 },
    /// Run force applied when the player is on the floor
    Run { acceleration: f32 },
    /// Horizontal force applied when the player is in the air
    AirRun { acceleration: f32 },
    /// Stop all movement of the player based on the boolean values
    Stop { horizontal: bool, vertical: bool },
    /// A force to push or pull a player relative to a metal object
    SteelPush { x_velocity: f32, y_velocity: f32 },
}
