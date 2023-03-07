use std::ops::{AddAssign, Div, Mul, Sub};

use num_traits::{Float, Zero};

use super::matrices::Matrix;

pub trait Vector<T> {
    fn normalized(&self) -> Self
    where
        T: Mul<Output = T> + AddAssign<T> + Float + Div<Output = T>;
    fn dot(&self, other: &Self) -> T
    where
        T: Zero + Mul<Output = T> + AddAssign<T> + Clone;
}

pub trait Vector1D<T> {
    fn x(&self) -> &T;
    fn x_mut(&mut self) -> &mut T;
}
pub trait Vector2D<T> {
    fn x(&self) -> &T;
    fn y(&self) -> &T;

    fn x_mut(&mut self) -> &mut T;
    fn y_mut(&mut self) -> &mut T;
}
pub trait Vector3D<T> {
    fn x(&self) -> &T;
    fn y(&self) -> &T;
    fn z(&self) -> &T;

    fn x_mut(&mut self) -> &mut T;
    fn y_mut(&mut self) -> &mut T;
    fn z_mut(&mut self) -> &mut T;

    fn cross(&self, other: &Self) -> Self
    where
        T: Mul<Output = T> + Sub<Output = T> + Clone;
}
pub trait Crossable {
    fn cross(&self, other: &Self) -> Self;
}
pub trait Vector4D<T> {
    fn x(&self) -> &T;
    fn y(&self) -> &T;
    fn z(&self) -> &T;
    fn w(&self) -> &T;

    fn x_mut(&mut self) -> &mut T;
    fn y_mut(&mut self) -> &mut T;
    fn z_mut(&mut self) -> &mut T;
    fn w_mut(&mut self) -> &mut T;
}

impl<T, const N: usize> Vector<T> for Matrix<T, N, 1> {
    fn normalized(&self) -> Self
    where
        T: Mul<Output = T> + AddAssign<T> + Float + Div<Output = T>,
    {
        let sum: T = Float::sqrt(self.dot(self));
        match sum.is_zero() {
            true => self.clone(),
            false => self / sum,
        }
    }

    fn dot(&self, other: &Self) -> T
    where
        T: Zero + Mul<Output = T> + AddAssign<T> + Clone,
    {
        let mut ans = T::zero();

        for i in 0..N {
            ans += self[i].clone() * other[i].clone();
        }

        ans
    }
}

impl<T> Vector1D<T> for Matrix<T, 1, 1> {
    fn x(&self) -> &T {
        &self[0]
    }

    fn x_mut(&mut self) -> &mut T {
        &mut self[0]
    }
}

impl<T> Vector2D<T> for Matrix<T, 2, 1> {
    fn x(&self) -> &T {
        &self[0]
    }

    fn y(&self) -> &T {
        &self[1]
    }

    fn x_mut(&mut self) -> &mut T {
        &mut self[0]
    }

    fn y_mut(&mut self) -> &mut T {
        &mut self[1]
    }
}
impl<T> Vector3D<T> for Matrix<T, 3, 1> {
    fn x(&self) -> &T {
        &self[0]
    }

    fn y(&self) -> &T {
        &self[1]
    }

    fn x_mut(&mut self) -> &mut T {
        &mut self[0]
    }

    fn y_mut(&mut self) -> &mut T {
        &mut self[1]
    }

    fn z(&self) -> &T {
        &self[2]
    }

    fn z_mut(&mut self) -> &mut T {
        &mut self[2]
    }
    fn cross(&self, other: &Self) -> Self
    where
        T: Mul<Output = T> + Sub<Output = T> + Clone,
    {
        [
            self.y().clone() * other.z().clone() - self.z().clone() * other.y().clone(),
            self.z().clone() * other.x().clone() - self.x().clone() * other.z().clone(),
            self.x().clone() * other.y().clone() - self.y().clone() * other.x().clone(),
        ]
        .into()
    }
}

impl<T> Vector4D<T> for Matrix<T, 4, 1> {
    fn x(&self) -> &T {
        &self[0]
    }

    fn y(&self) -> &T {
        &self[1]
    }

    fn x_mut(&mut self) -> &mut T {
        &mut self[0]
    }

    fn y_mut(&mut self) -> &mut T {
        &mut self[1]
    }

    fn z(&self) -> &T {
        &self[2]
    }

    fn z_mut(&mut self) -> &mut T {
        &mut self[2]
    }

    fn w(&self) -> &T {
        &self[3]
    }

    fn w_mut(&mut self) -> &mut T {
        &mut self[3]
    }
}
