use crate::player::{player::Player, traits::player_state::PlayerState};

#[derive(Clone, Copy)]
pub struct Attack;

impl PlayerState for Attack {
    fn enter(player: &mut Player) {
        // Set the animation speed for the melee attack
        player.set_animation_speed(1.0);
        // damage the enemy
        player.deal_damage();
        
    }

    fn update(player: &mut Player) {
        if player.is_anim_finished() {
            let previous_state = player.get_previous_state();
            player.set_state(previous_state);
        }
    }
}