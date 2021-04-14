use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::descriptor::{descriptor_set::PersistentDescriptorSet, PipelineLayoutAbstract};
use vulkano::device::{Device, DeviceExtensions, Features, Queue};
use vulkano::instance::{Instance, InstanceExtensions, PhysicalDevice};
use vulkano::pipeline::ComputePipeline;
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer, ImmutableBuffer},
    command_buffer::CommandBuffer,
    sync::GpuFuture,
};

use std::sync::Arc;

mod cs;

const NUM_SPHERES: usize = 5;

#[derive(Copy, Clone)]
struct Sphere {
    center: [f32; 3],
    radius: f32,
}

struct SphereData {
    spheres: [Sphere; NUM_SPHERES],
    specular: [[f32; 4]; NUM_SPHERES],
    albedo: [[f32; 4]; NUM_SPHERES],
}

struct SceneData {
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
    scene_buffer: Arc<ImmutableBuffer<SceneData>>,
    cam_buffer: Arc<CpuAccessibleBuffer<Camera>>,
    sphere_buffer: Arc<CpuAccessibleBuffer<SphereData>>,
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
            center: [0.0, -1.0, -1.0],
            radius: 0.5,
        }; NUM_SPHERES];

        let mut specular = [[0.0, 1.0, 0.0, 0.0]; NUM_SPHERES];
        let mut albedo = [[0.0, 1.0, 0.0, 0.0]; NUM_SPHERES];

        specular[0] = [1.0, 0.0, 0.0, 0.0];
        albedo[0] = [1.0, 0.0, 0.0, 0.0];
        specular[1] = [0.0, 1.0, 0.0, 0.0];
        albedo[1] = [0.0, 1.0, 0.0, 0.0];
        specular[2] = [0.0, 0.0, 1.0, 0.0];
        albedo[2] = [0.0, 0.0, 1.0, 0.0];
        specular[3] = [0.0, 0.0, 0.4, 0.0];
        albedo[3] = [0.0, 0.0, 0.8, 0.0];
        specular[4] = [0.4, 0.0, 0.4, 0.0];
        albedo[4] = [0.6, 0.0, 0.6, 0.0];

        for i in 1..NUM_SPHERES {
            spheres[i].center[0] = -1.0 - i as f32 * 1.5;
        }

        let sun = DirectionalLight {
            direction: [0.5, 1.0, 0.0, 0.5],
        };

        let scene_data = SceneData {
            sun,
            _width: crate::WIDTH as u32,
            _height: crate::HEIGHT as u32,
        };

        let sphere_data = SphereData {
            spheres,
            specular,
            albedo,
        };

        let usage = BufferUsage {
            transfer_source: false,
            transfer_destination: false,
            uniform_texel_buffer: false,
            storage_texel_buffer: false,
            uniform_buffer: false,
            storage_buffer: true,
            index_buffer: false,
            vertex_buffer: false,
            indirect_buffer: false,
            device_address: false,
        };

        let (scene_buffer, future) =
            ImmutableBuffer::from_data(scene_data, usage.clone(), queue.clone())
                .expect("Failed to create buffer");

        future
            .then_signal_fence_and_flush()
            .unwrap()
            .wait(None)
            .expect("Failed to create immutable buffer");

        let sphere_buffer =
            CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), false, sphere_data)
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
            sphere_buffer,
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
                .add_buffer(self.sphere_buffer.clone())
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
