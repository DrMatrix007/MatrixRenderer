use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub},
    vec::IntoIter,
};

pub trait Number:
    Default
    + Copy
    + Debug
    + Mul<Self, Output = Self>
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + Div<Self, Output = Self>
    + PartialEq
    + Neg<Output = Self>
    + PartialOrd
    + PartialEq
    + num_traits::identities::Zero
    + num_traits::identities::One
{
}

impl<
        T: Default
            + Copy
            + Debug
            + Mul<Self, Output = Self>
            + Add<Self, Output = Self>
            + AddAssign<Self>
            + Sub<Self, Output = Self>
            + Div<Self, Output = Self>
            + PartialEq
            + Neg<Output = Self>
            + num_traits::identities::Zero
            + num_traits::identities::One
            + PartialOrd
            + PartialEq,
    > Number for T
{
}

#[derive(Debug)]
pub struct Matrix<T: Number, const N: usize, const M: usize>(pub(self) Vec<T>);

impl<T: Number, const N: usize, const M: usize> Clone for Matrix<T, N, M> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Number, const N: usize, const M: usize> Neg for &Matrix<T, N, M> {
    type Output = Matrix<T, N, M>;

    fn neg(self) -> Self::Output {
        self.map(|_, x| -x)
    }
}

impl<T: Number, const N: usize, const M: usize> Neg for &mut Matrix<T, N, M> {
    type Output = Matrix<T, N, M>;

    fn neg(self) -> Self::Output {
        self.map(|_, x| -x)
    }
}
impl<T: Number, const N: usize, const M: usize> Neg for Matrix<T, N, M> {
    type Output = Matrix<T, N, M>;

    fn neg(self) -> Self::Output {
        -&self
    }
}

// impl<const N: usize, const M: usize, T> FromIterator<T, Matrix<N, 1>> for Matrix<T, N, M> {
//     fn from_iter<T: IntoIterator<Item = Matrix<T,N, 1>>>(iter: T) -> Self {
//         Matrix(iter.into_iter().flat_map(|x| x.0).collect())
//     }
// }

impl<const N: usize, const M: usize, T: Display + Number> Display for Matrix<T, N, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        for (pos, x) in self.iter() {
            if pos.0 == 0 {
                if pos.1!=0 {
                    write!(f," ")?;
                }
                write!(f, "[")?
            }
            write!(f, "{}", x)?;
            if pos.0 == N - 1 {
                if pos.1 == M - 1 {
                    write!(f, "]")?;
                } else {
                    writeln!(f, "],")?;
                }
            } else {
                write!(f, ",")?;
            }
        }

        write!(f, "]")
    }
}

impl<const N: usize, const M: usize, T: Number> Default for Matrix<T, N, M> {
    fn default() -> Self {
        Self::generate(Default::default)
    }
}

impl<const N: usize, const M: usize, T: Number> Matrix<T, N, M> {
    pub fn into_iter(&self) -> IntoIter<((usize, usize), &T)> {
        self.0
            .iter()
            .enumerate()
            .map(|(pos, x)| ((pos % N, pos / N), x))
            .collect::<Vec<_>>()
            .into_iter()
    }

