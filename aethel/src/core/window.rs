use glfw::{self, Context, Glfw, GlfwReceiver, PWindow, WindowEvent};

pub struct Window {
    glfw: Glfw,
    window: PWindow, 
    events: GlfwReceiver<(f64, WindowEvent)>
}
impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

        let (window, events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed).expect("Can't create window");
        Self { glfw: glfw, window: window, events: events}
    }
    pub fn should_close(&mut self) -> bool {
        self.window.should_close()
    }
    pub fn get_window(&mut self) -> &PWindow {
        &self.window
    }
    pub fn get_instance(&mut self) -> &Glfw {
        &self.glfw
    }
    pub fn get_event(&mut self) -> &GlfwReceiver<(f64, WindowEvent)>{
        &self.events
    }
    pub fn poll_events(&mut self){
        self.glfw.poll_events();
    }
    pub fn swap_buffers(&mut self){
        self.window.swap_buffers();
    }
}