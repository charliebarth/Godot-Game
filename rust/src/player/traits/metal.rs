use crate::player::player::Player;

pub trait Metal {
    /// This function is called every frame
    /// It will trigger the burn and low burn functions if the appropriate events are in the input manager
    ///
    /// # Arguments
    /// * `player` - Physics will be applied to this player
    fn update(&mut self, player: &mut Player);

    /// This function will use the metal/player ability and
    /// grants full benefits but consume the reserve faster than a low burn
    ///
    /// # Arguments
    /// * `player` - The player physics will be applied to
    fn burn(&mut self, player: &mut Player);

    /// This function will use the metal/player ability but provides fewer or weaker benefits
    /// than a regular burn but consumes the reserve slower
    ///
    /// # Arguments
    /// * `player` - The player physics will be applied to
    fn low_burn(&mut self, player: &mut Player);

    /// This function just returns a string representation of the metal (the name)
    /// This is used to updated metal reserve bars in the player's UI
    ///
    /// # Returns
    /// * `&str` - The name of the metal
    fn as_str(&self) -> &str;

    /// This function will increase the reserve of the metal
    ///
    /// # Arguments
    /// * `amount` - The amount to increase the reserve by
    fn increase_reserve(&mut self, amount: f64);
}
