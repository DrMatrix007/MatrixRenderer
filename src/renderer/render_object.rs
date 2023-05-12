use std::any::TypeId;

use matrix_engine::components::component::Component;
use wgpu::{Device, Queue};

use crate::pipelines::{
    buffers::{Vertex, VertexBuffer},
    instance_manager::VertexStructure,
};



pub struct RenderObject {
    buffer: Box<dyn VertexStructure<Vertex> + Sync + Send>,
    texture_name: String,
}

impl RenderObject {
    pub fn new(
        structure: impl VertexStructure<Vertex> + Send + Sync,
        texture_name: String,
    ) -> Self {
        Self {
            buffer: Box::new(structure),
            texture_name,
        }
    }

    pub fn texture_name(&self) -> &str {
        &self.texture_name
    }
    pub fn structure_type_id(&self) -> TypeId {
        self.buffer.type_id()
    }
    pub fn create_buffer(&self, device: &Device, queue: &Queue) -> VertexBuffer<Vertex> {
        self.buffer.craete_buffer(device, queue)
    }
}

impl Component for RenderObject {}
