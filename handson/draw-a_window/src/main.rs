extern crate glfw;
use glfw::{Action, Context, Key};

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let (mut window, events) = glfw
        .create_window(800, 600, "Hello OpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create window.");

    window.make_current();
    window.set_key_polling(true);

    while !window.should_close() {
        gl::load_with(|s| window.get_proc_address(s) as *const _);
        unsafe {
            gl::ClearColor(0.6, 0.3, 0.4, 0.75);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }

        window.swap_buffers();
    }
}
