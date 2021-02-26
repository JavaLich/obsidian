use vulkano::instance::{Instance, InstanceExtensions, PhysicalDevice};
use vulkano::device::{Device, DeviceExtensions, Features};

fn main() {
    let instance = Instance::new (None, &InstanceExtensions::none(), None)
        .expect("Failed to create instance");

    let physical = PhysicalDevice::enumerate(&instance).next()
        .expect("No vulkan device available");

    let queue_family = physical.queue_families()
        .find(|&q| q.supports_compute())
        .expect("Couldn't find compute queue family");

    let (device, mut queues) = {
        Device::new(physical, &Features::none(), &DeviceExtensions::none(),
        [(queue_family, 0.5)].iter().cloned()).expect("failed to create device")
    };

    let queue = queues.next().unwrap();
}
