use wgpu::{BufferUsages, Device, Queue};

use crate::pipelines::{buffers::{Vertex, VertexBuffer, BufferContainer}, instance_manager::VertexStructure};

pub struct Plain;

impl VertexStructure<Vertex> for Plain {
    fn craete_buffer(&self, device: &Device, _queue: &Queue) -> VertexBuffer<Vertex> {
        VertexBuffer::new(
            BufferContainer::<Vertex>::create_buffer(
                &Self::VERTICES,
                device,
                BufferUsages::COPY_DST | BufferUsages::VERTEX,
            ),
            Some(BufferContainer::<u16>::create_buffer(
                &Self::INDEXES,
                device,
                BufferUsages::INDEX | BufferUsages::COPY_DST,
            )),
        )
    }
}

impl Plain {
    const VERTICES: &[Vertex] = &[
        Vertex {
            position: [-0.5, 0.5, 0.0],
            texture_pos: [0., 0.],
        },
        Vertex {
            position: [0.5, 0.5, 0.0],
            texture_pos: [1.0, 0.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.0],
            texture_pos: [1.0, 1.0],
        },
        Vertex {
            position: [-0.5, -0.5, 0.0],
            texture_pos: [0.0, 1.0],
        },
    ];
    const INDEXES: &[u16] = &[0, 2, 1, 0, 3, 2];
}
