//! split_screen.rs
//!
//! This file contains the implementation of the SplitScreen class, which is responsible for
//! creating and managing a split screen view in the game. It handles the setup of two
//! viewports and their respective containers, as well as the addition of levels and players
//! to the split screen.
//!
//! Author: Charles Barth
//! Version: Spring 2025
use godot::{
    classes::{
        viewport::DefaultCanvasItemTextureFilter, HBoxContainer, IHBoxContainer, SubViewport,
        SubViewportContainer, World2D,
    },
    prelude::*,
};

use crate::{map::Map, player::player::Player};

/// SplitScreen class, responsible for creating/managing split screens 
#[derive(GodotClass)]
#[class(base=HBoxContainer)]
pub struct SplitScreen {
    /// The base HBoxContainer
    base: Base<HBoxContainer>,
    /// The two containers
    containers: [Gd<SubViewportContainer>; 2],
    /// The two viewports
    viewports: [Gd<SubViewport>; 2],
}

/// IHBoxContainer methods for the SplitScreen
#[godot_api]
impl IHBoxContainer for SplitScreen {
    /// The Godot constructor for the SplitScreen class node
    ///
    /// # Arguments
    /// * `base` - The base node type for the SplitScreen
    ///
    /// # Returns
    /// * An instance of the SplitScreen
    fn init(base: Base<HBoxContainer>) -> Self {
        Self {
            base,
            containers: [
                SubViewportContainer::new_alloc(),
                SubViewportContainer::new_alloc(),
            ],
            viewports: [SubViewport::new_alloc(), SubViewport::new_alloc()],
        }
    }

    /// The Godot ready function for the SplitScreen class node
    /// This runs when the node first enters the scene tree
    fn ready(&mut self) {
        // Setup the structure for two split screen views in a HBoxContainer
        let mut container1 = self.containers[0].clone();
        let mut container2 = self.containers[1].clone();
        self.base_mut().add_child(&container1);
        self.base_mut().add_child(&container2);
        container1.add_child(&self.viewports[0]);
        container2.add_child(&self.viewports[1]);

        // set self and all children to size 0, 0
        self.base_mut().set_size(Vector2::new(0.0, 0.0));
        self.set_container_sizes(vec![Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0)]);
        self.set_viewport_sizes(vec![Vector2i::new(0, 0), Vector2i::new(0, 0)]);

        // Set the canvas type for the viewports to nearest
        self.viewports[0]
            .set_default_canvas_item_texture_filter(DefaultCanvasItemTextureFilter::NEAREST);
        self.viewports[1]
            .set_default_canvas_item_texture_filter(DefaultCanvasItemTextureFilter::NEAREST);
    }
}

/// Methods for the SplitScreen
#[godot_api]
impl SplitScreen {
    #[func]
    /// Set the size of the containers
    ///
    /// # Arguments
    ///
    /// * `sizes` - A vector of Vector2 values representing the size of each container
    pub fn set_container_sizes(&mut self, sizes: Vec<Vector2>) {
        for (i, size) in sizes.iter().enumerate() {
            self.containers[i].set_size(*size);
        }
    }

    
    /// Set the size of the viewports
    ///
    /// # Arguments
    ///
    /// * `sizes` - A vector of Vector2i values representing the size of each viewport
    #[func]
    pub fn set_viewport_sizes(&mut self, sizes: Vec<Vector2i>) {
        for (i, size) in sizes.iter().enumerate() {
            self.viewports[i].set_size(*size);
        }
    }

    /// Add a level to the split screen
    /// The level will be added to the first viewport and the second viewport will be set to the same world
    ///
    /// # Arguments
    ///
    /// * `level` - A Gd<Map> value representing the level to add
    #[func]
    pub fn add_level(&mut self, level: Gd<Map>) {
        self.viewports[0].add_child(&level);
        let world_2d = self.viewports[0].get_world_2d().expect("World2D not found");
        self.viewports[1].set_world_2d(&world_2d);
    }

    /// Add a level to the split screen
    /// Two levels need to be passed so they can be added to the two viewports
    ///
    /// # Arguments
    /// * `level_one` - A Gd<Map> value representing the first level to add
    /// * `level_two` - A Gd<Map> value representing the second level to add
    #[func]
    pub fn add_levels(&mut self, level_one: Gd<Map>, level_two: Gd<Map>) {
        self.viewports[0].add_child(&level_one);
        self.viewports[1].add_child(&level_two);
    }

    /// Add a world to the split screen
    /// This is used for the second split screen so both viewports will use the world rather than having their own
    /// level added to them.
    ///
    /// # Arguments
    ///
    /// * `world` - A Gd<World2D> value representing the world to add
    #[func]
    pub fn add_world(&mut self, world: Gd<World2D>) {
        self.viewports[0].set_world_2d(&world);
        self.viewports[1].set_world_2d(&world);
    }

