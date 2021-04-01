mod compute;
mod graphics;

use compute::Tracer;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut tracer = Tracer::init();
    graphics::run(&mut tracer);
}
