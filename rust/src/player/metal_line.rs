use godot::{
    classes::{INode2D, Node2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Node2D)]
#[derive(Debug)]
pub struct MetalLine {
    base: Base<Node2D>,
    points: Option<PackedVector2Array>,
    colors: Option<PackedColorArray>,
    should_show: bool,
}

#[godot_api]
impl INode2D for MetalLine {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            points: Some(PackedVector2Array::new()),
            colors: Some(PackedColorArray::new()),
            should_show: false,
        }
    }

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

    #[func]
    pub fn set_should_show(&mut self, should_show: bool) {
        self.should_show = should_show;
    }
}
