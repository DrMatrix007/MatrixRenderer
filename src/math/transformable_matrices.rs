use num_traits::{Float, cast};

use super::{
    matrices::{Vector3, Matrix4, Number},
    vectors::{Vector, Vector3D},
};

pub trait TransformMatrix {
    type Sub;

    fn look_to_rh(eye: &Self::Sub, dir: &Self::Sub, up: &Self::Sub) -> Self;
    fn look_to_lh(eye: &Self::Sub, dir: &Self::Sub, up: &Self::Sub) -> Self;

    fn look_at_rh(eye: &Self::Sub, center: &Self::Sub, up: &Self::Sub) -> Self;
    fn look_at_lh(eye: &Self::Sub, center: &Self::Sub, up: &Self::Sub) -> Self;
}

impl<T: Number> TransformMatrix for Matrix4<T> {
    type Sub = Vector3<T>;

    fn look_to_rh(eye: &Self::Sub, dir: &Self::Sub, up: &Self::Sub) -> Self {
        let f = dir.normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(&f);

        Matrix4::from([
            [*s.x(), *u.x(), -*f.x(), T::zero()],
            [*s.y(), *u.y(), -*f.y(), T::zero()],
            [*s.z(), *u.z(), -*f.z(), T::zero()],
            [-eye.dot(&s), -eye.dot(&u), eye.dot(&f), T::one()],
        ])
    }

    fn look_to_lh(eye: &Self::Sub, dir: &Self::Sub, up: &Self::Sub) -> Self {
        Self::look_to_rh(eye, &-dir, up)
    }

    fn look_at_rh(eye: &Self::Sub, center: &Self::Sub, up: &Self::Sub) -> Self {
        Self::look_to_rh(eye, &(center - eye), up)
    }

    fn look_at_lh(eye: &Self::Sub, center: &Self::Sub, up: &Self::Sub) -> Self {
        Self::look_to_lh(eye, &(center - eye), up)
    }
}
pub struct Prespective<T: Number> {
    pub fovy_rad: T,
    pub aspect: T,
    pub near: T,
    pub far: T,
}

impl<T: Number + Float> From<Prespective<T>> for Matrix4<T> {
    fn from(value: Prespective<T>) -> Self {
        assert!(value.near < value.far);
        assert!(!value.aspect.is_zero());
        let two: T = cast(2.0).unwrap();

        let f = T::one() / (value.fovy_rad / two).tan();

        Matrix4::from([
            [f / value.aspect, T::zero(), T::zero(), T::zero()],
            [T::zero(), f, T::zero(), T::zero()],
            [
                T::zero(),
                T::zero(),
                (value.far + value.near) / (value.near - value.far),
                -T::one(),
            ],
            [
                T::zero(),
                T::zero(),
                (two * value.far * value.near) / (value.near - value.far),
                T::zero(),
            ],
        ])
    }
}
