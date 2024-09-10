pub trait PlayerState {
    fn start();
    // Note: I am planning for this to be called every physics frame the player is in this state.
    // I do not like the current name and will rename when I come up with a more apt name for
    // the method that happens when the player is already in the state.
    fn trigger();
}
