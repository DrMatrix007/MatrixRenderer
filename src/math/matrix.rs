use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Sub},
    vec::IntoIter,
};

#[derive(Debug)]
pub struct Matrix<T, const N: usize, const M: usize>(pub(self) Vec<T>);

impl<T: Clone, const N: usize, const M: usize> Clone for Matrix<T, N, M> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

// impl<const N: usize, const M: usize, T> FromIterator<T, Matrix<N, 1>> for Matrix<T, N, M> {
//     fn from_iter<T: IntoIterator<Item = Matrix<T,N, 1>>>(iter: T) -> Self {
//         Matrix(iter.into_iter().flat_map(|x| x.0).collect())
//     }
// }

impl<const N: usize, const M: usize, T: Display> Display for Matrix<T, N, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        for (pos, x) in self.iter() {
            if pos.0 == 0 {
                write!(f, "[{},", x)?
            } else if pos.0 == M - 1 {
                if pos.1 == N - 1 {
                    write!(f, "{}]", x)?;
                } else {
                    writeln!(f, "{}]", x)?;
                }
            } else {
                write!(f, "{},", x)?;
            }
        }

        write!(f, "]")
    }
}

impl<const N: usize, const M: usize, T: Default> Default for Matrix<T, N, M> {
    fn default() -> Self {
        Self::generate(Default::default)
    }
}

impl<const N: usize, const M: usize, T> Matrix<T, N, M> {
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
            .map(|(pos, x)| ((pos % M, pos / M), x))
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = ((usize, usize), &mut T)> {
        self.0
            .iter_mut()
            .enumerate()
            .map(|(pos, x)| ((pos % N, pos / N), x))
    }

    pub fn trasnpose(&self) -> Matrix<T, M, N>
    where
        T: Default + Clone,
    {
        let mut ans = Matrix::default();
        for (pos, i) in ans.iter_mut() {
            *i = self[(pos.1, pos.0)].clone();
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
            *i = x[pos].clone() * self[pos].clone();
        }
        ans
    }

    pub fn map(&self, mut f: impl FnMut((usize, usize), T) -> T) -> Matrix<T, N, M>
    where
        T: Default + Clone,
    {
        let mut ans = self.clone();
        for (pos, val) in ans.iter_mut() {
            *val = f(pos, val.clone());
        }
        ans
    }
    pub fn max(&self) -> T
    where
        T: std::cmp::Ord + Clone,
    {
        self.iter()
            .map(|(_, y)| y.clone())
            .fold(self[(0, 0)].clone(), T::max)
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
            *b = self[a].clone() + rhs[a].clone();
        }
        ans
    }
    pub fn sub_matrix(&self, rhs: &Self) -> Self
    where
        T: Sub<T, Output = T> + Default + Clone,
    {
        let mut ans = Matrix::default();
        for (a, b) in ans.iter_mut() {
            *b = self[a].clone() - rhs[a].clone();
        }
        ans
    }
    pub fn mul_matrix<const K: usize>(&self, rhs: &Matrix<T, K, N>) -> Matrix<T, K, M>
    where
        T: Mul<T, Output = T> + AddAssign<T> + Default + Clone,
    {
        let mut ans = Matrix::default();

        for (pos, x) in ans.iter_mut() {
            *x = Default::default();
            for n in 0..N {
                *x += self[(n, pos.1)].clone() * rhs[(pos.0, n)].clone();
            }
        }
        ans
    }
    pub fn mul_element_wise(&self, m: &Matrix<T, N, M>) -> Self
    where
        T: Mul<T, Output = T> + Default + Clone,
    {
        self.map(|pos, x| x * m[pos].clone())
    }
    pub fn div_element_wise(&self, m: &Matrix<T, N, M>) -> Self
    where
        T: Div<T, Output = T> + Default + Clone,
    {
        self.map(|pos, x| x / m[pos].clone())
    }
}

impl<const N: usize, const M: usize, T> Index<(usize, usize)> for Matrix<T, N, M> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0 + index.1 * N]
    }
}

impl<const N: usize, const M: usize, T> IndexMut<(usize, usize)> for Matrix<T, N, M> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index.0 + index.1 * N]
    }
}
impl<T, const N: usize, const M: usize> From<[[T; N]; M]> for Matrix<T, N, M>
where
    T: Clone,
{
    fn from(val: [[T; N]; M]) -> Self {
        Matrix(val.concat().into_iter().collect())
    }
}

// impl<T: Into<f64> + Clone, const N: usize> ToMatrix<N, 1> for [T; N] {
//     fn to_matrix(self) -> Matrix<N, 1> {
//         Matrix(self.concat().into_iter().map(|x| x.into()).collect())
//     }
// }

mod add {
    use std::ops::Add;

    use super::Matrix;
    impl<const N: usize, const M: usize, T: Add<T, Output = T> + Clone + Default>
        Add<&Matrix<T, N, M>> for &Matrix<T, N, M>
    {
        type Output = Matrix<T, N, M>;

        fn add(self, rhs: &Matrix<T, N, M>) -> Self::Output {
            let mut ans = Matrix::default();
            for (a, b) in ans.iter_mut() {
                *b = self[a].clone() + rhs[a].clone();
            }
            ans
        }
    }
    impl<const N: usize, const M: usize, T> Add<Matrix<T, N, M>> for Matrix<T, N, M>
    where
        T: Add<T, Output = T> + Default + Clone,
    {
        type Output = Matrix<T, N, M>;

        fn add(self, rhs: Matrix<T, N, M>) -> Self::Output {
            self.add_matrix(&rhs)
        }
    }
    impl<const N: usize, const M: usize, T: Add<T, Output = T>> Add<&Matrix<T, N, M>>
        for Matrix<T, N, M>
    where
        T: Add<T, Output = T> + Default + Clone,
    {
        type Output = Matrix<T, N, M>;

        fn add(self, rhs: &Matrix<T, N, M>) -> Self::Output {
            (self).add_matrix(rhs)
        }
    }
    impl<const N: usize, const M: usize, T> Add<Matrix<T, N, M>> for &Matrix<T, N, M>
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

