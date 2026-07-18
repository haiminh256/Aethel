pub mod vertexbuffer;
pub mod elementbuffer;
pub mod arraybuffer;

use nalgebra_glm as glm;
#[allow(dead_code)]
#[repr(C)]
pub struct Vertex {
    position: glm::Vec3,
    color: glm::Vec3,
    normal: glm::Vec3,
    tex_coord: glm::Vec2
}
impl Vertex {
    pub fn new(position: glm::Vec3, color: glm::Vec3, normal: glm::Vec3, tex_coord: glm::Vec2) -> Self {
        Self { position, color, normal, tex_coord }
    }
}