    /// Get the world of the split screen
    ///
    /// # Returns
    ///
    /// * A Gd<World2D> value representing the world of the split screen
    #[func]
    pub fn get_world(&self) -> Gd<World2D> {
        self.viewports[1].get_world_2d().expect("World2D not found")
    }

    /// Add players to the split screen
    /// If there is only one player then the first viewport will be used and the same size as the HBoxContainer
    /// If there are two players then the first viewport will be used for the first player and the second viewport will be used for the second player
    /// In this case each viewport will be the full height of the HBoxContainer and the width will be half of the HBoxContainer
    ///
    /// # Arguments
    ///
    /// * `players` - A vector of Gd<Player> values representing the players to add
    #[func]
    pub fn add_players(&mut self, players: Vec<Gd<Player>>) {
        let hbox_size = self.base().get_size();

        match players.len() {
            1 => {
                // Single player - use full viewport size
                self.set_container_sizes(vec![hbox_size, Vector2::new(0.0, 0.0)]);
                self.set_viewport_sizes(vec![
                    Vector2i::new(hbox_size.x as i32, hbox_size.y as i32),
                    Vector2i::new(0, 0),
                ]);
            }
            2 => {
                // Two players - split screen horizontally
                let half_width = hbox_size.x / 2.0;
                let container_size = Vector2::new(half_width, hbox_size.y);
                let viewport_size = Vector2i::new(half_width as i32, hbox_size.y as i32);

                self.set_container_sizes(vec![container_size, container_size]);
                self.set_viewport_sizes(vec![viewport_size, viewport_size]);
            }
            _ => {}
        }

        for (i, player) in players.iter().enumerate() {
            self.viewports[i].set_use_hdr_2d(true);
            self.viewports[i].add_child(player);

            self.viewports[i].set_canvas_cull_mask(
                1 << player.bind().get_player_id() as u32 * 2
                    | 1 << player.bind().get_player_id() as u32 * 2 - 1
                    | 1,
            );
        }
    }

    /// Adds players to the levels of the split screen
    /// This is used to add players to the levels of the split screen
    ///
    /// # Arguments
    ///
    /// * `players` - A vector of Gd<Player> values representing the players to add
    pub fn add_players_to_levels(&mut self, players: Vec<Gd<Player>>) {
        let hbox_size = self.base().get_size();

        match players.len() {
            1 => {
                // Single player - use full viewport size
                self.set_container_sizes(vec![hbox_size, Vector2::new(0.0, 0.0)]);
                self.set_viewport_sizes(vec![
                    Vector2i::new(hbox_size.x as i32, hbox_size.y as i32),
                    Vector2i::new(0, 0),
                ]);
            }
            2 => {
                // Two players - split screen horizontally
                let half_width = hbox_size.x / 2.0;
                let container_size = Vector2::new(half_width, hbox_size.y);
                let viewport_size = Vector2i::new(half_width as i32, hbox_size.y as i32);

                self.set_container_sizes(vec![container_size, container_size]);
                self.set_viewport_sizes(vec![viewport_size, viewport_size]);
            }
            _ => {}
        }

        for (i, player) in players.iter().enumerate() {
            self.viewports[i].set_use_hdr_2d(true);
            let mut level = self.viewports[i].get_child(0).expect("Level not found");
            level.add_child(player);

            self.viewports[i].set_canvas_cull_mask(
                1 << player.bind().get_player_id() as u32 * 2
                    | 1 << player.bind().get_player_id() as u32 * 2 - 1
                    | 1,
            );
        }
    }

    /// Reset the split screen
    #[func]
    pub fn reset(&mut self) {
        self.set_container_sizes(vec![Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0)]);
        self.set_viewport_sizes(vec![Vector2i::new(0, 0), Vector2i::new(0, 0)]);
        self.base_mut().set_size(Vector2::new(0.0, 0.0));

        // Remove all children from the viewports
        for viewport in self.viewports.iter() {
            for mut child in viewport.get_children().iter_shared() {
                child.queue_free();
            }
        }
    }

    /// Get the spawn point for a player
    ///
    /// # Arguments
    ///
    /// * `player_id` - The ID of the player to get the spawn point for
    ///
    /// # Returns
    ///
    /// * A Vector2 value representing the spawn point for the player
    pub fn get_spawn_point(&self, player_id: i32) -> Vector2 {
        self.viewports[(player_id % 2) as usize]
            .get_child(0)
            .expect("Level not found")
            .try_cast::<Map>()
            .expect("Map not found")
            .bind()
            .get_spawn_point("3".to_string())
    }
}
