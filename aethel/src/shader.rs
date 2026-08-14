use std::fs;
use std::ffi::CString;
use std::ptr;

pub struct Shader {
    pub id: u32, 
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        unsafe {
            let mut success = 0;
            let mut info_log = vec![0u8; 512]; // Mảng chứa log lỗi của OpenGL

            // ========================================================
            // 1. ĐỌC VÀ BIÊN DỊCH VERTEX SHADER
            // ========================================================
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let vertex_source = fs::read_to_string(vertex_path)
                .unwrap_or_else(|err| panic!("[-] Failed to open vertex shader file: {vertex_path}. Error: {err}"));
            
            // Ép chuỗi văn bản sang CString
            let c_vertex_source = CString::new(vertex_source.as_bytes()).unwrap();
            let vertex_ptr = c_vertex_source.as_ptr();
            
            // Nạp source và ép buộc giữ c_vertex_source sống để compile
            gl::ShaderSource(vertex_shader, 1, &vertex_ptr, ptr::null());
            gl::CompileShader(vertex_shader);

            // Kiểm tra lỗi biên dịch Vertex Shader
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success == gl::FALSE as i32 {
                gl::GetShaderInfoLog(vertex_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut i8);
                let log_string = String::from_utf8_lossy(&info_log);
                eprintln!("[-] Vertex Shader Compilation Failed ({vertex_path}):\n{log_string}");
            }

            // ========================================================
            // 2. ĐỌC VÀ BIÊN DỊCH FRAGMENT SHADER
            // ========================================================
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let fragment_source = fs::read_to_string(fragment_path)
                .unwrap_or_else(|err| panic!("[-] Failed to open fragment shader file: {fragment_path}. Error: {err}"));
            
            let c_fragment_source = CString::new(fragment_source.as_bytes()).unwrap();
            let fragment_ptr = c_fragment_source.as_ptr();
            
            gl::ShaderSource(fragment_shader, 1, &fragment_ptr, ptr::null());
            gl::CompileShader(fragment_shader);

            // Kiểm tra lỗi biên dịch Fragment Shader
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success == gl::FALSE as i32 {
                gl::GetShaderInfoLog(fragment_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut i8);
                let log_string = String::from_utf8_lossy(&info_log);
                eprintln!("[-] Fragment Shader Compilation Failed ({fragment_path}):\n{log_string}");
            }

            // ========================================================
            // 3. TẠO SHADER PROGRAM VÀ LIÊN KẾT (LINK)
            // ========================================================
            let program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vertex_shader);
            gl::AttachShader(program_id, fragment_shader);
            gl::LinkProgram(program_id);

            // Kiểm tra lỗi Link chương trình tổng
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
            if success == gl::FALSE as i32 {
                gl::GetProgramInfoLog(program_id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut i8);
                let log_string = String::from_utf8_lossy(&info_log);
                eprintln!("[-] Shader Program Linking Failed:\n{log_string}");
            }

            // Dọn dẹp các shader thành phần sau khi đã link thành công vào chương trình tổng
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Shader { id: program_id }
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn delete(&self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
    pub fn set_int(&self, name: &str, value: i32) {
        let uniform_name = CString::new(name).unwrap();
        unsafe {
            gl::Uniform1i(gl::GetUniformLocation(self.id, uniform_name.as_ptr() as *const i8), value);
        }
    }
}