    pub fn iter(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        self.0
            .iter()
            .enumerate()
            .map(|(pos, x)| ((pos % N, pos / N), x))
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = ((usize, usize), &mut T)> {
        self.0
            .iter_mut()
            .enumerate()
            .map(|(pos, x)| ((pos % N, pos / N), x))
    }

    pub fn trasnpose(&self) -> Matrix<T, M, N> {
        let mut ans = Matrix::default();
        for (pos, i) in ans.iter_mut() {
            *i = self[(pos.1, pos.0)];
        }
        ans
    }
    pub fn generate(f: impl FnMut() -> T) -> Self {
        Self(std::iter::repeat_with(f).take(N * M).collect())
    }

    pub fn element_wise_product(&self, x: &Matrix<T, N, M>) -> Matrix<T, N, M>
    where
        T: Default + Mul<T, Output = T> + Clone,
    {
        let mut ans = Matrix::default();

        for (pos, i) in ans.iter_mut() {
            *i = x[pos] * self[pos];
        }
        ans
    }

    pub fn map(&self, mut f: impl FnMut((usize, usize), T) -> T) -> Matrix<T, N, M>
    where
        T: Default + Clone,
    {
        let mut ans = self.clone();
        for (pos, val) in ans.iter_mut() {
            *val = f(pos, *val);
        }
        ans
    }
    pub fn max(&self) -> T
    where
        T: std::cmp::Ord,
    {
        self.iter()
            .map(|(_, y)| y)
            .fold(self[(0, 0)], |x, y| T::max(x, *y))
    }
    pub fn sub_matrices_vertically(&self) -> impl Iterator<Item = Matrix<T, N, 1>> + '_
    where
        T: Clone + Default,
    {
        (0..M).map(|offset| {
            let mut v = self
                .0
                .iter()
                .skip(offset * N)
                .take(N)
                .cloned()
                .collect::<Vec<_>>();
            v.resize(N, T::default());
            Matrix::<T, N, 1>(v)
        })
    }

    // pub fn sub(&self, i: usize) -> Matrix<T, N, 1>
    // where
    //     T: Clone,
    // {
    //     if i < M {
    //         Matrix(self.0.iter().skip(i * N).take(N).cloned().collect())
    //     } else {
    //         panic!("access bounds out of range");
    //     }
    // }
    // pub fn set_sub(&mut self, i: usize, m: &Matrix<T, N, 1>) {
    //     for x in 0..N {
    //         self[(x, i)] = m[(x, 0)];
    //     }
    // }
    pub fn add_matrix(&self, rhs: &Self) -> Self
    where
        T: Add<T, Output = T> + Default + Clone,
    {
        let mut ans = Matrix::default();
        for (a, b) in ans.iter_mut() {
            *b = self[a] + rhs[a];
        }
        ans
    }
    pub fn sub_matrix(&self, rhs: &Self) -> Self
    where
        T: Sub<T, Output = T> + Default + Clone,
    {
        let mut ans = Matrix::default();
        for (a, b) in ans.iter_mut() {
            *b = self[a] - rhs[a];
        }
        ans
    }
    pub fn mul_matrix<const K: usize>(&self, rhs: &Matrix<T, M, K>) -> Matrix<T, N, K>
    where
        T: Mul<T, Output = T> + AddAssign<T> + Default + Clone,
    {
        let mut ans = Matrix::default();

        for (pos, x) in ans.iter_mut() {
            *x = Default::default();
            for m in 0..M {
                *x += self[(pos.0, m)] * rhs[(m, pos.1)];
            }
        }
        ans
    }
    pub fn mul_element_wise(&self, m: &Matrix<T, N, M>) -> Self
    where
        T: Mul<T, Output = T> + Default + Clone,
    {
        self.map(|pos, x| x * m[pos])
    }
    pub fn div_element_wise(&self, m: &Matrix<T, N, M>) -> Self
    where
        T: Div<T, Output = T> + Default + Clone,
    {
        self.map(|pos, x| x / m[pos])
    }

    pub fn zeros() -> Self {
        Self::generate(|| T::zero())
    }
    pub fn identity() -> Self {
        let mut ans = Self::zeros();
        for i in 0..N.min(M) {
            ans[(i, i)] = T::one();
        }

        ans
    }
}

impl<const N: usize, const M: usize, T: Number> Index<(usize, usize)> for Matrix<T, N, M> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0 + index.1 * N]
    }
}

impl<const N: usize, const M: usize, T: Number> IndexMut<(usize, usize)> for Matrix<T, N, M> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index.0 + index.1 * N]
    }
}

