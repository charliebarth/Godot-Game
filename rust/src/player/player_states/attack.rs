use crate::player::{player::Player, traits::player_state::PlayerState};

#[derive(Clone, Copy)]
pub struct Attack;

impl PlayerState for Attack {
    /// The player is now attacking. Enable the hitbox for the melee attack
    ///
    /// # Arguments
    /// * `player` - The player object that is attacking
    fn enter(player: &mut Player) {
        // Enable the hitbox of the player while they are attacking
        player.enable_hitbox();
    }

    /// On every frame, check if the attack animation is finished.
    /// If it is, disable the hitbox and set the player's state to the previous state.
    ///
    /// # Arguments
    /// * `player` - The player object that is attacking
    fn update(player: &mut Player) {
        if player.is_anim_finished() {
            // get the previous state of the player
            let previous_state = player.get_previous_state();
            // Disable the hitbox of the player when the attack animation is finished
            player.disable_hitbox();
            // Set the player's state to the previous state
            player.set_state(previous_state);
        }
    }
}
