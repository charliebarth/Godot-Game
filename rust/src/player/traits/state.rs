/// Charles Barth
/// CS 495 Capstone 1
///
/// A trait defining what methods all player states must have.
pub trait State {
    /// Called when first transitioning to the player state. This will trigger any
    /// associated actions such as applying to physics to make the player jump.
    /// This method can contain one time logic that shouldn't trigger every frame
    /// the player spends in this state.
    fn enter();

    /// Called every frame after starting the current state. This will contain the logic and
    /// checks/cases needed to transition to other states.
    fn update();
}
