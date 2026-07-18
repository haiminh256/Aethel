pub mod vertexbuffer;
pub mod elementbuffer;
pub mod arraybuffer;

use nalgebra_glm as glm;
#[allow(dead_code)]
#[repr(C)]
pub struct Vertex {
    position: glm::Vec3,
    color: glm::Vec4,
    normal: glm::Vec3,
    tex_coord: glm::Vec2
}
impl Vertex {
    pub fn new(position: glm::Vec3, color: glm::Vec3, normal: glm::Vec3, tex_coord: glm::Vec2) -> Self {
        Self { position, color: glm::vec4(color.x, color.y, color.z, 1.0), normal, tex_coord }
    }
}