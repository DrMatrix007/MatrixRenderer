use std::marker::PhantomData;

use bytemuck::{Pod, Zeroable};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferAddress, BufferUsages, Device, VertexAttribute, VertexBufferLayout,
};

pub trait Bufferable: Pod + Zeroable {
    fn describe<'a>() -> VertexBufferLayout<'a>;

    fn create_buffer(data: &[Self], indexes: &[u16], device: &Device) -> BufferContainer<Self>;
}

pub struct BufferContainer<T: Bufferable> {
    marker: PhantomData<T>,
    buffer: Buffer,
    index_buffer: Buffer,
    size: usize,
}

impl<T: Bufferable> BufferContainer<T> {
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }
    pub fn index_buffer(&self) -> &Buffer {
        &self.index_buffer
    }

    pub(crate) fn len(&self) -> usize {
        self.size
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub texture_pos: [f32; 2],
}

impl Vertex {
    const ATTRS: [VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];
}
impl Bufferable for Vertex {
    fn describe<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRS,
        }
    }

    fn create_buffer(data: &[Self], indexes: &[u16], device: &Device) -> BufferContainer<Self> {
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("vertex buffer"),
            contents: bytemuck::cast_slice(data),
            usage: BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex index buffer"),
            contents: bytemuck::cast_slice(indexes),
            usage: BufferUsages::INDEX,
        });
        BufferContainer {
            marker: PhantomData,
            buffer,
            index_buffer,
            size: indexes.len(),
        }
    }
}
