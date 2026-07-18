use std::{ffi::CString, os::raw::c_void};
use ::stb_image::stb_image::{stbi_image_free, stbi_load, stbi_set_flip_vertically_on_load};
pub struct Texture {
    id: u32,
    width: i32,
    height: i32,
    channels: i32
}
impl Texture {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }
        Self { id: id, width: 0, height: 0, channels: 0 }
    }
    pub fn load(&mut self, file_path: &str){
        unsafe {
            stbi_set_flip_vertically_on_load(1);
            let path = CString::new(file_path).unwrap();
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
    		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            let data = stbi_load(path.as_ptr(),&mut self.width,&mut self.height,&mut self.channels, 0);
            if data.is_null() {
                return;
            }
            let format = match self.channels {
                1 => gl::RED,
                3 => gl::RGB,
                4 => gl::RGBA,
                _ => panic!("Unsupported channel count: {}", self.channels),
            };
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, format as i32, self.width, self.height, 0, format, gl::UNSIGNED_BYTE, data as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            stbi_image_free(data as *mut c_void);
        }       
    } 
    pub fn bind(&self, unit: u32){
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
    pub fn delete(&self){
        unsafe {
            gl::DeleteTextures(1, self.id as *const u32);
        }
    }
}