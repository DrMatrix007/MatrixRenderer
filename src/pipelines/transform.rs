use bytemuck::{Pod, Zeroable};
use matrix_engine::components::component::Component;
use wgpu::{BufferAddress, VertexAttribute};

use crate::math::matrices::Matrix4;

use super::buffers::Bufferable;

pub struct Transform {
    data: Matrix4<f32>,
}

impl Component for Transform {}

impl Transform {
    pub fn raw(&self) -> RawTransform {
        self.data.clone().into()
    }
    pub fn identity() -> Self {
        Self {
            data: Matrix4::identity(),
        }
    }
}


#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct RawTransform {
    data: [[f32; 4]; 4],
}

impl From<Matrix4<f32>> for RawTransform {
    fn from(value: Matrix4<f32>) -> Self {
        Self { data: value.into() }
    }
}

impl Default for RawTransform {
    fn default() -> Self {
        Self {
            data: Matrix4::identity().into(),
        }
    }
}

impl RawTransform {
    const ATTRS: &[VertexAttribute] = &[
        wgpu::VertexAttribute {
            offset: 0,
            shader_location: 5,
            format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
            offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
            shader_location: 6,
            format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
            offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
            shader_location: 7,
            format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
            offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
            shader_location: 8,
            format: wgpu::VertexFormat::Float32x4,
        },
    ];
}

impl Bufferable for RawTransform {
    fn describe<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<RawTransform>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: Self::ATTRS,
        }
    }
}
