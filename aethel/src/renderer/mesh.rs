use crate::shader::Shader;
use crate::vertex::Vertex;
use crate::renderer::texture::Texture;
use crate::vertex::arraybuffer::ArrayBuffer;
use crate::vertex::elementbuffer::ElementBuffer;
use crate::vertex::vertexbuffer::VertexBuffer;

pub struct Mesh {
    m_vertices: Vec<Vertex>,
    m_indices: Vec<u32>,
    m_textures: Vec<Texture>, // Đổi từ Option thành Vec trực tiếp để dễ quản lý
    m_vbo: VertexBuffer,      // Khởi tạo trực tiếp thay vì bọc trong Option
    m_vao: ArrayBuffer,
    m_ebo: ElementBuffer,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        let vao = ArrayBuffer::new(); 
        let vbo = VertexBuffer::new();
        let ebo = ElementBuffer::new();

        let mut mesh = Self {
            m_vertices: vertices,
            m_indices: indices,
            m_textures: Vec::new(),
            m_vbo: vbo,
            m_vao: vao,
            m_ebo: ebo,
        };

        mesh.setup_mesh();
        mesh
    }
    pub fn draw(&self, _shader: &Shader) {
        let mut diffuse_nr: u32 = 1;
        let mut specular_nr: u32 = 1;
        let mut normal_nr: u32 = 1;
        let mut height_nr: u32 = 1;

        for (i, texture) in self.m_textures.iter().enumerate() {
            // Giả định trường 'Type' trong Texture là một chuỗi &str hoặc String
            let name = &texture.tex_type; 
            
            let number = match name.as_str() {
                "texture_diffuse" => { let val = diffuse_nr; diffuse_nr += 1; val },
                "texture_specular" => { let val = specular_nr; specular_nr += 1; val },
                "texture_normal" => { let val = normal_nr; normal_nr += 1; val },
                "texture_height" => { let val = height_nr; height_nr += 1; val },
                _ => 1,
            };

            let _uniform_name = format!("{}{}", name, number);

            texture.bind(i as u32);
        }

        self.m_vao.bind();
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.m_indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
            gl::ActiveTexture(gl::TEXTURE0);
        }
    }

    pub fn setup_mesh(&mut self) {
        self.m_vao.bind();
        
        self.m_vbo.bind();
        self.m_vbo.load_data(&self.m_vertices);

        self.m_ebo.bind();
        self.m_ebo.load_data(&self.m_indices);

        let stride = std::mem::size_of::<Vertex>() as i32;
        self.m_vao.vertex_atrrib_pointer(0, 3, stride, std::mem::offset_of!(Vertex, position));
        self.m_vao.vertex_atrrib_pointer(1, 4, stride, std::mem::offset_of!(Vertex, color));
        self.m_vao.vertex_atrrib_pointer(2, 3, stride, std::mem::offset_of!(Vertex, normal));
        self.m_vao.vertex_atrrib_pointer(3, 2, stride, std::mem::offset_of!(Vertex, tex_coord));
    }
}