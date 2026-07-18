use aethel::core::window::Window;
use aethel::math::time::Time;
use aethel::renderer::Renderer;
use aethel::renderer::texture::Texture;
use aethel::shader::Shader;
use aethel::vertex::arraybuffer::ArrayBuffer;
use aethel::vertex::elementbuffer::ElementBuffer;
use aethel::vertex::vertexbuffer::VertexBuffer;
use aethel::renderer::camera::Camera;
use nalgebra_glm as glm;

fn main(){
    let mut window: Window = Window::new(800, 600, "Game Engine");
    window.make_context_current();
    let renderer: Renderer = Renderer::new();
    renderer.init(&mut window);

    let vertices = [
	 0.5, -0.5,  0.5,  1.0, 1.0, 1.0, 1.0,  0.0, 0.0, // 0
	-0.5, -0.5,  0.5,  1.0, 1.0, 1.0, 1.0,  1.0, 0.0, // 1
	-0.5,  0.5,  0.5,  1.0, 1.0, 1.0, 1.0,  1.0, 1.0, // 2
	 0.5,  0.5,  0.5,  1.0, 1.0, 1.0, 1.0,  0.0, 1.0, // 3

	 // Back (-Z)
	  0.5, -0.5, -0.5,  1.0, 1.0, 1.0, 1.0,  0.0, 0.0, // 4
	 -0.5, -0.5, -0.5,  1.0, 1.0, 1.0, 1.0,  1.0, 0.0, // 5
	 -0.5,  0.5, -0.5,  1.0, 1.0, 1.0, 1.0,  1.0, 1.0, // 6
	  0.5,  0.5, -0.5,  1.0, 1.0, 1.0, 1.0,  0.0, 1.0, // 7

	  // Left (-X)
	  -0.5, -0.5, -0.5,  1.0, 1.0, 1.0, 1.0,  0.0, 0.0, // 8
	  -0.5, -0.5,  0.5,  1.0, 1.0, 1.0, 1.0,  1.0, 0.0, // 9
	  -0.5,  0.5,  0.5,  1.0, 1.0, 1.0, 1.0,  1.0, 1.0, // 10
	  -0.5,  0.5, -0.5,  1.0, 1.0, 1.0, 1.0,  0.0, 1.0, // 11

	  // Right (+X)
	   0.5, -0.5,  0.5,  1.0, 1.0, 1.0, 1.0,  0.0, 0.0, // 12
	   0.5, -0.5, -0.5,  1.0, 1.0, 1.0, 1.0,  1.0, 0.0, // 13
	   0.5,  0.5, -0.5,  1.0, 1.0, 1.0, 1.0,  1.0, 1.0, // 14
	   0.5,  0.5,  0.5,  1.0, 1.0, 1.0, 1.0,  0.0, 1.0, // 15

	   // Top (+Y)
	   -0.5,  0.5,  0.5,  1.0, 1.0, 1.0, 1.0,  0.0, 0.0, // 16
	   -0.5,  0.5, -0.5,  1.0, 1.0, 1.0, 1.0,  1.0, 0.0, // 17
		0.5,  0.5, -0.5,  1.0, 1.0, 1.0, 1.0,  1.0, 1.0, // 18
		0.5,  0.5,  0.5,  1.0, 1.0, 1.0, 1.0,  0.0, 1.0, // 19
       -0.5, -0.5, -0.5,  1.0, 1.0, 1.0, 1.0,  0.0, 0.0, // 20
       -0.5, -0.5,  0.5,  1.0, 1.0, 1.0, 1.0,  1.0, 0.0, // 21
        0.5, -0.5,  0.5,  1.0, 1.0, 1.0, 1.0,  1.0, 1.0, // 22
        0.5, -0.5, -0.5,  1.0, 1.0, 1.0, 1.0,  0.0, 1.0, // 23
    ];
    let indices = [
        // Front
        0, 1, 2, 2, 3, 0,

        // Back
        4, 5, 6, 6, 7, 4,

        // Left
        8, 9, 10, 10, 11, 8,

        // Right
        12, 13, 14, 14, 15, 12,

        // Top
        16, 17, 18, 18, 19, 16,

        // Bottom
        20, 21, 22, 22, 23, 20,
    ]; 
    let shader = Shader::new("shader/vertex.glsl","shader/fragment.glsl");
    let mut vbo = VertexBuffer::new();
    let mut vao = ArrayBuffer::new();
    let mut ebo = ElementBuffer::new();
    let mut timer = Time::new();
    let mut texture = Texture::new();
    texture.load("res/hoshino.png");
    vao.bind();
    vbo.bind();
    vbo.load_data(&vertices);
    ebo.bind();
    ebo.load_data(&indices);
    vao.vertex_atrrib_pointer(0, 3, 9 * size_of::<f32>() as i32, 0);
    vao.vertex_atrrib_pointer(1, 4, 9 * size_of::<f32>() as i32, 3);
    vao.vertex_atrrib_pointer(2, 2, 9 * size_of::<f32>() as i32, 7);
    let mut camera = Camera::new(800.0, 600.0, glm::vec3(0.0, 0.0, 2.0));
    while !window.should_close(){
        let delta_time = timer.get_delta_time(window.get_instance()) as f32;
        window.clear_color(0.2, 0.3, 0.3);
        window.clear();
        vao.bind();
        shader.use_program();
        camera.matrix(45.0, 0.1, 100.0, &shader, "camMatrix");
        texture.bind(0);
        shader.set_int("tex0", 0);
        window.draw(36, 0);
        camera.handle_inputs(window.get_window() ,delta_time);
        window.swap_buffers();
        window.poll_events();
    }
    vao.delete();
    vbo.delete();
    ebo.delete();
    shader.delete();

}
