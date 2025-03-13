use godot::{builtin::Vector2, obj::Gd};

use crate::player::player::Player;

/// An enum that represents the different forces that can be applied to the player
/// Forces are used to move the player around the map
///
/// NOTE:
/// acceleration is meant to be multiplied by delta as it is a force to be applied over multiple frames
/// impulse is not effected by delta because it is a one time occurence
///
/// Jump is an example of both. It has an impulse which is an instant application of force that creates a minimum jump height
/// There is also an acceleration which is effect by gravity because it is applied every frame the player is still holding the jump button.
#[derive(PartialEq)]
pub enum Force {
    /// Gravity
    /// This force is always applied to the player but will be countered by the normal force when the player is on the floor
    Gravity {
        acceleration: f64,
    },
    /// Jump force applied when the player jumps
    Jump {
        /// applied every frame the player holds the jump button
        acceleration: Vector2,
        /// the initial instant upwards force which is only meant to be applied once
        impulse: Vector2,
    },
    /// Normal force applied when the player is on the floor
    /// The normal force is a force that is proportional to gravity and counteracts it's effect
    NormalForce {
        magnitude: f64,
    },
    /// Run force applied when the player is on the floor
    Run {
        acceleration: f32,
    },
    /// Horizontal force applied when the player is in the air
    AirRun {
        acceleration: f32,
    },
    /// Stop all movement of the player based on the boolean values
    Stop {
        horizontal: bool,
        vertical: bool,
    },
    /// A force to push or pull a player relative to a metal object
    SteelPush {
        x_acceleration: f32,
        y_acceleration: f32,
    },
    Friction {
        acceleration: f32,
    },
    AirResistance {
        acceleration: f32,
    },
    PlayerSteelPush {
        acceleration: Vector2,
        player: Gd<Player>,
    },
}
