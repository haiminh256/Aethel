use std::{os::raw::c_void};


pub struct ArrayBuffer {
    id: u32
}
impl ArrayBuffer {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        Self { id: id }
    }
    pub fn bind(&self){
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }
    pub fn vertex_atrrib_pointer(&self, location: u32, components: i32, stride: i32, offset_bytes: usize){
        unsafe {
            let f32_count = offset_bytes / size_of::<f32>();
            
            let offset_ptr = (f32_count * size_of::<f32>()) as *const c_void;
            gl::VertexAttribPointer(location, components, gl::FLOAT, gl::FALSE, stride, offset_ptr);
            gl::EnableVertexAttribArray(location);
        }
    }
    pub fn delete(&mut self){
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}