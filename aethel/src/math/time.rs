use glfw::Glfw;

pub struct Time {
    delta_time: f64,
    last_frame: f64
}
impl Time {
    pub fn new() -> Self {
        Self { delta_time: 0.0, last_frame: 0.0 }
    }
    pub fn get_delta_time(&mut self, glfw: &Glfw) -> f64 {
        let current_frame = glfw.get_time();
        self.delta_time = current_frame - self.last_frame;
        self.last_frame = current_frame;
        self.delta_time
    }
}