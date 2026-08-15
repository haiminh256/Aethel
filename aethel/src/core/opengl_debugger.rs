use std::ffi::CStr;
use std::os::raw::{c_char, c_void};

pub struct OpenGLDebugger;

impl OpenGLDebugger {
    pub fn enable_debug() {
        unsafe {
            // 1. Bật tính năng gỡ lỗi của OpenGL
            gl::Enable(gl::DEBUG_OUTPUT);

            // 2. Ép buộc OpenGL gọi hàm callback ngay lập tức khi xảy ra lỗi
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);

            // 3. Đăng ký hàm callback của chúng ta với OpenGL
            gl::DebugMessageCallback(Some(Self::message_callback), std::ptr::null());

            // 4. Bộ lọc: Nhận tất cả các loại thông báo lỗi từ mọi nguồn
            gl::DebugMessageControl(
                gl::DONT_CARE,
                gl::DONT_CARE,
                gl::DONT_CARE,
                0,
                std::ptr::null(),
                gl::TRUE,
            );
        }
        println!("[INFO] OpenGL Debug Output has been successfully enabled!");
    }

    // Định nghĩa hàm callback với extern "system" tương đương __stdcall trên Windows
    extern "system" fn message_callback(
        source: u32,
        ty: u32,
        id: u32,
        severity: u32,
        _length: i32,
        message: *const c_char,
        _user_param: *mut c_void,
    ) {
        // Bỏ qua các thông báo thông tin thông thường không quan trọng
        if id == 131185 || id == 131218 || id == 131204 {
            return;
        }

        // Chuyển đổi con trỏ C-string sang Rust &str an toàn
        let msg_str = unsafe {
            if message.is_null() {
                "No message"
            } else {
                CStr::from_ptr(message).to_str().unwrap_or("Invalid UTF-8 string")
            }
        };

        // Phân loại nguồn phát ra lỗi
        let source_str = match source {
            gl::DEBUG_SOURCE_API => "API",
            gl::DEBUG_SOURCE_WINDOW_SYSTEM => "Window System",
            gl::DEBUG_SOURCE_SHADER_COMPILER => "Shader Compiler",
            gl::DEBUG_SOURCE_THIRD_PARTY => "Third Party",
            gl::DEBUG_SOURCE_APPLICATION => "Application",
            _ => "Unknown",
        };

        // Phân loại kiểu lỗi (Rust đổi 'type' thành 'ty' vì trùng từ khóa)
        let type_str = match ty {
            gl::DEBUG_TYPE_ERROR => "Error",
            gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "Deprecated Behavior",
            gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "Undefined Behavior",
            gl::DEBUG_TYPE_PORTABILITY => "Portability Issue",
            gl::DEBUG_TYPE_PERFORMANCE => "Performance Issue",
            _ => "Other",
        };

        // In log dựa trên mức độ nghiêm trọng (Severity)
        match severity {
            gl::DEBUG_SEVERITY_HIGH => {
                eprintln!(
                    "[OpenGL High Error] Source: {} | Type: {} | ID: {}\nMessage: {}",
                    source_str, type_str, id, msg_str
                );
                
                // Thay thế cho __debugbreak() trong C++ để kích hoạt ngắt chương trình khi gỡ lỗi
                #[cfg(target_arch = "x86_64")]
                unsafe { std::arch::asm!("int3") };
                #[cfg(target_arch = "x86")]
                unsafe { std::arch::asm!("int3") };
            }
            gl::DEBUG_SEVERITY_MEDIUM => {
                eprintln!(
                    "[OpenGL Medium Error] Source: {} | Type: {} | Message: {}",
                    source_str, type_str, msg_str
                );
            }
            gl::DEBUG_SEVERITY_LOW => {
                println!("[OpenGL Low Warning] Source: {} | Message: {}", source_str, msg_str);
            }
            gl::DEBUG_SEVERITY_NOTIFICATION => {
                // Tháo chú thích ra nếu bạn muốn xem cả thông báo hệ thống thông thường
                // println!("[OpenGL Info] Message: {}", msg_str);
            }
            _ => {}
        }
    }
}
