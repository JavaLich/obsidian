use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::descriptor::{descriptor_set::PersistentDescriptorSet, PipelineLayoutAbstract};
use vulkano::device::{Device, DeviceExtensions, Features, Queue};
use vulkano::instance::{Instance, InstanceExtensions, PhysicalDevice};
use vulkano::pipeline::ComputePipeline;
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    command_buffer::CommandBuffer,
    sync::GpuFuture,
};

use std::sync::Arc;

mod cs;

const NUM_SPHERES: usize = 10;

#[derive(Copy, Clone)]
struct Sphere {
    center: [f32; 3],
    radius: f32,
}

struct SceneData {
    spheres: [Sphere; NUM_SPHERES],
    sun: DirectionalLight,
    _width: u32,
    _height: u32,
}

struct DirectionalLight {
    direction: [f32; 4],
}

struct Camera {
    position: [f32; 3],
    _vp_height: f32,
    _focal_length: f32,
}

pub struct Tracer {
    device: Arc<Device>,
    queue: Arc<Queue>,
    data_buffer: Arc<CpuAccessibleBuffer<[u32; crate::WIDTH * crate::HEIGHT]>>,
    scene_buffer: Arc<CpuAccessibleBuffer<SceneData>>,
    cam_buffer: Arc<CpuAccessibleBuffer<Camera>>,
    shader: cs::Shader,
}

impl Tracer {
    pub fn init() -> Tracer {
        let instance = Instance::new(None, &InstanceExtensions::none(), None)
            .expect("Failed to create instance");

        let physical = PhysicalDevice::enumerate(&instance)
            .next()
            .expect("No vulkan device available");

        let queue_family = physical
            .queue_families()
            .find(|&q| q.supports_compute())
            .expect("Couldn't find compute queue family");

        let device_ext = DeviceExtensions {
            khr_storage_buffer_storage_class: true,
            ..DeviceExtensions::none()
        };

        let (device, mut queues) = {
            Device::new(
                physical,
                &Features::none(),
                &device_ext,
                [(queue_family, 0.5)].iter().cloned(),
            )
            .expect("failed to create device")
        };

        let queue = queues.next().unwrap();

        let data = [0u32; crate::WIDTH * crate::HEIGHT];
        let data_buffer =
            CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), false, data)
                .expect("Failed to create buffer");

        let mut spheres = [Sphere {
            center: [0.0, 0.0, -1.0],
            radius: 0.5,
        }; NUM_SPHERES];

        for i in 1..NUM_SPHERES - 1 {
            spheres[i].center[0] = -1.0 - i as f32 * 1.5;
        }

        spheres[9].center = [0.0, 100.5, -1.0];
        spheres[9].radius = 100f32;

        let sun = DirectionalLight {
            direction: [0.0, 1.0, -1.0, 0.5],
        };

        let scene_data = SceneData {
            spheres,
            sun,
            _width: crate::WIDTH as u32,
            _height: crate::HEIGHT as u32,
        };
        let scene_buffer =
            CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), false, scene_data)
                .expect("Failed to create buffer");
        let cam = Camera {
            position: [0.0, -1.0, 1.0],
            _vp_height: 2.0,
            _focal_length: 1.0,
        };
        let cam_buffer =
            CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), false, cam)
                .expect("Failed to create buffer");

        let shader = cs::Shader::load(device.clone()).expect("Failed to create shader module");

        return Tracer {
            device,
            queue,
            data_buffer,
            scene_buffer,
            cam_buffer,
            shader,
        };
    }

    pub fn set_camera_pos(&mut self, pos: [f32; 3]) {
        let mut content = self.cam_buffer.write().unwrap();
        content.position = pos;
    }

    pub fn change_camera_pos(&mut self, x: f32, y: f32, z: f32) {
        let mut content = self.cam_buffer.write().unwrap();
        content.position[0] += x;
        content.position[1] += y;
        content.position[2] += z;
    }

    pub fn compute(&self) -> Vec<u32> {
        let compute_pipeline = Arc::new(
            ComputePipeline::new(
                self.device.clone(),
                &self.shader.main_entry_point(),
                &(),
                None,
            )
            .expect("Failed to create compute pipeline"),
        );

        let layout = compute_pipeline.layout().descriptor_set_layout(0).unwrap();

        let set = Arc::new(
            PersistentDescriptorSet::start(layout.clone())
                .add_buffer(self.data_buffer.clone())
                .unwrap()
                .add_buffer(self.scene_buffer.clone())
                .unwrap()
                .add_buffer(self.cam_buffer.clone())
                .unwrap()
                .build()
                .unwrap(),
        );

        let mut builder =
            AutoCommandBufferBuilder::new(self.device.clone(), self.queue.family()).unwrap();

        builder
            .dispatch([10000, 1, 1], compute_pipeline.clone(), set.clone(), ())
            .unwrap();

        let command_buffer = builder.build().unwrap();

        let finished = command_buffer.execute(self.queue.clone()).unwrap();

        finished
            .then_signal_fence_and_flush()
            .unwrap()
            .wait(None)
            .unwrap();

        let content = self.data_buffer.read().unwrap();

        content.to_vec()
    }
}
