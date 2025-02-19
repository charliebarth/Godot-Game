use godot::{
    classes::{ColorRect, ILabel, Label, ShaderMaterial, Timer},
    prelude::*,
};

/// The Disconnected class is responsible for displaying a timer and removing the player from the game when the timer runs out.
/// This node is made active when the device responsible for the player disconnects from the game.
/// It will be diabled if the device reconnects before the timer runs out.
#[derive(GodotClass)]
#[class(base=Label)]
pub struct Disconnected {
    /// The base node of the Disconnected class.
    base: Base<Label>,
    /// The progress bar that shows the remaining time.
    progress_bar: Option<Gd<ColorRect>>,
    /// The timer that counts down the remaining time.
    timer: Option<Gd<Timer>>,
}

#[godot_api]
impl ILabel for Disconnected {
    /// The Godot constructor for the Disconnected class.
    ///
    /// # Arguments
    /// * `base` - The base node of the Disconnected class.
    ///
    /// # Returns
    /// * `Disconnected` - A new instance of the Disconnected class.
    fn init(base: Base<Label>) -> Self {
        Self {
            base,
            progress_bar: None,
            timer: None,
        }
    }

    /// This is a build in method for Godot that is called every visual frame.
    /// This is where the progress bar is updated based on the remaining time.
    ///
    /// # Arguments
    /// * `delta` - The time since the last frame.
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

            material.set_shader_parameter("value", &Variant::from(remaining_time as f64 / 10.0));

            self.base_mut().set_text(&GString::from(format!(
                "Disconnected\n{}",
                remaining_time as i32
            )));
        }
    }
}

#[godot_api]
impl Disconnected {
    /// Returns the progress bar node.
    ///
    /// # Returns
    /// * `Gd<ColorRect>` - The progress bar node.
    pub fn get_progress_bar(&mut self) -> Gd<ColorRect> {
        if self.progress_bar.is_none() {
            self.progress_bar = Some(self.base().get_node_as::<ColorRect>("ProgressBar"));
        }

        self.progress_bar
            .as_ref()
            .expect("LineSelector node not found")
            .clone()
    }

    /// Returns the timer node.
    ///
    /// # Returns
    /// * `Gd<Timer>` - The timer node.
    pub fn get_timer(&mut self) -> Gd<Timer> {
        if self.timer.is_none() {
            self.timer = Some(self.base().get_node_as::<Timer>("Timer"));
        }

        self.timer.as_ref().expect("Timer node not found").clone()
    }

    /// This is a function that is exposed to the Godot engine so that it can be called when a signal is emitted.
    /// When this node is made visible, the timer will be started.
    /// When this node is made invisible, the timer will be stopped.
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
