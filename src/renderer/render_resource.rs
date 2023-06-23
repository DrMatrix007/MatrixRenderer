use matrix_engine::components::resources::Resource;
use vulkano::{
    buffer::Buffer,
    device::{Device, DeviceCreateInfo, QueueCreateInfo, QueueFlags},
    instance::{Instance, InstanceCreateInfo},
    memory::allocator::StandardMemoryAllocator,
    VulkanLibrary,
};

pub struct RenderResource {
    library: VulkanLibrary,
    instance: Instance,
}

impl Resource for RenderResource {}

impl RenderResource {
    pub fn new() {
        let lib = VulkanLibrary::new().expect("no vulkan library");
        let instance =
            Instance::new(lib, InstanceCreateInfo::default()).expect("failed to craete instance");

        let physical_device = instance
            .enumerate_physical_devices()
            .expect("no devices")
            .next()
            .expect("no device available");

        let queue_family_index = physical_device
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(_, props)| props.queue_flags.contains(QueueFlags::GRAPHICS))
            .expect("cant find queue") as u32;

        let (device, mut queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .expect("failed to create device");

        let queue = queues.next().unwrap();

        let memory = StandardMemoryAllocator::new_default(device.clone());
    }
}
