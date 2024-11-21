/*use crate::player::{player::Player, traits::player_state::PlayerState};

#[derive(Clone, Copy)]
pub struct Attack;

impl PlayerState for Attack {
    fn enter(player: &mut Player) {
        // Set the animation speed for the melee attack
        player.set_animation_speed(1.0);
        // damage the enemy
        /*if let Some(target_player) = player.get_target_player() {
            player.deal_damage(target_player, 10.0); // Deal 10 damage to the target player
        }*/
        
    }

    fn update(player: &mut Player) {
        if player.is_anim_finished() {
            let previous_state = player.get_previous_state();
            player.set_state(previous_state);
        }
    }
}*/