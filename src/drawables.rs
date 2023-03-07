use std::ops::Range;

use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, Buffer, BufferUsages, Device, IndexFormat, Sampler, TextureView,
};

use crate::{
    pipelines::{Pipeline, Renderer3D},
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
pub struct SquareConfig<'a> {
    pub device: &'a Device,
    pub pipeline: &'a Pipeline<Renderer3D, dyn Drawable2D>,
    pub view: &'a TextureView,
    pub sampler: &'a Sampler,
    pub pos: &'a [f32; 3],
    pub size: &'a [f32; 2],
}
impl Square {
    // const VERTEXES: [Vertex; 4] = [
    //     Vertex::new([-0.5, -0.5, 0.0], [1.0, 1.0]),
    //     Vertex::new([0.5, -0.5, 0.0], [0.0, 1.0]),
    //     Vertex::new([0.5, 0.5, 0.0], [0.0, 0.0]),
    //     Vertex::new([-0.5, 0.5, 0.0], [1.0, 0.0]),
    // ];

    fn create_indicies(pos: &[f32; 3], size: &[f32; 2]) -> [Vertex; 4] {
        [
            Vertex::new([pos[0], pos[1], pos[2]], [1.0, 1.0]),
            Vertex::new([pos[0] + size[0], pos[1],  pos[2]], [0.0, 1.0]),
            Vertex::new([pos[0] + size[0], pos[1] + size[1],  pos[2]], [0.0, 0.0]),
            Vertex::new([pos[0], pos[1] + size[1],  pos[2]], [1.0, 0.0]),
        ]
    }

    const INDEXES: [u16; 6] = [0, 1, 2, 0, 2, 3];
    pub fn new(conf: &SquareConfig) -> Self {
        Self {
            data: conf.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                usage: BufferUsages::VERTEX,
                contents: bytemuck::cast_slice(&Self::create_indicies(conf.pos, conf.size)),
            }),
            indexes: conf.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&Self::INDEXES),
                usage: BufferUsages::INDEX,
            }),
            texture: conf.pipeline.renderer().create_texture_group(
                conf.device,
                conf.view,
                conf.sampler,
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
