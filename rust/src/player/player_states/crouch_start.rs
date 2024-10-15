use crate::player::{
    enums::player_states::PlayerStates, player::Player, traits::player_state::PlayerState,
};

#[derive(Clone, Copy)]
pub struct CrouchStart;

impl PlayerState for CrouchStart {
    fn enter(_player: &mut Player) {}

    fn update(player: &mut Player) {
        if player.is_anim_finished() {
            player.set_state(PlayerStates::Crouch);
        }
    }
}
