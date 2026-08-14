use nalgebra_glm as glm;

pub struct Transform {
	pub position: glm::Vec3,
	pub rotation: glm::Vec3,
	pub scale: glm::Vec3
}
impl Transform {
	pub fn new() -> Self{
		let position = glm::vec3(0.0, 0.0, 0.0);
		let rotation = glm::vec3(0.0, 0.0, 0.0);
		let scale = glm::vec3(1.0, 1.0, 1.0);

		Self { position: position, rotation: rotation, scale: scale}
	}
	pub fn get_model_matrix(&self) -> glm::Mat4 {
		let mut model = glm::identity::<f32, 4>();

		model = glm::translate(&model, &self.position);

		let rotation_rad = glm::radians(&self.rotation);
		// Z trước
		model = glm::rotate(&model, rotation_rad[0], &glm::vec3(0.0, 0.0, 1.0));

		// Y giữa
		model = glm::rotate(&model, rotation_rad[1], &glm::vec3(0.0, 1.0, 0.0));

		// X sau
		model = glm::rotate(&model, rotation_rad[2], &glm::vec3(1.0, 0.0, 0.0));

		model = glm::scale(&model, &self.scale);
		model
	}
}