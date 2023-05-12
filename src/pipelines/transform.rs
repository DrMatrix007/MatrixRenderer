use bytemuck::{Pod, Zeroable};
use matrix_engine::components::component::Component;
use wgpu::{BufferAddress, VertexAttribute};

use crate::math::{
    matrices::{Matrix4, Vector3},
    vectors::Vector3D,
};

use super::buffers::Bufferable;

pub struct Transform {
    pub position: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub rotate: Vector3<f32>,
}

impl Component for Transform {}

impl Transform {
    pub fn identity() -> Self {
        Self {
            position: Vector3::zeros(),
            scale: Vector3::ones(),
            rotate: Vector3::zeros(),
        }
    }
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct InstanceTransform {
    data: [[f32; 4]; 4],
}

impl From<&Transform> for InstanceTransform {
    fn from(value: &Transform) -> Self {
        let scale = Matrix4::from([
            [*value.scale.x(), 0.0, 0.0, 0.0],
            [0.0, *value.scale.y(), 0.0, 0.0],
            [0.0, 0.0, *value.scale.z(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let pos = Matrix4::from([
            [1., 0., 0., *value.position.x()],
            [0., 1., 0., *value.position.y()],
            [0., 0., 1., *value.position.z()],
            [0., 0., 0., 1.],
        ]);

        let rotate = Matrix4::rotate_x(*value.rotate.x())
            * Matrix4::rotate_y(*value.rotate.y())
            * Matrix4::rotate_z(*value.rotate.z());
        (pos * rotate * scale).into()
    }
}

impl From<Matrix4<f32>> for InstanceTransform {
    fn from(value: Matrix4<f32>) -> Self {
        Self { data: value.into() }
    }
}

impl Default for InstanceTransform {
    fn default() -> Self {
        Self {
            data: Matrix4::identity().into(),
        }
    }
}

impl InstanceTransform {
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

impl Bufferable for InstanceTransform {
    fn describe<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceTransform>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: Self::ATTRS,
        }
    }
}
