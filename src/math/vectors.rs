use super::matrices::{Matrix, Number};

pub trait Vector<T> {
    fn normalize(&self) -> Self;
    fn dot(&self, other: &Self) -> T;
}

pub trait Vector1D<T>: Vector<T> {
    fn x(&self) -> &T;
    fn x_mut(&mut self) -> &mut T;
}
pub trait Vector2D<T>: Vector<T> {
    fn x(&self) -> &T;
    fn y(&self) -> &T;

    fn x_mut(&mut self) -> &mut T;
    fn y_mut(&mut self) -> &mut T;
}
pub trait Vector3D<T>: Vector<T> {
    fn x(&self) -> &T;
    fn y(&self) -> &T;
    fn z(&self) -> &T;

    fn x_mut(&mut self) -> &mut T;
    fn y_mut(&mut self) -> &mut T;
    fn z_mut(&mut self) -> &mut T;

    fn cross(&self, other: &Self) -> Self;
}
pub trait Vector4D<T>: Vector<T> {
    fn x(&self) -> &T;
    fn y(&self) -> &T;
    fn z(&self) -> &T;
    fn w(&self) -> &T;

    fn x_mut(&mut self) -> &mut T;
    fn y_mut(&mut self) -> &mut T;
    fn z_mut(&mut self) -> &mut T;
    fn w_mut(&mut self) -> &mut T;
}

impl<T: Number, const N: usize> Vector<T> for Matrix<T, N, 1> {
    fn normalize(&self) -> Self {
        let sum: T = self
            .iter()
            .map(|(_, y)| *y * *y)
            .fold(T::default(), |x, y| x + y);
        match sum.is_zero() {
            true => self.clone(),
            false => self.clone() / sum,
        }
    }

    fn dot(&self, other: &Self) -> T {
        let mut ans = T::zero();

        for i in 0..N {
            ans += self[i] * other[i];
        }

        ans
    }
}

impl<T: Number> Vector1D<T> for Matrix<T, 1, 1> {
    fn x(&self) -> &T {
        &self[0]
    }

    fn x_mut(&mut self) -> &mut T {
        &mut self[0]
    }
}

impl<T: Number> Vector2D<T> for Matrix<T, 2, 1> {
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
        &mut self[0]
    }
}
impl<T: Number> Vector3D<T> for Matrix<T, 3, 1> {
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
        &mut self[0]
    }

    fn z(&self) -> &T {
        &self[2]
    }

    fn z_mut(&mut self) -> &mut T {
        &mut self[2]
    }

    fn cross(&self, other: &Self) -> Self {
        let mut ans = Self::default();
        *ans.x_mut() = *self.y() * *other.z() - *self.z() * *other.y();
        *ans.y_mut() = *self.z() * *other.x() - *self.x() * *other.z();
        *ans.z_mut() = *self.x() * *other.y() - *self.y() * *other.x();

        ans
    }
}

impl<T: Number> Vector4D<T> for Matrix<T, 4, 1> {
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
        &mut self[0]
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
