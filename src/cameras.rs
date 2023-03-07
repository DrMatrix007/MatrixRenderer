use bytemuck::{Pod, Zeroable};

use crate::math::{
    matrices::{Matrix4, Vector3},
    transformable_matrices::{Prespective, TransformMatrix},
};

pub struct CameraPrespective {
    pub eye: Vector3<f32>,
    pub target: Vector3<f32>,
    pub up: Vector3<f32>,
    pub aspect: f32,
    pub fovy_rad: f32,
    pub znear: f32,
    pub zfar: f32,
}

pub enum Camera {
    Prespective(CameraPrespective),
}

pub static OPENGL_TO_WGPU_MATRIX: [[f32; 4]; 4] = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 0.5, 0.0],
    [0.0, 0.0, 0.5, 1.0],
];

impl Camera {
    pub fn build_projection_matrix(&self) -> Matrix4<f32> {
        match self {
            Camera::Prespective(cam) => {
                let view = Matrix4::look_to_rh(&cam.eye, &cam.target, &cam.up);
                println!("{}", view);
                let proj: Matrix4<f32> = Prespective {
                    fovy_rad: cam.fovy_rad,
                    aspect: cam.aspect,
                    far: cam.zfar,
                    near: cam.znear,
                }
                .into();
                Matrix4::from(OPENGL_TO_WGPU_MATRIX) * proj * view
            }
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct CameraUniform {
    pub proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn from_camera(&mut self, c: &Camera) {
        self.proj = c.build_projection_matrix().into();
    }
}

impl Default for CameraUniform {
    fn default() -> Self {
        Self {
            proj: Matrix4::identity().into(),
        }
    }
}
