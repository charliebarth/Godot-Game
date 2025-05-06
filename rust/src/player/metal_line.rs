//! metal_line.rs
//!
//! This module defines the MetalLine class, which is responsible for drawing lines from the player.
//! It uses the Godot engine's drawing capabilities to render the lines. The lines can also have
//! different colors and can be shown or hidden based on the player's actions.
//!
//! Author: Charles Barth
//! Version: Spring 2025
use godot::{
    classes::{INode2D, Node2D},
    prelude::*,
};

use super::player::Player;

/// The MetalLine class is responsible for drawing lines from the player to nearby metal objects.
#[derive(GodotClass)]
#[class(base=Node2D)]
#[derive(Debug)]
pub struct MetalLine {
    /// The base node of the MetalLine.
    base: Base<Node2D>,
    /// The points that make up the line.
    points: PackedVector2Array,
    /// The colors for each line segment.
    colors: PackedColorArray,
    /// Whether the lines should be shown or not.
    should_show: bool,
    /// Whether the player is a remote ghost player or not
    remote_player: bool,
}

/// INode2D methods for the MetalLine
#[godot_api]
impl INode2D for MetalLine {
    /// The Godot constructor for the MetalLine class.
    ///
    /// # Arguments
    /// * `base` - The base node of the MetalLine.
    ///
    /// # Returns
    /// * `MetalLine` - A new instance of the MetalLine class.
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            points: PackedVector2Array::new(),
            colors: PackedColorArray::new(),
            should_show: false,
            remote_player: false,
        }
    }

    /// This a built in method for Godot that is called when a node is first added to the scene.
    fn ready(&mut self) {
        self.remote_player = self
            .base()
            .get_parent()
            .unwrap()
            .try_cast::<Player>()
            .unwrap()
            .bind()
            .is_remote_player();
    }

    /// This is a build in method for Godot that is called when a node is first added to the scene.
    /// It can also be called in other circumstances such as when the node is made visible.
    fn draw(&mut self) {
        if !self.should_show
            || self.points.is_empty()
            || self.colors.is_empty()
            || self.remote_player
        {
            return;
        }

        let points = self.points.clone();
        let colors = self.colors.clone();
        self.base_mut()
            .draw_multiline_colors_ex(&points, &colors)
            .width(2.0)
            .done();
    }

    /// This is a build in method for Godot that is called every frame.
    fn process(&mut self, _delta: f64) {
        if self.should_show {
            self.base_mut().queue_redraw();
        }
    }
}

#[godot_api]
impl MetalLine {
    /// Adds a line segment to the MetalLine.
    /// The line segment will be drawn from the player to the given end point.
    ///
    /// # Arguments
    /// * `end` - The end point of the line segment.
    /// * `color` - The color of the line segment.
    pub fn replace_lines(&mut self, points: PackedVector2Array, colors: PackedColorArray) {
        self.points = points;
        self.colors = colors;
    }

    /// Updates the MetalLine to determine if it should be shown or not.
    ///
    /// # Arguments
    /// * `should_show` - Whether the MetalLine should be shown or not.
    #[func]
    pub fn set_should_show(&mut self, should_show: bool) {
        self.should_show = should_show;
    }

    /// Updates the color of a specific line segment.
    ///
    /// # Arguments
    /// * `color` - The new color for the line segment.
    /// * `index` - The index of the line segment to update.
    pub fn update_color(&mut self, color: Color, index: usize) {
        if self.colors.len() > index {
            self.colors[index] = color;
        }
    }
}
