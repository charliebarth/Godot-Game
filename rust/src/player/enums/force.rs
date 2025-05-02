// force.rs
//
// This file defines the `Force` enum and its associated methods.
//
// Author: Charles Barth
// Version: Spring 2025
use godot::{builtin::Vector2, obj::Gd};

use crate::player::player::Player;

/// An enum that represents the different forces that can be applied to the player
/// Forces are used to move the player around the map
///
/// NOTE:
/// acceleration is meant to be multiplied by delta as it is a force to be applied over
/// multiple frames impulse is not effected by delta because it is a one time occurrence.
///
/// Jump is an example of both. It has an impulse which is an instant application of force that
/// creates a minimum jump height.
/// There is also an acceleration which is effect by gravity because it is applied every frame the
/// player is still holding the jump button.
#[derive(PartialEq)]
pub enum Force {
    /// Gravity
    /// This force is always applied to the player but will be countered by the normal force when
    /// the player is on the floor.
    Gravity {
        acceleration: f64,
    },
    /// Jump force applied when the player jumps
    Jump {
        /// applied every frame the player holds the jump button
        acceleration: f32,
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
    /// A force to apply to a player when they are on the floor
    Friction {
        acceleration: f32,
    },
    /// A force to apply to a player when they are in the air
    AirResistance {
        acceleration: f32,
    },
    PlayerSteelPush {
        acceleration: Vector2,
        player: Gd<Player>,
    },
    Impact {
        acceleration: Vector2,
    },
}

/// These are modifiers which will be applied to incoming player forces before they are actually
/// applied to the player themselves.
/// For instance a pewter modifier will increase any run forces and jump forces by some percentage.
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum ForceModifierTag {
    Pewter,
}

#[derive(Debug)]
pub enum ForceModifier {
    Pewter { run_boost: f64, jump_boost: f64 },
}

impl ForceModifier {
    pub fn tag(&self) -> ForceModifierTag {
        match self {
            ForceModifier::Pewter { .. } => ForceModifierTag::Pewter,
        }
    }

    pub fn combine_modifiers(&self, other: ForceModifier) -> ForceModifier {
        match (self, other) {
            (
                ForceModifier::Pewter {
                    run_boost: a_run,
                    jump_boost: a_jump,
                },
                ForceModifier::Pewter {
                    run_boost: b_run,
                    jump_boost: b_jump,
                },
            ) => ForceModifier::Pewter {
                run_boost: 1.0 - (1.0 - a_run) * (1.0 - b_run),
                jump_boost: 1.0 - (1.0 - a_jump) * (1.0 - b_jump),
            },
        }
    }
}
