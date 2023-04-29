use std::{
    fs::{self, File},
    io::Read,
};

use wgpu::{Device, ShaderModuleDescriptor};

pub struct Shaders {
    module: wgpu::ShaderModule,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
struct FileNotFound;

impl Shaders {
    pub fn new(device: Device, filename: String, label: &str) -> Result<Self, FileNotFound> {
        let mut shader = fs::read_to_string(filename)?;

        let module = device.create_shader_module(ShaderModuleDescriptor {
            label: Some(label),
            source: wgpu::ShaderSource::Wgsl(shader),
        });

        Ok(Self { module })
    }
}
