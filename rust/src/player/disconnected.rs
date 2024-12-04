use godot::{
    classes::{ColorRect, ILabel, Label, ShaderMaterial, Timer},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Label)]
pub struct Disconnected {
    base: Base<Label>,
    progress_bar: Option<Gd<ColorRect>>,
    timer: Option<Gd<Timer>>,
}

#[godot_api]
impl ILabel for Disconnected {
    fn init(base: Base<Label>) -> Self {
        Self {
            base,
            progress_bar: None,
            timer: None,
        }
    }

    fn process(&mut self, _delta: f64) {
        let timer = self.get_timer();

        if !timer.is_stopped() {
            let remaining_time = timer.get_time_left();
            let progress_bar = self.get_progress_bar();
            let mut material = progress_bar
                .get_material()
                .expect("Material not found")
                .try_cast::<ShaderMaterial>()
                .unwrap();

            material
                .set_shader_parameter("value".into(), Variant::from(remaining_time as f64 / 10.0));

            self.base_mut().set_text(GString::from(format!(
                "Disconnected\n{}",
                remaining_time as i32
            )));
        }
    }
}

#[godot_api]
impl Disconnected {
    pub fn get_progress_bar(&mut self) -> Gd<ColorRect> {
        if self.progress_bar.is_none() {
            self.progress_bar = Some(self.base().get_node_as::<ColorRect>("ProgressBar"));
        }

        self.progress_bar
            .as_ref()
            .expect("LineSelector node not found")
            .clone()
    }

    pub fn get_timer(&mut self) -> Gd<Timer> {
        if self.timer.is_none() {
            self.timer = Some(self.base().get_node_as::<Timer>("Timer"));
        }

        self.timer.as_ref().expect("Timer node not found").clone()
    }

    #[func]
    pub fn on_visibility_changed(&mut self) {
        if self.base().is_visible() {
            let mut timer = self.get_timer();
            timer.set_wait_time(10.0);
            timer.start();
        } else {
            self.get_timer().stop();
        }
    }
}
