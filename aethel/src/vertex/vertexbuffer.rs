use std::os::raw::c_void;

pub struct VertexBuffer {
    id: u32
}
impl VertexBuffer {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Self { id: id }
    }
    pub fn load_data(&self, data: &[f32]){
        unsafe {
            let size = (data.len() * size_of::<f32>()) as isize;
            gl::BufferData(gl::ARRAY_BUFFER, size, data.as_ptr() as *const c_void, gl::STATIC_DRAW);
        }
    }
    pub fn bind(&self){
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }
    pub fn delete(&mut self){
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}