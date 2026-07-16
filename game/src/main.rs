use aethel::core::window::Window;

fn main(){
    let mut window: Window = Window::new(800, 600, "Game Engine");

    while !window.should_close(){
        window.swap_buffers();
        window.poll_events();
    }
}

