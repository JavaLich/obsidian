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

    let mut pos = [0f32; 3];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        tracer.set_camera_pos(pos);

        let pixels = tracer.compute();
        pos[0] -= 0.01;

        window
            .update_with_buffer(&pixels, crate::WIDTH, crate::HEIGHT)
            .unwrap();
    }
}
