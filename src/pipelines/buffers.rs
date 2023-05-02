use std::marker::PhantomData;

use bytemuck::{Pod, Zeroable};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferAddress, BufferUsages, Device, VertexAttribute, VertexBufferLayout,
};

pub struct BufferContainer<T: Bufferable> {
    marker: PhantomData<T>,
    buffer: Buffer,
    index_buffer: Option<Buffer>,
    size: usize,
}

impl<T: Bufferable> BufferContainer<T> {
    pub fn new(buffer: Buffer, index_buffer: Option<Buffer>, size: usize) -> Self {
        Self {
            marker: PhantomData,
            buffer,
            index_buffer,
            size,
        }
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }
    pub fn index_buffer(&self) -> Option<&Buffer> {
        self.index_buffer.as_ref()
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

pub trait IntoBytes<T: Bufferable> {
    fn get_bytes(&self) -> &[u8];
    fn size(&self) -> usize;
}

impl<T: Bufferable> IntoBytes<T> for T {
    fn get_bytes(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }

    fn size(&self) -> usize {
        1
    }
}
impl<T: Bufferable, const N: usize> IntoBytes<T> for [T; N] {
    fn get_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(self)
    }

    fn size(&self) -> usize {
        N
    }
}
impl<T: Bufferable> IntoBytes<T> for [T] {
    fn get_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(self)
    }

    fn size(&self) -> usize {
        <[T]>::len(self)
    }
}
impl<T: Bufferable> IntoBytes<T> for &'_ [T] {
    fn get_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(self)
    }

    fn size(&self) -> usize {
        <[T]>::len(self)
    }
}

pub trait Bufferable: Pod + Zeroable {
    fn describe<'a>() -> VertexBufferLayout<'a>;
    fn create_buffer(
        data: &dyn IntoBytes<Self>,
        indexes: Option<&[u16]>,
        device: &Device,
    ) -> BufferContainer<Self>;
}

impl Bufferable for Vertex {
    fn create_buffer(
        data: &dyn IntoBytes<Self>,
        indexes: Option<&[u16]>,
        device: &Device,
    ) -> BufferContainer<Self> {
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("vertex buffer"),
            contents: data.get_bytes(),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = indexes.map(|indexes| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("vertex index buffer"),
                contents: bytemuck::cast_slice(indexes),
                usage: BufferUsages::INDEX,
            })
        });
        BufferContainer {
            marker: PhantomData,
            buffer,
            index_buffer,
            size: indexes.map(|x| x.len()).unwrap_or_else(|| data.size()),
        }
    }

    fn describe<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRS,
        }
    }
}
