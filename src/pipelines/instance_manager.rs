use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

use wgpu::{BufferUsages, Device, Queue};

use crate::renderer::render_object::RenderObject;

use super::{
    bind_groups::BindGroupContainer,
    buffers::{BufferContainer, Bufferable, Vertex, VertexBuffer},
    group_layout_manager::BindGroupLayoutManager,
    texture::MatrixTexture,
    transform::{InstanceTransform, Transform},
};

pub trait VertexStructure<Vertex: Bufferable>: Any {
    fn craete_buffer(&self, device: &Device, queue: &Queue) -> VertexBuffer<Vertex>;
}

pub struct InstancedData {
    texture: MatrixTexture,
    texture_group: BindGroupContainer<(MatrixTexture,)>,
    transform_buffer: BufferContainer<InstanceTransform>,
    transform_vec: Vec<InstanceTransform>,
    buffer: Arc<VertexBuffer<Vertex>>,
}

impl InstancedData {
    pub fn new(
        texture_name: &str,
        device: &Device,
        queue: &Queue,
        buffer: Arc<VertexBuffer<Vertex>>,
        manager: &mut BindGroupLayoutManager,
    ) -> Self {
        let t =
            MatrixTexture::from_name(texture_name, device, queue, "instanced generated texture")
                .expect("this shouldnt be implemnted now");
        let group = manager.create_group::<(MatrixTexture,)>((&t,));
        Self {
            texture: t,
            buffer,
            transform_vec: Vec::new(),
            texture_group: group,
            transform_buffer: BufferContainer::create_buffer(
                &InstanceTransform::default(),
                device,
                queue,
                BufferUsages::VERTEX | BufferUsages::COPY_DST,
                true,
            ),
        }
    }

    fn prepare_capacity(&mut self, _count: u64, device: &Device, queue: &Queue) -> bool {
        // if self.transform_buffer.size() < count || (self.transform_buffer.size() / 2) > count {
        //     let new_size = (2_u32).pow((count as f32).log2().ceil() as u32);
        //     self.transform_buffer = BufferContainer::create_with_size(
        //         count,
        //         device,
        //         self.transform_buffer.usage(),
        //         true,
        //     );
        //     true
        // } else {
        //     false
        // }
        if self.transform_buffer.size() as usize != self.transform_vec.capacity() {
            self.transform_buffer = BufferContainer::create_with_size(
                self.transform_vec.capacity() as u64,
                device,
                BufferUsages::COPY_DST | BufferUsages::VERTEX,
                false,
            );
            queue.write_buffer(
                self.transform_buffer.buffer(),
                0,
                bytemuck::cast_slice(&self.transform_vec),
            );
            return true;
        }
        false
    }

    pub fn texture_group(&self) -> &BindGroupContainer<(MatrixTexture,)> {
        &self.texture_group
    }

    pub fn transform_buffer(&self) -> &BufferContainer<InstanceTransform> {
        &self.transform_buffer
    }

    pub fn structure_buffer(&self) -> &VertexBuffer<Vertex> {
        self.buffer.as_ref()
    }

    pub fn push(&mut self, raw: InstanceTransform, _device: &Device, _queue: &Queue) {
        self.transform_vec.push(raw);

        if self.transform_vec.capacity() != self.transform_buffer.size() as usize {}
    }
}

pub struct InstanceManager {
    device: Arc<Device>,
    queue: Arc<Queue>,
    data: HashMap<(TypeId, String), (u64, InstancedData)>,
    buffer: HashMap<TypeId, (u64, Arc<VertexBuffer<Vertex>>)>,
}

impl InstanceManager {
    pub fn new(device: Arc<Device>, queue: Arc<Queue>) -> Self {
        Self {
            device,
            queue,
            buffer: Default::default(),
            data: Default::default(),
        }
    }

    pub fn registr_object(
        &mut self,
        obj: &RenderObject,
        transform: &Transform,
        group_manager: &mut BindGroupLayoutManager,
    ) {
        self.data
            .entry((obj.structure_type_id(), obj.texture_name().into()))
            .and_modify(|(x, _)| *x += 1)
            .or_insert_with(|| {
                (
                    1,
                    InstancedData::new(
                        obj.texture_name(),
                        &self.device,
                        &self.queue,
                        self.buffer
                            .entry(obj.structure_type_id())
                            .or_insert_with(|| {
                                (1, Arc::new(obj.create_buffer(&self.device, &self.queue)))
                            })
                            .1
                            .clone(),
                        group_manager,
                    ),
                )
            })
            .1
            .push(
                InstanceTransform::from(transform),
                &self.device,
                &self.queue,
            );
        self.buffer
            .entry(obj.structure_type_id())
            .and_modify(|(x, _)| *x += 1)
            .or_insert_with(|| (1, Arc::new(obj.create_buffer(&self.device, &self.queue))));
    }
    pub fn prepare(&mut self, _group_manager: &mut BindGroupLayoutManager) -> bool {
        self.data.retain(|_, (x, _)| x > &mut 0);
        self.data
            .iter_mut()
            .map(|((_structure, _texture_name), (count, data))| {
                data.prepare_capacity(*count, &self.device, &self.queue)
            })
            .any(|x| x)
    }
    pub fn iter_data(&self) -> impl Iterator<Item = (&'_ InstancedData, u32)> {
        self.data
            .iter()
            .map(|(_, (count, data))| (data, *count as u32))
    }
    pub fn clear(&mut self) {
        for (_, (i, _)) in &mut self.data {
            *i = 0;
        }
    }
}