impl<const N: usize, T: Number> Index<usize> for Matrix<T, N, 1> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize, T: Number> IndexMut<usize> for Matrix<T, N, 1> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Number, const N: usize, const M: usize> From<[[T; N]; M]> for Matrix<T, N, M> {
    fn from(val: [[T; N]; M]) -> Self {
        Matrix(val.concat().into_iter().collect())
    }
}

impl<T: Number, const N: usize, const M: usize> From<Matrix<T, N, M>> for [[T; N]; M] {
    fn from(val: Matrix<T, N, M>) -> Self {
        val.0
            .chunks(M)
            .map(|x| TryInto::<[T; N]>::try_into(x).unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }
}

impl<T: Number, const N: usize> From<[T; N]> for Matrix<T, N, 1> {
    fn from(val: [T; N]) -> Self {
        Self(val.into_iter().collect::<Vec<_>>())
    }
}

// impl<T: Into<f64> + Clone, const N: usize> ToMatrix<N, 1> for [T; N] {
//     fn to_matrix(self) -> Matrix<N, 1> {
//         Matrix(self.concat().into_iter().map(|x| x.into()).collect())
//     }
// }

mod add {
    use std::ops::Add;

    use super::{Matrix, Number};
    impl<const N: usize, const M: usize, T: Number> Add<&Matrix<T, N, M>> for &Matrix<T, N, M> {
        type Output = Matrix<T, N, M>;

        fn add(self, rhs: &Matrix<T, N, M>) -> Self::Output {
            let mut ans = Matrix::default();
            for (a, b) in ans.iter_mut() {
                *b = self[a] + rhs[a];
            }
            ans
        }
    }
    impl<const N: usize, const M: usize, T: Number> Add<Matrix<T, N, M>> for Matrix<T, N, M>
    where
        T: Add<T, Output = T> + Default + Clone,
    {
        type Output = Matrix<T, N, M>;

        fn add(self, rhs: Matrix<T, N, M>) -> Self::Output {
            self.add_matrix(&rhs)
        }
    }
    impl<const N: usize, const M: usize, T: Number> Add<&Matrix<T, N, M>> for Matrix<T, N, M>
    where
        T: Add<T, Output = T> + Default + Clone,
    {
        type Output = Matrix<T, N, M>;

        fn add(self, rhs: &Matrix<T, N, M>) -> Self::Output {
            (self).add_matrix(rhs)
        }
    }
    impl<const N: usize, const M: usize, T: Number> Add<Matrix<T, N, M>> for &Matrix<T, N, M>
    where
        T: Add<T, Output = T> + Default + Clone,
    {
        type Output = Matrix<T, N, M>;

        fn add(self, rhs: Matrix<T, N, M>) -> Self::Output {
            self.add_matrix(&rhs)
        }
    }
}
mod sub {
    use std::ops::Sub;

    use super::{Matrix, Number};

    impl<const N: usize, const M: usize, T: Number> Sub<&Matrix<T, N, M>> for &Matrix<T, N, M> {
        type Output = Matrix<T, N, M>;

        fn sub(self, rhs: &Matrix<T, N, M>) -> Self::Output {
            let mut ans = Matrix::default();
            for (a, b) in ans.iter_mut() {
                *b = self[a] - rhs[a];
            }
            ans
        }
    }
    impl<const N: usize, const M: usize, T: Number> Sub<Matrix<T, N, M>> for Matrix<T, N, M> {
        type Output = Matrix<T, N, M>;

        fn sub(self, rhs: Matrix<T, N, M>) -> Self::Output {
            self.sub_matrix(&rhs)
        }
    }
    impl<const N: usize, const M: usize, T: Number> Sub<&Matrix<T, N, M>> for Matrix<T, N, M> {
        type Output = Matrix<T, N, M>;

        fn sub(self, rhs: &Matrix<T, N, M>) -> Self::Output {
            self.sub_matrix(rhs)
        }
    }
    impl<const N: usize, const M: usize, T: Number> Sub<Matrix<T, N, M>> for &Matrix<T, N, M> {
        type Output = Matrix<T, N, M>;

