use minifb::{Key, Window, WindowOptions};

pub fn init(pixels: &Vec<u32>) {
    let mut window = Window::new("RayTracer-RS", crate::WIDTH, crate::HEIGHT, 
        WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&pixels, crate::WIDTH, crate::HEIGHT)
            .unwrap();
    }
}
