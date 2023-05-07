use std::{collections::HashMap, any::TypeId};

use wgpu::Device;

use crate::renderer::render_object::RenderObject;

use super::buffers::{Bufferable, BufferContainer};

trait BufferRef<B:Bufferable> {
    fn craete_buffer(&self,device:&Device,queue:&Queue) -> BufferContainer<B>;
}



pub struct Plain;


struct InstanceManager {
    count: HashMap<TypeId,usize>,
    
}

impl InstanceManager {
    fn registr_object(obj:&RenderObject) {

    }
}