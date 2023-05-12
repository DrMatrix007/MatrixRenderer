use std::f32::consts::PI;

use bytemuck::{Pod, Zeroable};
use lazy_static::lazy_static;
use matrix_engine::components::resources::Resource;
use wgpu::{BindGroupEntry, BindGroupLayoutEntry, BufferUsages, Queue, ShaderStages};

use crate::{
    math::{
        matrices::{Matrix4, Vector3},
        transformable_matrices::{Prespective, TransformMatrix},
    },
    pipelines::{
        bind_groups::{BindDataEntry, BindGroupContainer},
        buffers::{BufferContainer, Bufferable},
        transform::Transform,
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
    pub fn generate_transform_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(&self.eye, &(&self.eye + &self.dir), &Vector3::up());

        let proj: Matrix4<f32> = &*OPENGL_TO_WGPU_MATRIX * Matrix4::from(&self.prespective) * view;

        proj
    }
}

pub struct CameraResource {
    group: BindGroupContainer<(CameraUniform,)>,
    camera_buffer: BufferContainer<CameraUniform>,
    transform: Transform,
    camera: Camera,
}

impl CameraResource {
    pub fn group(&self) -> &BindGroupContainer<(CameraUniform,)> {
        &self.group
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}

impl CameraResource {
    pub fn new(resource: &mut RendererResource) -> Self {
        let layout = resource
            .group_layout_manager_mut()
            .get_bind_group_layout::<(CameraUniform,)>();
        let camera_uniform = CameraUniform::default();
        let buffer = BufferContainer::<CameraUniform>::create_buffer(
            &camera_uniform,
            resource.device(),
            resource.queue(),
            BufferUsages::COPY_DST | BufferUsages::UNIFORM,
            false,
        );

        let group = layout.create_bind_group(resource.device(), (&buffer,));

        let camera = Camera::new(
            Vector3::from([[1.0, 0.0, 2.0]]),
            Vector3::from([[0.0, 0.0, -1.0]]),
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
            transform: Transform::identity(),
            camera,
        }
    }

    pub fn update_buffer(&mut self, queue: &Queue) {
        let data = self.camera.generate_transform_matrix().into_arrays();
        queue.write_buffer(self.camera_buffer.buffer(), 0, bytemuck::bytes_of(&data));
    }
}

impl Resource for CameraResource {}
