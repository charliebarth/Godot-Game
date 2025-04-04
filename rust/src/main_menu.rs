use godot::{
    classes::{AnimatedSprite2D, AnimationPlayer, Control, InputEvent, ResourceLoader, RichTextLabel, ShaderMaterial},
    prelude::*,
};

/// The MainMenu class is responsible for managing the main menu UI.
#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct MainMenu {
    /// The base node of the MainMenu.
    base: Base<Node2D>,
    /// A reference to the settings menu.
    settings_menu: Option<Gd<Control>>,
    /// A reference to the UI buttons of the main menu.
    main_menu: Option<Gd<Control>>,
    new_game_menu: Option<Gd<Control>>,
    team_choice_menu: Option<Gd<Control>>,
}

#[godot_api]
impl INode2D for MainMenu {
    /// The Godot constructor for the MainMenu class.
    ///
    /// # Arguments
    /// * `base` - The base node of the MainMenu.
    ///
    /// # Returns
    /// * `MainMenu` - A new instance of the MainMenu class.
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            settings_menu: None,
            main_menu: None,
            new_game_menu: None,
            team_choice_menu: None,
        }
    }


}

#[godot_api]
impl MainMenu {

    
    /// This function reveals the player sprite on the main menu
    /// to show that a player has joined the game.
    ///
    /// # Arguments
    /// * `player_id` - The id of the player that has joined the game.
    #[func]
    pub fn add_player(&self, player_id: i32) {
        let mut player = self.base()
        .get_node_as::<AnimatedSprite2D>(format!("Player{}", player_id).as_str());

        player.set_visible(true);

    }

    /// This function hides the player sprite on the main menu
    /// to show that a player has left the game.
    ///
    /// # Arguments
    /// * `player_id` - The id of the player that has left the game.
    #[func]
    pub fn remove_player(&self, player_id: i32) {
        self.base()
            .get_node_as::<AnimatedSprite2D>(format!("Player{}", player_id).as_str())
            .set_visible(false);
    }

    #[func]
    fn set_player_team(&self, id: i32, blue: bool){
        let mut path = "";
        if blue {
            path = "res://shaders/blue_outline.tres";
        } else {
            path = "res://shaders/red_outline.tres";
        }

        let shader = ResourceLoader::singleton().load(path);
        if let Ok(shader) = shader.unwrap().try_cast::<ShaderMaterial>(){
            let mut player = self.base()
                .get_node_as::<AnimatedSprite2D>(
                    format!("Player{}", id + 1).as_str()
                );

            player.set_material(&shader);
            godot_print!("TRIED TO SET MATERIAL OF {}'s", player.get_name())
        }
    }

    /// This function adds a notification to the notification box.
    /// This is used for error messages such as trying to start a game with an incorrect number of players.
    /// This is also used for victory messages after a game has ended.
    /// The notification will fade out after a few seconds.
    ///
    /// # Arguments
    /// * `msg` - The message to display in the notification box.
    #[func]
    pub fn add_notification(&self, msg: String) {
        let mut notification_box = self.base().get_node_as::<RichTextLabel>("NotificationBox");
        notification_box.clear();
        notification_box.append_text(format!("[center]{}", msg).as_str());

        let mut notification_box_animation =
            notification_box.get_node_as::<AnimationPlayer>("AnimationPlayer");
        notification_box_animation.set_current_animation("fade_out");
        notification_box_animation.play();
    }

    /// This function returns the settings menu node.
    ///
    /// # Returns
    /// * `Control` - The settings menu node.
    fn get_settings_menu(&mut self) -> Gd<Control> {
        if self.settings_menu.is_none() {
            self.settings_menu = Some(self.base().get_node_as::<Control>("Settings"));
        }

        self.settings_menu
            .as_ref()
            .expect("SettingsMenu node not found")
            .clone()
    }

    /// This function returns the main menu node.
    ///
    /// # Returns
    /// * `Control` - The main menu node.
    fn get_main_menu(&mut self) -> Gd<Control> {
        if self.main_menu.is_none() {
            self.main_menu = Some(self.base().get_node_as::<Control>("MainMenuUI"));
        }

        self.main_menu
            .as_ref()
            .expect("MainMenu node not found")
            .clone()
    }

    /// This function returns the main menu node.
    ///
    /// # Returns
    /// * `Control` - The main menu node.
    fn get_new_game_menu(&mut self) -> Gd<Control> {
        if self.new_game_menu.is_none() {
            self.new_game_menu = Some(self.base().get_node_as::<Control>("NewGame"));
        }

        self.new_game_menu
            .as_ref()
            .expect("NewGameMenu node not found")
            .clone()
    }

    /// This function returns the main menu node.
    ///
    /// # Returns
    /// * `Control` - The main menu node.
    fn get_team_choice_menu(&mut self) -> Gd<Control> {
        if self.team_choice_menu.is_none() {
            self.team_choice_menu = Some(self.base().get_node_as::<Control>("TeamChoice"));
        }

        self.team_choice_menu
            .as_ref()
            .expect("TeamChoiceMenu node not found")
            .clone()
    }

    /// This function swaps the main menu with the settings menu.
    #[func]
    pub fn swap_to_settings(&mut self) {
        let mut main_menu = self.get_main_menu();
        let mut settings_menu = self.get_settings_menu();

        main_menu.set_process(false);
        main_menu.set_visible(false);

        settings_menu.set_process(true);
        settings_menu.set_visible(true);

        godot_print!("Swapped to settings")
    }


    /// This function swaps the current menu with the main menu.
    #[func]
    pub fn swap_to_main_menu(&mut self) {
        let mut main_menu = self.get_main_menu();
        let mut new_game_menu = self.get_new_game_menu();
        let mut settings_menu = self.get_settings_menu();

        main_menu.set_process(true);
        main_menu.set_visible(true);

        new_game_menu.set_process(false);
        new_game_menu.set_visible(false);

        settings_menu.set_process(false);
        settings_menu.set_visible(false);
    }



    /// This function swaps the settings menu with the main menu.
    #[func]
    pub fn swap_to_new_game_menu(&mut self) {
        let mut main_menu = self.get_main_menu();
        let mut new_game_menu = self.get_new_game_menu();
        let mut team_choice_menu = self.get_team_choice_menu();

        main_menu.set_process(false);
        main_menu.set_visible(false);

        new_game_menu.set_process(true);
        new_game_menu.set_visible(true);  

        team_choice_menu.set_process(false);
        team_choice_menu.set_visible(false);      

    }

    /// This function swaps the settings menu with the main menu.
    #[func]
    pub fn swap_to_team_choice_menu(&mut self) {
        let mut new_game_menu = self.get_new_game_menu();
        let mut team_choice_menu = self.get_team_choice_menu();

        new_game_menu.set_process(false);
        new_game_menu.set_visible(false);

        team_choice_menu.set_process(true);
        team_choice_menu.set_visible(true);    

        // TODO - needs to change if we ever support more players on local 
        // Makes the on screen players visible in front of the menu UI
        for i in 1..5{
            let mut player = self.base_mut()
            .get_node_as::<AnimatedSprite2D>(format!("Player{}", i).as_str());

            if player.is_visible(){
                player.set_z_index(1);
            }
        }

    }
}
