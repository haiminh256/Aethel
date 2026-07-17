use std::os::raw::c_void;

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
    pub fn make_context_current(&mut self){
        self.window.make_current();
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
    pub fn clear(&self){
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
    pub fn clear_color(&self, red: f32, green: f32, blue: f32){
        unsafe {
            gl::ClearColor(red, green, blue, 1.0);
        }
    }
    pub fn draw(&self, count: i32, indices: i32){
        unsafe {
            gl::DrawElements(gl::TRIANGLES, count, gl::UNSIGNED_INT, indices as *const c_void);
        }
    }
}