        fn sub(self, rhs: Matrix<T, N, M>) -> Self::Output {
            self.sub_matrix(&rhs)
        }
    }
}
mod mul {
    use std::ops::{AddAssign, Mul};

    use super::{Matrix, Number};

    impl<const N: usize, const M: usize, const K: usize, T: Number> Mul<&Matrix<T, M, K>>
        for &Matrix<T, N, M>
    where
        T: Mul<T, Output = T> + AddAssign<T> + Default + Clone,
    {
        type Output = Matrix<T, N, K>;

        fn mul(self, rhs: &Matrix<T, M, K>) -> Self::Output {
            self.mul_matrix(rhs)
        }
    }
    impl<const N: usize, const M: usize, const K: usize, T: Number> Mul<Matrix<T, M, K>>
        for &Matrix<T, N, M>
    {
        type Output = Matrix<T, N, K>;

        fn mul(self, rhs: Matrix<T, M, K>) -> Self::Output {
            self.mul(&rhs)
        }
    }
    impl<const N: usize, const M: usize, const K: usize, T: Number> Mul<&Matrix<T, M, K>>
        for Matrix<T, N, M>
    {
        type Output = Matrix<T, N, K>;

        fn mul(self, rhs: &Matrix<T, M, K>) -> Self::Output {
            (&self).mul(rhs)
        }
    }
    impl<const N: usize, const M: usize, const K: usize, T: Number> Mul<Matrix<T, M, K>>
        for Matrix<T, N, M>
    {
        type Output = Matrix<T, N, K>;

        fn mul(self, rhs: Matrix<T, M, K>) -> Self::Output {
            self.mul(&rhs)
        }
    }

    impl<const N: usize, const M: usize, T: Number> Mul<T> for &Matrix<T, N, M> {
        type Output = Matrix<T, N, M>;

        fn mul(self, rhs: T) -> Self::Output {
            let mut ans = Matrix::default();
            for (pos, i) in ans.iter_mut() {
                *i = (self[pos]) * (rhs);
            }
            ans
        }
    }
    impl<const N: usize, const M: usize, T: Number> Mul<T> for Matrix<T, N, M> {
        type Output = Matrix<T, N, M>;

        fn mul(self, rhs: T) -> Self::Output {
            &self * rhs
        }
    }
    // impl<const N: usize, const M: usize, T:Mul<T,Output = T>> Mul<Matrix<T, N, M>> for T {
    //     type Output = Matrix<T, N, M>;

    //     fn mul(self, rhs: Matrix<T, N, M>) -> Self::Output {
    //         rhs * self
    //     }
    // }
    // impl<'a, const N: usize, const M: usize, T: Mul<Matrix<T, N, M>, Output = Matrix<T, N, M>>>
    //     Mul<&Matrix<T, N, M>> for T
    // {
    //     type Output = Matrix<T, N, M>;

    //     fn mul(self, rhs: &Matrix<T, N, M>) -> Self::Output {
    //         rhs * self
    //     }
    // }
}
mod div {
    use std::ops::Div;

    use super::{Matrix, Number};

    impl<const N: usize, const M: usize, T: Number> Div<T> for &Matrix<T, N, M> {
        type Output = Matrix<T, N, M>;

        fn div(self, rhs: T) -> Self::Output {
            let mut ans = Matrix::default();
            for (pos, i) in ans.iter_mut() {
                *i = self[pos] / rhs;
            }
            ans
        }
    }
    impl<const N: usize, const M: usize, T: Number> Div<T> for Matrix<T, N, M> {
        type Output = Matrix<T, N, M>;

        fn div(self, rhs: T) -> Self::Output {
            &self / rhs
        }
    }
}

pub type Matrix4<T> = Matrix<T, 4, 4>;
pub type Matrix3<T> = Matrix<T, 3, 3>;
pub type Vector4<T> = Matrix<T, 4, 1>;
pub type Vector3<T> = Matrix<T, 3, 1>;
