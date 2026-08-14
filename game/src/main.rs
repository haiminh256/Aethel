use aethel::core::window::Window;
use aethel::math::time::Time;
use aethel::renderer::Renderer;
use aethel::renderer::texture::Texture;
use aethel::shader::Shader;
use aethel::vertex::arraybuffer::ArrayBuffer;
use aethel::vertex::elementbuffer::ElementBuffer;
use aethel::vertex::vertexbuffer::VertexBuffer;
use aethel::renderer::camera::Camera;
use aethel::vertex::Vertex;
use aethel::renderer::transform::Transform;
use nalgebra_glm as glm;

fn main(){
    let mut window: Window = Window::new(800, 600, "Game Engine");
    window.make_context_current();
    let renderer: Renderer = Renderer::new();
    renderer.init(&mut window);

    let vertices = [
        // Front (+Z)
        Vertex::new(glm::vec3( 0.5, -0.5,  0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.0, 0.0, 0.0), glm::vec2(0.0, 0.0)), // 0
        Vertex::new(glm::vec3(-0.5, -0.5,  0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.0, 0.0, 0.0), glm::vec2(1.0, 0.0)), // 1
        Vertex::new(glm::vec3(-0.5,  0.5,  0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.0, 0.0, 0.0), glm::vec2(1.0, 1.0)), // 2
        Vertex::new(glm::vec3( 0.5,  0.5,  0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.0, 0.0, 0.0), glm::vec2(0.0, 1.0)), // 3

        // Back (-Z)
        Vertex::new(glm::vec3( 0.5, -0.5, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.0, 0.0, 0.0), glm::vec2(0.0, 0.0)), // 4
        Vertex::new(glm::vec3(-0.5, -0.5, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.0, 0.0, 0.0), glm::vec2(1.0, 0.0)), // 5
        Vertex::new(glm::vec3(-0.5,  0.5, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.0, 0.0, 0.0), glm::vec2(1.0, 1.0)), // 6
        Vertex::new(glm::vec3( 0.5,  0.5, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.0, 0.0, 0.0), glm::vec2(0.0, 1.0)), // 7

        // Left (-X)
        Vertex::new(glm::vec3(-0.5, -0.5, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.0, 0.0, 0.0), glm::vec2(0.0, 0.0)), // 8
        Vertex::new(glm::vec3(-0.5, -0.5,  0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.0, 0.0, 0.0), glm::vec2(1.0, 0.0)), // 9
        Vertex::new(glm::vec3(-0.5,  0.5,  0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.0, 0.0, 0.0), glm::vec2(1.0, 1.0)), // 10
        Vertex::new(glm::vec3(-0.5,  0.5, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.0, 0.0, 0.0), glm::vec2(0.0, 1.0)), // 11

        // Right (+X)
        Vertex::new(glm::vec3( 0.5, -0.5,  0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.0, 0.0, 0.0), glm::vec2(0.0, 0.0)), // 12
        Vertex::new(glm::vec3( 0.5, -0.5, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.0, 0.0, 0.0), glm::vec2(1.0, 0.0)), // 13
        Vertex::new(glm::vec3( 0.5,  0.5, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.0, 0.0, 0.0), glm::vec2(1.0, 1.0)), // 14
        Vertex::new(glm::vec3( 0.5,  0.5,  0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.0, 0.0, 0.0), glm::vec2(0.0, 1.0)), // 15

        // Top (+Y)
        Vertex::new(glm::vec3(-0.5,  0.5,  0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.0, 0.0, 0.0), glm::vec2(0.0, 0.0)), // 16
        Vertex::new(glm::vec3(-0.5,  0.5, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.0, 0.0, 0.0), glm::vec2(1.0, 0.0)), // 17
        Vertex::new(glm::vec3( 0.5,  0.5, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.0, 0.0, 0.0), glm::vec2(1.0, 1.0)), // 18
        Vertex::new(glm::vec3( 0.5,  0.5,  0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.0, 0.0, 0.0), glm::vec2(0.0, 1.0)), // 19

        // Bottom (-Y)
        Vertex::new(glm::vec3(-0.5, -0.5, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.0, 0.0, 0.0), glm::vec2(0.0, 0.0)), // 20
        Vertex::new(glm::vec3(-0.5, -0.5,  0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.0, 0.0, 0.0), glm::vec2(1.0, 0.0)), // 21
        Vertex::new(glm::vec3( 0.5, -0.5,  0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.0, 0.0, 0.0), glm::vec2(1.0, 1.0)), // 22
        Vertex::new(glm::vec3( 0.5, -0.5, -0.5), glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.0, 0.0, 0.0), glm::vec2(0.0, 1.0)), // 23
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
    vao.vertex_atrrib_pointer(0, 3, std::mem::size_of::<Vertex>() as i32, std::mem::offset_of!(Vertex, position));
    vao.vertex_atrrib_pointer(1, 4, std::mem::size_of::<Vertex>() as i32, std::mem::offset_of!(Vertex, color));
    vao.vertex_atrrib_pointer(2, 3, std::mem::size_of::<Vertex>() as i32, std::mem::offset_of!(Vertex, normal));
    vao.vertex_atrrib_pointer(3, 2, std::mem::size_of::<Vertex>() as i32, std::mem::offset_of!(Vertex, tex_coord));
    let mut camera = Camera::new(800.0, 600.0, glm::vec3(0.0, 0.0, 2.0));

    let mut cube_transform = Transform::new();
    cube_transform.position = glm::vec3(0.0, 0.0, 0.0);
    while !window.should_close(){
        let delta_time = timer.get_delta_time(window.get_instance()) as f32;
        window.clear_color(0.2, 0.3, 0.3);
        window.clear();
        vao.bind();
        cube_transform.rotation.y += 50.0 * delta_time;
        shader.use_program();
        shader.set_mat4("model", &cube_transform.get_model_matrix());
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
