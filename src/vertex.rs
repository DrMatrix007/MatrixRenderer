use bytemuck::{Pod, Zeroable};
use wgpu::{vertex_attr_array, BufferAddress, VertexAttribute, VertexBufferLayout};


#[repr(C)]
#[derive(Debug,Clone, Copy,Pod,Default,Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub texture_position: [f32; 2],
}

impl Vertex {
    const ATTRS: [VertexAttribute; 2] = vertex_attr_array![0=>Float32x3,1=>Float32x2];

    pub fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRS,
        }
    }

    pub const fn new(position: [f32; 3], texture_position: [f32; 2]) -> Self {
        Self {
            position,
            texture_position,
        }
    }
}
