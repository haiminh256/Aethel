use std::{ffi::CString, os::raw::c_void};
use ::stb_image::stb_image::{stbi_image_free, stbi_load, stbi_set_flip_vertically_on_load};

pub struct Texture {
    id: u32,
    width: i32,
    height: i32,
    channels: i32,
    pub tex_type: String, // Tương ứng với std::string Type trong C++
    pub path: String,     // Tương ứng với std::string Path trong C++
}

impl Texture {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }
        Self { 
            id, 
            width: 0, 
            height: 0, 
            channels: 0,
            tex_type: String::new(), // Giá trị mặc định ban đầu là chuỗi rỗng
            path: String::new(),     // Giá trị mặc định ban đầu là chuỗi rỗng
        }
    }

    // Hàm load nhận thêm biến flip (bool) đúng theo hàm C++ Load(..., bool flip)
    pub fn load(&mut self, file_path: &str, flip: bool) -> bool {
        unsafe {
            let flip_val = if flip { 1 } else { 0 };
            stbi_set_flip_vertically_on_load(flip_val);
            
            let path_cstring = match CString::new(file_path) {
                Ok(p) => p,
                Err(_) => {
                    println!("Texture::Load() failed: Invalid path string");
                    return false;
                }
            };

            // Lưu lại đường dẫn vào struct giống như logic C++
            self.path = file_path.to_string();

            // Thiết lập thông số texture giống C++ trước khi load dữ liệu
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            let data = stbi_load(
                path_cstring.as_ptr(),
                &mut self.width,
                &mut self.height,
                &mut self.channels,
                0
            );

            if data.is_null() {
                println!("Texture::Load() failed");
                return false;
            }

            let format = match self.channels {
                1 => gl::RED,
                3 => gl::RGB,
                4 => gl::RGBA,
                _ => gl::RGB, // Mặc định về GL_RGB giống C++
            };

            gl::BindTexture(gl::TEXTURE_2D, self.id);

            // C++ thiết lập lại các thông số này một lần nữa sau khi bind
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as i32,
                self.width,
                self.height,
                0,
                format,
                gl::UNSIGNED_BYTE,
                data as *const c_void
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
            stbi_image_free(data as *mut c_void);
            true
        }       
    } 

    pub fn bind(&self, unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn delete(&mut self) {
        unsafe {
            if self.id != 0 {
                gl::DeleteTextures(1, &self.id);
                self.id = 0;
            }
        }
    }

    // Tương ứng với void Texture::setType(std::string Type)
    pub fn set_type(&mut self, tex_type: &str) {
        self.tex_type = tex_type.to_string();
    }

    // Tương ứng với void Texture::setPath(std::string Path)
    pub fn set_path(&mut self, path: &str) {
        self.path = path.to_string();
    }
}
