mod compute;
mod graphics;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let pixels = compute::init();
    graphics::init(&pixels);
}
