pub mod camera;
use crate::core::window::Window;

pub struct Renderer;
impl Renderer {
    pub fn new() -> Self{
        Self {}
    }
    pub fn init(&self, window: &mut Window) {
        gl::load_with(|symbol| {
            match window.get_instance().get_proc_address_raw(symbol) {
                Some(f) => f as *const std::ffi::c_void,
                None => std::ptr::null(),
            }
        }); 
    }
}