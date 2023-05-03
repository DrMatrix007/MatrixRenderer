use std::f32::consts::PI;

use bytemuck::{Pod, Zeroable};
use lazy_static::lazy_static;
use matrix_engine::components::resources::Resource;
use wgpu::{
    BindGroupEntry, BindGroupLayoutEntry, BufferUsages, Queue, ShaderStages,
};

use crate::{
    math::{
        matrices::{Matrix4, Vector3},
        transformable_matrices::{Prespective, TransformMatrix},
    },
    pipelines::{
        bind_groups::{BindDataEntry, BindGroupContainer},
        buffers::{BufferContainer, Bufferable},
    },
};

use super::renderer_system::RendererResource;

#[repr(C)]
#[derive(Pod, Zeroable, Debug, Clone, Copy)]
pub struct CameraUniform {
    pub data: [[f32; 4]; 4],
}

impl Default for CameraUniform {
    fn default() -> Self {
        Self {
            data: Matrix4::identity().into(),
        }
    }
}

impl CameraUniform {
    fn read_from_matrix(&mut self, m: &Matrix4<f32>) {
        self.data = m.into();
    }
}
impl Bufferable for CameraUniform {
    fn describe<'a>() -> wgpu::VertexBufferLayout<'a> {
        todo!()
    }
}
impl BindDataEntry for CameraUniform {
    type Args<'a> = &'a BufferContainer<CameraUniform>;

    fn layout_entries() -> Box<dyn Iterator<Item = BindGroupLayoutEntry>> {
        Box::new(std::iter::once(BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }))
    }

    fn entries<'a>(args: Self::Args<'a>) -> Box<dyn Iterator<Item = BindGroupEntry<'a>> + 'a> {
        Box::new(std::iter::once(BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: args.buffer(),
                offset: 0,
                size: None,
            }),
        }))
    }
}

lazy_static! {
    static ref OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::from([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 0.5, 0.0],
        [0.0, 0.0, 0.5, 1.0],
    ]);
}

pub struct Camera {
    pub prespective: Prespective<f32>,
    pub eye: Vector3<f32>,
    pub dir: Vector3<f32>,
}

impl Camera {
    pub fn new(eye: Vector3<f32>, dir: Vector3<f32>, prespective: Prespective<f32>) -> Self {
        Self {
            eye,
            prespective,
            dir,
        }
    }
    pub fn update_uniform(&self, uniform: &mut CameraUniform) {
        let view = Matrix4::look_at_rh(&self.eye, &(&self.eye + &self.dir), &Vector3::up());

        let proj: Matrix4<f32> = &*OPENGL_TO_WGPU_MATRIX * Matrix4::from(&self.prespective) * view;

        uniform.data = (proj).into();
    }
}

pub struct CameraResource {
    group: BindGroupContainer<(CameraUniform,)>,
    camera_buffer: BufferContainer<CameraUniform>,
    camera_uniform: CameraUniform,
    camera: Camera,
}

impl CameraResource {
    pub fn group(&self) -> &BindGroupContainer<(CameraUniform,)> {
        &self.group
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_uniform(&self) -> &CameraUniform {
        &self.camera_uniform
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
}

impl CameraResource {
    pub fn new(device: &mut RendererResource) -> Self {
        let layout = device.get_bind_group_layout::<(CameraUniform,)>();
        let camera_uniform = CameraUniform::default();
        let buffer = BufferContainer::<CameraUniform>::create_buffer(
            &camera_uniform,
            device.device(),
            BufferUsages::COPY_DST | BufferUsages::UNIFORM,
        );

        let group = layout.create_bind_group(device.device(), (&buffer,));

        let camera = Camera::new(
            Vector3::from([1.0, 0.0, 2.0]),
            Vector3::from([0.0, 0.0, -1.0]),
            Prespective {
                fovy_rad: PI / 4.0,
                aspect: 1.0,
                near: 0.1,
                far: 1000.0,
            },
        );

        Self {
            group,
            camera_buffer: buffer,
            camera_uniform,
            camera,
        }
    }

    pub fn update_buffer(&mut self, queue: &Queue) {
        self.camera.update_uniform(&mut self.camera_uniform);
        queue.write_buffer(
            self.camera_buffer.buffer(),
            0,
            bytemuck::bytes_of(&self.camera_uniform),
        );
    }
}

impl Resource for CameraResource {}
