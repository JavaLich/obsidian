use crate::compute::Tracer;

use minifb::{Key, Window, WindowOptions};

use nalgebra::{Vector3, Rotation3, Matrix3, Matrix4, Point3, Point4};

use std::f32;

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

    let mut axisangle = Vector3::z() * f32::consts::FRAC_1_PI;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::W) {
            tracer.change_camera_pos(0f32, 0f32, -speed);
        }
        if window.is_key_down(Key::S) {
            tracer.change_camera_pos(0f32, 0f32, speed);
        }
        if window.is_key_down(Key::A) {
            tracer.change_camera_pos(-speed, 0f32, 0f32);
        }
        if window.is_key_down(Key::D) {
            tracer.change_camera_pos(speed, 0f32, 0f32);
        }
        if window.is_key_down(Key::LeftShift) {
            tracer.change_camera_pos(0f32, speed, 0f32);
        }
        if window.is_key_down(Key::Space) {
            tracer.change_camera_pos(0f32, -speed, 0f32);
        }
        // rotate (0, 0, 1) based on keybindings
        // rotation matrix?
        if window.is_key_down(Key::Left) {
            axisangle = Vector3::z();
        }
        if window.is_key_down(Key::Right) {
            axisangle.x += 0.1;
        }
        if window.is_key_down(Key::Up) {
            axisangle.y -= 0.1;
        }
        if window.is_key_down(Key::Down) {
            axisangle.y += 0.1;
        }
        let dir = Vector3::new(0.0, 0.0, 1.0);
        let up = Vector3::y();
        
        let rot = Rotation3::face_towards(&dir, &up);
        let result = rot * Vector3::new(0., 0., 1.);
        tracer.set_camera_direction(result.x, result.y, result.z);

        let pixels = tracer.compute();

        window
            .update_with_buffer(&pixels, crate::WIDTH, crate::HEIGHT)
            .unwrap();
    }
}
