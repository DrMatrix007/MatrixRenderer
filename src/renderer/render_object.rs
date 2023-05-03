use matrix_engine::components::component::Component;
use wgpu::BufferUsages;

use crate::{
    pipelines::{
        bind_groups::BindGroupContainer,
        buffers::{BufferContainer, Vertex},
        texture::MatrixTexture,
    },
    texture,
};

use super::renderer_system::RendererResource;

pub struct RenderObject {
    buffer: BufferContainer<Vertex>,
    index_buffer: BufferContainer<u16>,
    texture: MatrixTexture,
    texture_group: BindGroupContainer<(MatrixTexture,)>,
}

impl RenderObject {
    pub fn new(resource: &mut RendererResource) -> Self {
        let image = texture!("pic.png", resource.device(), resource.queue(), "pic").unwrap();

        let group = resource.get_bind_group_layout::<(MatrixTexture,)>();

        let group = group.create_bind_group(resource.device(), (&image,));

        Self {
            buffer: BufferContainer::<Vertex>::create_buffer(
                &Self::VERTICES,
                resource.device(),
                BufferUsages::COPY_DST | BufferUsages::VERTEX,
            ),
            index_buffer: BufferContainer::<u16>::create_buffer(
                &Self::INDEXES,
                resource.device(),
                BufferUsages::INDEX | BufferUsages::COPY_DST,
            ),
            texture: image,
            texture_group: group,
        }
    }

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

    pub(crate) fn texture_group(&self) -> &BindGroupContainer<(MatrixTexture,)> {
        &self.texture_group
    }

    pub(crate) fn buffer(&self) -> &BufferContainer<Vertex> {
        &self.buffer
    }

    pub(crate) fn index_buffer(&self) -> &BufferContainer<u16> {
        &self.index_buffer
    }
}

impl Component for RenderObject {}
