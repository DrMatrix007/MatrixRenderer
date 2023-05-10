use std::any::TypeId;

use matrix_engine::components::component::Component;
use wgpu::{Device, Queue};


use crate::{
    pipelines::{
        buffers::{ Vertex, BufferContainer, VertexBuffer},
        instance_manager::VertexStructure,
        texture::MatrixTexture, structures::plain::Plain,
    },
};

use super::renderer_system::RendererResource;

pub struct RenderObject {
    buffer: Box<dyn VertexStructure<Vertex> + Sync + Send>,
    texture_name: String,
}

impl RenderObject {
    pub fn new(resource: &mut RendererResource) -> Self {
        let image =
            MatrixTexture::from_name("./pic.png".into(), resource.device(), resource.queue(), "pic")
                .unwrap();

        let group = resource.group_layout_manager_mut().get_bind_group_layout::<(MatrixTexture,)>();

        let _group = group.create_bind_group(resource.device(), (&image,));

        Self {
            buffer: Box::new(Plain),
            texture_name: "pic.png".to_string(),
        }
    }

    pub fn texture_name(&self) -> &str {
        &self.texture_name
    }
    pub fn structure_type_id(&self) -> TypeId {
        self.buffer.type_id()
    }
    pub fn create_buffer(&self,device:&Device,queue:&Queue) -> VertexBuffer<Vertex> {
        self.buffer.craete_buffer(device, queue)
    }

}

impl Component for RenderObject {}