    use super::Matrix;

    impl<const N: usize, const M: usize, T: Sub<T, Output = T> + Clone + Default>
        Sub<&Matrix<T, N, M>> for &Matrix<T, N, M>
    {
        type Output = Matrix<T, N, M>;

        fn sub(self, rhs: &Matrix<T, N, M>) -> Self::Output {
            let mut ans = Matrix::default();
            for (a, b) in ans.iter_mut() {
                *b = self[a].clone() - rhs[a].clone();
            }
            ans
        }
    }
    impl<const N: usize, const M: usize, T> Sub<Matrix<T, N, M>> for Matrix<T, N, M>
    where
        T: Sub<T, Output = T> + Default + Clone,
    {
        type Output = Matrix<T, N, M>;

        fn sub(self, rhs: Matrix<T, N, M>) -> Self::Output {
            self.sub_matrix(&rhs)
        }
    }
    impl<const N: usize, const M: usize, T> Sub<&Matrix<T, N, M>> for Matrix<T, N, M>
    where
        T: Sub<T, Output = T> + Default + Clone,
    {
        type Output = Matrix<T, N, M>;

        fn sub(self, rhs: &Matrix<T, N, M>) -> Self::Output {
            self.sub_matrix(rhs)
        }
    }
    impl<const N: usize, const M: usize, T> Sub<Matrix<T, N, M>> for &Matrix<T, N, M>
    where
        T: Sub<T, Output = T> + Default + Clone,
    {
        type Output = Matrix<T, N, M>;

        fn sub(self, rhs: Matrix<T, N, M>) -> Self::Output {
            self.sub_matrix(&rhs)
        }
    }
}
mod mul {
    use std::ops::{AddAssign, Mul};

    use super::Matrix;

    impl<
            const N: usize,
            const M: usize,
            const K: usize,
            T: Mul<T, Output = T> + AddAssign<T> + Default + Clone,
        > Mul<&Matrix<T, K, N>> for &Matrix<T, N, M>
        where
        T: Mul<T, Output = T> + AddAssign<T> + Default + Clone,

    {
        type Output = Matrix<T, K, M>;

        fn mul(self, rhs: &Matrix<T, K, N>) -> Self::Output {
            self.mul_matrix(rhs)
        }
    }
    impl<
            const N: usize,
            const M: usize,
            const K: usize,
            T: Mul<T, Output = T> + AddAssign<T> + Default + Clone,
        > Mul<Matrix<T, K, N>> for &Matrix<T, N, M>
    {
        type Output = Matrix<T, K, M>;

        fn mul(self, rhs: Matrix<T, K, N>) -> Self::Output {
            self.mul(&rhs)
        }
    }
    impl<
            const N: usize,
            const M: usize,
            const K: usize,
            T: Mul<T, Output = T> + AddAssign<T> + Default + Clone,
        > Mul<&Matrix<T, K, N>> for Matrix<T, N, M>
    {
        type Output = Matrix<T, K, M>;

        fn mul(self, rhs: &Matrix<T, K, N>) -> Self::Output {
            (&self).mul(rhs)
        }
    }
    impl<
            const N: usize,
            const M: usize,
            const K: usize,
            T: Mul<T, Output = T> + AddAssign<T> + Default + Clone,
        > Mul<Matrix<T, K, N>> for Matrix<T, N, M>
    {
        type Output = Matrix<T, K, M>;

        fn mul(self, rhs: Matrix<T, K, N>) -> Self::Output {
            self.mul(&rhs)
        }
    }

    impl<'a, const N: usize, const M: usize, T: Mul<T, Output = T> + Clone + Default + 'a> Mul<T>
        for &Matrix<T, N, M>
    {
        type Output = Matrix<T, N, M>;

        fn mul(self, rhs: T) -> Self::Output {
            let mut ans = Matrix::default();
            for (pos, i) in ans.iter_mut() {
                *i = (self[pos].clone()) * (rhs.clone());
            }
            ans
        }
    }
    impl<
            const N: usize,
            const M: usize,
            T: Mul<T, Output = T> + AddAssign<T> + Default + Clone,
        > Mul<T> for Matrix<T, N, M>
    {
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

    use super::Matrix;

    impl<const N: usize, const M: usize, T: Div<T, Output = T> + Default + Clone> Div<T>
        for &Matrix<T, N, M>
    {
        type Output = Matrix<T, N, M>;

        fn div(self, rhs: T) -> Self::Output {
            let mut ans = Matrix::default();
            for (pos, i) in ans.iter_mut() {
                *i = self[pos].clone() / rhs.clone();
            }
            ans
        }
    }
    impl<const N: usize, const M: usize, T: Div<T, Output = T> + Clone + Default> Div<T>
        for Matrix<T, N, M>
    {
        type Output = Matrix<T, N, M>;

        fn div(self, rhs: T) -> Self::Output {
            &self / rhs
        }
    }
}
