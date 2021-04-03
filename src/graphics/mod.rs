use crate::compute::Tracer;

use minifb::{Key, Window, WindowOptions};

pub fn run(tracer: &mut Tracer) {
    let mut window = Window::new(
        "RayTracer-RS",
        crate::WIDTH,
        crate::HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let speed = 0.1;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::W) {
            tracer.change_camera_pos(0f32, 0f32, speed);
        }
        if window.is_key_down(Key::S) {
            tracer.change_camera_pos(0f32, 0f32, -speed);
        }
        if window.is_key_down(Key::A) {
            tracer.change_camera_pos(-speed, 0f32, 0f32);
        }
        if window.is_key_down(Key::D) {
            tracer.change_camera_pos(speed, 0f32, 0f32);
        }

        let pixels = tracer.compute();

        window
            .update_with_buffer(&pixels, crate::WIDTH, crate::HEIGHT)
            .unwrap();
    }
}
