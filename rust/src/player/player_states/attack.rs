use crate::player::{player::Player, traits::player_state::PlayerState};

#[derive(Clone, Copy)]
pub struct Attack;

impl PlayerState for Attack {
    fn enter(player: &mut Player) {
        // Set the animation speed for the melee attack TODO: Create an animation for melee attack
        // player.set_animation_speed(1.0);
        // Enable the hitbox of the player while they are attacking
        player.enable_hitbox();
        
    }

    fn update(player: &mut Player) {
        if player.is_anim_finished() {
            let previous_state = player.get_previous_state();
            player.set_state(previous_state);
            // Disable the hitbox of the player when the attack animation is finished
            player.disable_hitbox();
        }
    }
}