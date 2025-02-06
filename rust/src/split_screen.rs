use godot::{
    classes::{
        viewport::DefaultCanvasItemTextureFilter, HBoxContainer, IHBoxContainer, SubViewport,
        SubViewportContainer, World2D,
    },
    prelude::*,
};

use crate::{map::Map, player::player::Player};

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

#[godot_api]
impl IHBoxContainer for SplitScreen {
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

    #[func]
    /// Set the size of the viewports
    ///
    /// # Arguments
    ///
    /// * `sizes` - A vector of Vector2i values representing the size of each viewport
    pub fn set_viewport_sizes(&mut self, sizes: Vec<Vector2i>) {
        for (i, size) in sizes.iter().enumerate() {
            self.viewports[i].set_size(*size);
        }
    }

    #[func]
    /// Add a level to the split screen
    /// The level will be added to the first viewport and the second viewport will be set to the same world
    ///
    /// # Arguments
    ///
    /// * `level` - A Gd<Map> value representing the level to add
    pub fn add_level(&mut self, level: Gd<Map>) {
        self.viewports[0].add_child(&level);
        let world_2d = self.viewports[0].get_world_2d().expect("World2D not found");
        self.viewports[1].set_world_2d(&world_2d);
    }

    #[func]
    /// Add a world to the split screen
    /// This is used for the second split screen so both viewports will use the world rather than having their own
    /// level added to them.
    ///
    /// # Arguments
    ///
    /// * `world` - A Gd<World2D> value representing the world to add
    pub fn add_world(&mut self, world: Gd<World2D>) {
        self.viewports[0].set_world_2d(&world);
        self.viewports[1].set_world_2d(&world);
    }

    #[func]
    /// Get the world of the split screen
    ///
    /// # Returns
    ///
    /// * A Gd<World2D> value representing the world of the split screen
    pub fn get_world(&self) -> Gd<World2D> {
        self.viewports[1].get_world_2d().expect("World2D not found")
    }

    #[func]
    /// Add players to the split screen
    /// If there is only one player then the first viewport will be used and the same size as the HBoxContainer
    /// If there are two players then the first viewport will be used for the first player and the second viewport will be used for the second player
    /// In this case each viewport will be the full height of the HBoxContainer and the width will be half of the HBoxContainer
    ///
    /// # Arguments
    ///
    /// * `players` - A vector of Gd<Player> values representing the players to add
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

    #[func]
    /// Reset the split screen
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
}
