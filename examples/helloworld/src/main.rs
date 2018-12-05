extern crate gl;
// include the OpenGL type aliases
use gl::types::*;

extern crate glfw;

use glfw::{Action, Context, Key};

fn main() {
   let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();

    gl::load_with(|s| glfw.get_proc_address_raw(s));

    // loading a specific function pointer
    gl::Viewport::load_with(|s| glfw.get_proc_address_raw(s));

    window.set_key_polling(true);

    // Loop until the user closes the window
    while !window.should_close() {
        // render
        unsafe{
            gl::ClearColor(1.0,0.0,0.0,0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Swap front and back buffers
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                },
                _ => {},
            }
        }
    }
}