use godot::classes::{ITextureProgressBar, ResourceLoader, Texture2D, TextureProgressBar};
/// Represents a Metal Bar that contains the amount of reserves for a particular metal type.
///
/// Author : Trinity Pittman
/// Version : Fall 2024
use godot::prelude::*;

/// The maximum number of metal reserves a player can have
const MAX_RESERVE: f64 = 100.0;
/// The minumum number of metal reserves a player can have
const MIN_RESERVE: f64 = 0.0;

#[derive(GodotClass)]
#[class(base=TextureProgressBar)]
/// Struct that reprents a Metal Reserve Bar
pub struct MetalBar {
    // The base node of the metal bar
    base: Base<TextureProgressBar>,
    /// The amount of Metal reserved in the bar
    reserves: f64,
}

#[godot_api]
/// Godot methods for Metal reserve bar
impl ITextureProgressBar for MetalBar {
    /// The Godot contructor for the MetalBar class node
    ///
    /// # Arguments
    /// * `base` - The base node type for the metal bar
    ///
    /// # Returns
    /// * `MetalBar` - The MetalBar node
    fn init(base: Base<TextureProgressBar>) -> Self {
        Self {
            base,
            reserves: 0.0,
        }
    }

    /// The Godot method called when the coin counter enters the scene tree for the first time
    /// Sets the Metals value to 0.0 at the start of the round and sets min and max value
    fn ready(&mut self) {
        self.base_mut().set_value(0.0);
        self.base_mut().set_min(MIN_RESERVE);
        self.base_mut().set_max(MAX_RESERVE);
    }
}

/// Methods for the metal bar
impl MetalBar {
    /// Sets the under texture and progress texture of this metal bar
    ///
    /// # Arguments
    /// * `path` (&str) - the path to the progress texture
    pub fn set_texture(&mut self, path: &str) {
        // Every bar will have the same under texture so we set that first
        let under_path: &str = "res://assets/HealthMetalBars/HealthBar DARK.png";

        let texture_under: Gd<Texture2D> = self.load_texture(under_path);
        self.base_mut().set_under_texture(texture_under); // Set to godot node

        // The progress texture is dependent on the type of metal and is passed into this function
        let path_str: String = format!("res://assets/HealthMetalBars/metal_bar_prog_{}.png", path);

        let texture_progress: Gd<Texture2D> = self.load_texture(path_str.as_str());
        self.base_mut().set_progress_texture(texture_progress); // Set to godot node

        self.base_mut()
            .set_texture_progress_offset(Vector2::new(0.0, 1.0)); // offset for prog
    }

    /// Loads in a texture given a path to the file.
    ///
    /// # Arguments
    /// * `path` (&str) - the path the the texture to load
    /// # Returns
    /// * (`Gd<Texture2D>`) - the texture loaded
    fn load_texture(&mut self, path: &str) -> Gd<Texture2D> {
        let mut loader: Gd<ResourceLoader> = ResourceLoader::singleton();

        let tex: Option<Gd<Resource>> = loader.load(path.into());

        tex.unwrap().cast::<Texture2D>()
    }

    /// Sets the name of this metal bar
    /// # Arguments
    /// * `name` (&str) - the name to set it to
    pub fn set_name(&mut self, name: &str) {
        let name_g: GString = GString::from(name); // Change the string to a GString for godot
        self.base_mut().set_name(name_g);
    }

    /// Gets the name of this metal bar
    /// # Returns
    /// * (StringName) - the name of the bar
    pub fn get_name(&mut self) -> StringName {
        self.base_mut().get_name()
    }

    /// Hides this metal bar from the scene
    pub fn hide(&mut self) {
        self.base_mut().hide();
    }

    /// Getter method for the current number of reserves
    /// # Returns
    /// * (f64) - the current reserves
    pub fn get_reserves(&mut self) -> f64 {
        self.reserves
    }

    /// Setter method for the reserves
    ///
    /// # Arguments
    /// * `reserves` (f64) - the reserve value to set the reserves to
    pub fn set_value(&mut self, reserves: f64) {
        self.base_mut().set_value(reserves);
    }

    /// Adjusts the number of reserves of this metal positively or negatively
    ///
    /// # Arguments
    /// * `reserve` (f64) - the reserve amount to increment or decrement by
    pub fn adjust_reserves(&mut self, reserve: f64) {
        let new_reserve = if reserve < 0.0 {
            // If adjusting negatively
            if self.reserves < -reserve {
                MIN_RESERVE
            } else {
                self.reserves + reserve
            }
        } else {
            // If adjusting positively
            if self.reserves + reserve > MAX_RESERVE {
                MAX_RESERVE
            } else {
                self.reserves + reserve
            }
        };

        self.reserves = new_reserve.clamp(MIN_RESERVE, MAX_RESERVE);
        self.set_value(self.reserves); // Set the value in the godot node
    }
}
