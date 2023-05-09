use std::marker::PhantomData;

use bytemuck::{Pod, Zeroable};
use matrix_engine::impl_all;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferAddress, BufferDescriptor, BufferUsages, Device, Queue, VertexAttribute,
    VertexBufferLayout,
};

pub struct BufferContainer<T: Pod + Zeroable> {
    marker: PhantomData<T>,
    buffer: Buffer,
    size: u64,
}

impl<T: Pod + Zeroable> BufferContainer<T> {
    pub fn new(buffer: Buffer, size: u64) -> Self {
        Self {
            marker: PhantomData,
            buffer,
            size,
        }
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn size(&self) -> u64 {
        self.size
    }
    pub fn create_buffer(
        data: &dyn IntoBytes<T>,
        device: &Device,
        usage: BufferUsages,
    ) -> BufferContainer<T> {
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("vertex buffer"),
            contents: data.get_bytes(),
            usage,
        });

        BufferContainer {
            marker: PhantomData,
            buffer,
            size: data.size() as u64,
        }
    }

    pub fn clone_data_with_size(
        &self,
        device: &Device,
        queue: &Queue,
        new_obj_len: u64,
        label: &str,
    ) -> Self {
        let t_size = std::mem::size_of::<T>() as u64;

        let buffer = device.create_buffer(&BufferDescriptor {
            label: Some(label),
            size: (new_obj_len * t_size),
            usage: self.buffer.usage(),
            mapped_at_creation: false,
        });
        let write_size = (new_obj_len).min(self.size);

        queue.write_buffer(
            &buffer,
            0,
            &self
                .buffer
                .slice(0..(t_size * write_size))
                .get_mapped_range(),
        );

        if write_size < new_obj_len {
            let mut v = Vec::new();
            v.resize(((new_obj_len - write_size) * t_size) as usize, 0);
            queue.write_buffer(&buffer, write_size * t_size, &v);
        }
        Self::new(buffer, new_obj_len)
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

pub trait IntoBytes<T: Pod + Zeroable> {
    fn get_bytes(&self) -> &[u8];
    fn size(&self) -> usize;
}

impl<T: Pod + Zeroable> IntoBytes<T> for T {
    fn get_bytes(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }

    fn size(&self) -> usize {
        1
    }
}
impl<T: Pod + Zeroable, const N: usize> IntoBytes<T> for [T; N] {
    fn get_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(self)
    }

    fn size(&self) -> usize {
        N
    }
}
impl<T: Pod + Zeroable> IntoBytes<T> for [T] {
    fn get_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(self)
    }

    fn size(&self) -> usize {
        <[T]>::len(self)
    }
}
impl<T: Pod + Zeroable> IntoBytes<T> for &'_ [T] {
    fn get_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(self)
    }

    fn size(&self) -> usize {
        <[T]>::len(self)
    }
}

pub trait Bufferable: Pod + Zeroable {
    fn describe<'a>() -> VertexBufferLayout<'a>;
}

impl Bufferable for Vertex {
    fn describe<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRS,
        }
    }
}

pub struct VertexBuffer<Vertex: Bufferable> {
    buffer: BufferContainer<Vertex>,
    index_buffer: Option<BufferContainer<u16>>,
}

impl<Vertex: Bufferable> VertexBuffer<Vertex> {
    pub fn new(
        buffer: BufferContainer<Vertex>,
        index_buffer: Option<BufferContainer<u16>>,
    ) -> Self {
        Self {
            buffer,
            index_buffer,
        }
    }
}



pub trait BufferGroup {
    fn describe<'a>() -> Vec<VertexBufferLayout<'a>>;
}


macro_rules! impl_buffer_group {
    ($($t:ident),+) => {
        impl<$($t:Bufferable,)+> BufferGroup for ($($t,)+) {
            fn describe<'a>() -> Vec<VertexBufferLayout<'a>> {
                vec![
                    $($t::describe(),)+                 
                ]            
            }
        }
    }
}

impl_all!(impl_buffer_group);