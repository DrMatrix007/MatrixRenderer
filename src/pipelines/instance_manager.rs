use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

use wgpu::{Device, Queue};

use crate::renderer::render_object::RenderObject;

use super::{
    bind_groups::BindGroupContainer,
    buffers::{Bufferable, VertexBuffer, BufferContainer},
    texture::{self, MatrixTexture}, transform::RawTransform,
};

pub trait VertexStructure<Vertex: Bufferable>: Any {
    fn craete_buffer(&self, device: &Device, queue: &Queue) -> VertexBuffer<Vertex>;
}

struct InstancedData {
    texture: MatrixTexture,
    texture_group: BindGroupContainer<(MatrixTexture,)>,
    transform_buffer: BufferContainer<RawTransform>
}

impl InstancedData {
    pub fn new(texture_name: &str, device: &Device, queue: &Queue) {}

    fn prepare_capacity(&self, count: usize) {
        
    }
}

pub struct InstanceManager {
    device: Arc<Device>,
    queue: Arc<Queue>,
    data: HashMap<(TypeId, String), (usize, Option<InstancedData>)>,
}

impl InstanceManager {
    pub fn new(device: Arc<Device>, queue: Arc<Queue>) -> Self {
        Self {
            device,
            queue,
            data: Default::default(),
        }
    }

    fn registr_object(&mut self, obj: &RenderObject) {
        self.data
            .entry((obj.structure_type_id(), obj.texture_name().into()))
            .and_modify(|(x, _)| *x += 1)
            .or_insert((1, None));
    }
    fn prepare(&mut self) {
        self.data.retain(|_, (x, _)| x > &mut 0);

        for ((structure, texture_name), (count, data)) in &mut self.data {
            data.as_mut().map(|x| {
                x.prepare_capacity(*count)
            });
        }
    }
}
