use crate::player::player::Player;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum PlayerState {
    Idle,
    Run,
    Jump,
    Fall,
}

impl PlayerState {
    pub fn enter(&self, player: &mut Player) {
        match self {
            PlayerState::Idle => player_states::enter_idle(player),
            PlayerState::Run => player_states::enter_run(player),
            _ => {}
        }
    }

    pub fn update(&self, player: &mut Player, delta: f64) {
        match self {
            PlayerState::Idle => player_states::update_idle(player, delta),
            PlayerState::Run => player_states::update_run(player, delta),
            _ => {}
        }
    }
}

mod player_states {
    use crate::player::player::Player;

    pub fn enter_idle(player: &mut Player) {
        println!("Entering Idle state");
        // One-time logic for Idle state
    }

    pub fn update_idle(player: &mut Player, delta: f64) {
        println!("Updating Idle state");
        // Update logic for Idle state
    }

    pub fn enter_run(player: &mut Player) {
        println!("Entering Run state");
        // One-time logic for Run state
    }

    pub fn update_run(player: &mut Player, delta: f64) {
        println!("Updating Run state");
        // Update logic for Run state
    }

    // Add similar functions for Jump, Fall, etc.
}
