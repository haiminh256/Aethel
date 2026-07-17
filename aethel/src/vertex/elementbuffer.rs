use std::os::raw::c_void;

pub struct ElementBuffer {
    id: u32
}
impl ElementBuffer {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Self { id: id }
    }
    pub fn load_data(&self, indices: &[u32]){
        unsafe {
            let size = (indices.len() * size_of::<u32>()) as isize;
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, indices.as_ptr() as *const c_void, gl::STATIC_DRAW);
        }
    }
    pub fn bind(&self){
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }
    pub fn delete(&mut self){
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}