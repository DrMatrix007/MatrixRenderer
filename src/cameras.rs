use bytemuck::{Pod, Zeroable};

use crate::math::{
    matrices::{Matrix, Matrix4, Vector3},
    transformable_matrices::{Prespective, TransformMatrix},
};

pub struct Camera {
    pub eye: Vector3<f32>,
    pub target: Vector3<f32>,
    pub up: Vector3<f32>,
    pub aspect: f32,
    pub fovy_rad: f32,
    pub znear: f32,
    pub zfar: f32,
}

pub static OPENGL_TO_WGPU_MATRIX: [[f32; 4]; 4] = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 0.5, 0.0],
    [0.0, 0.0, 0.5, 1.0],
];

impl Camera {
    pub fn build_projection_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(&self.eye, &self.target, &self.up);


        let proj: Matrix4<f32> = Prespective {
            fovy_rad: self.fovy_rad,
            aspect: self.aspect,
            far: self.zfar,
            near: self.znear,
        }
        .into();
        println!("view: \n{view}");
        Matrix::from(OPENGL_TO_WGPU_MATRIX) * proj * view
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
