use std::ops::Range;

use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, Buffer, BufferUsages, Device, IndexFormat, Sampler, TextureView,
};

use crate::{
    pipelines::{Pipeline, Renderer2D},
    vertex::Vertex,
};

pub struct BufferData<'a> {
    pub vertex_buffer: &'a Buffer,
    pub index_buffer: &'a Buffer,
    pub index_format: IndexFormat,
}

pub trait Drawable2D {
    fn get_texture_group(&self) -> &BindGroup;
    fn get_vertex_buffer(&self) -> BufferData;
    fn get_verticies_range(&self) -> Range<u32>;
}

pub struct Square {
    data: Buffer,
    indexes: Buffer,
    texture: BindGroup,
}

impl Square {
    const VERTEXES: [Vertex; 4] = [
        Vertex::new([-0.5, -0.5, 0.0], [1.0, 1.0]),
        Vertex::new([0.5, -0.5, 0.0], [0.0, 1.0]),
        Vertex::new([0.5, 0.5, 0.0], [0.0, 0.0]),
        Vertex::new([-0.5, 0.5, 0.0], [1.0, 0.0]),
    ];
    const INDEXES: [u16; 6] = [0, 1, 2, 0, 2, 3];
    pub fn new(
        d: &Device,
        p: &Pipeline<Renderer2D, dyn Drawable2D>,
        v: &TextureView,
        s: &Sampler,
    ) -> Self {
        Self {
            data: d.create_buffer_init(&BufferInitDescriptor {
                label: None,
                usage: BufferUsages::VERTEX,
                contents: bytemuck::cast_slice(&Self::VERTEXES),
            }),
            indexes: d.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&Self::INDEXES),
                usage: BufferUsages::INDEX,
            }),
            texture: p.renderer().create_texture_group(
                d,
                v,s,
            ),
        }
    }
}

impl Drawable2D for Square {
    fn get_texture_group(&self) -> &BindGroup {
        &self.texture
    }

    fn get_vertex_buffer(&self) -> BufferData {
        BufferData {
            vertex_buffer: &self.data,
            index_buffer: &self.indexes,
            index_format: IndexFormat::Uint16,
        }
    }

    fn get_verticies_range(&self) -> Range<u32> {
        0..Self::INDEXES.len() as u32
    }
}
