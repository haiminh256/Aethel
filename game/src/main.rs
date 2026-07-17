use aethel::core::window::Window;
use aethel::math::time::Time;
use aethel::renderer::Renderer;
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
        0.5,  0.5, 0.0,
        0.5, -0.5, 0.0,
       -0.5, -0.5, 0.0,
       -0.5,  0.5, 0.0
    ];
    let indices = [
        0, 1, 3,
        1, 2, 3
    ]; 
    let shader = Shader::new("shader/vertex.glsl","shader/fragment.glsl");
    let mut vbo = VertexBuffer::new();
    let mut vao = ArrayBuffer::new();
    let mut ebo = ElementBuffer::new();
    let mut timer = Time::new();
    vao.bind();
    vbo.bind();
    vbo.load_data(&vertices);
    ebo.bind();
    ebo.load_data(&indices);
    vao.vertex_atrrib_pointer(0, 3, 3 * size_of::<f32>() as i32, 0);
    let mut camera = Camera::new(800.0, 600.0, glm::vec3(0.0, 0.0, 2.0));
    while !window.should_close(){
        let delta_time = timer.get_delta_time(window.get_instance()) as f32;
        window.clear_color(0.2, 0.3, 0.3);
        window.clear();
        vao.bind();
        shader.use_program();
        camera.matrix(45.0, 0.1, 100.0, &shader, "camMatrix");
        window.draw(6, 0);
        camera.handle_inputs(window.get_window() ,delta_time);
        window.swap_buffers();
        window.poll_events();
    }
    vao.delete();
    vbo.delete();
    ebo.delete();
    shader.delete();

}
