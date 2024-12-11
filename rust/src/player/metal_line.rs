use godot::{
    classes::{INode2D, Node2D},
    prelude::*,
};

/// The MetalLine class is responsible for drawing lines from the player to nearby metal objects.
#[derive(GodotClass)]
#[class(base=Node2D)]
#[derive(Debug)]
pub struct MetalLine {
    /// The base node of the MetalLine.
    base: Base<Node2D>,
    /// The points that make up the line.
    points: Option<PackedVector2Array>,
    /// The colors for each line segment.
    colors: Option<PackedColorArray>,
    /// Whether the lines should be shown or not.
    should_show: bool,
}

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
            points: None,
            colors: None,
            should_show: false,
        }
    }

    /// This is a build in method for Godot that is called when a node is first added to the scene.
    /// It can also be called in other circumstances such as when the node is made visible.
    fn draw(&mut self) {
        if !self.should_show || self.points.is_none() {
            return;
        }

        let points = self.points.take().unwrap();
        let colors = self.colors.take().unwrap();
        self.base_mut()
            .draw_multiline_colors_ex(points, colors)
            .width(2.0)
            .done();
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
    #[func]
    pub fn add_line_segment(&mut self, end: Vector2, color: Color) {
        let mut points: PackedVector2Array;
        let mut colors: PackedColorArray;

        if self.points.is_none() {
            points = PackedVector2Array::new();
            colors = PackedColorArray::new();
        } else {
            points = self.points.take().unwrap();
            colors = self.colors.take().unwrap();
        }

        let start = self.base().to_local(self.base().get_global_position());
        points.push(start);
        points.push(self.base().to_local(end));
        colors.push(color);

        self.points = Some(points);
        self.colors = Some(colors);
    }

    /// Updates the MetalLine to determine if it should be shown or not.
    ///
    /// # Arguments
    /// * `should_show` - Whether the MetalLine should be shown or not.
    #[func]
    pub fn set_should_show(&mut self, should_show: bool) {
        self.should_show = should_show;
    }
}
