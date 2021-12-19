use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Index, Mul, Sub};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct IntVector<const D: usize> {
    xs: [i64; D],
}

impl<const D: usize> Display for IntVector<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self.xs.iter().map(|n| n.to_string()).join(",");
        write!(f, "[{}]", s)
    }
}

impl<const D: usize> From<[i64; D]> for IntVector<D> {
    fn from(src: [i64; D]) -> Self {
        IntVector { xs: src }
    }
}

impl<const D: usize> From<&[i64]> for IntVector<D> {
    fn from(src: &[i64]) -> Self {
        let mut xs = [0; D];
        xs.copy_from_slice(src);
        IntVector { xs }
    }
}

impl<const D: usize> Index<usize> for IntVector<D> {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        Index::index(&self.xs, index)
    }
}

impl<const D: usize> Add for &IntVector<D> {
    type Output = IntVector<D>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = IntVector { xs: [0; D] };
        for i in 0..D {
            ret.xs[i] = self[i] + rhs[i];
        }
        ret
    }
}

impl<const D: usize> Sub for &IntVector<D> {
    type Output = IntVector<D>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut ret = IntVector { xs: [0; D] };
        for i in 0..D {
            ret.xs[i] = self.xs[i] - rhs.xs[i];
        }

        ret
    }
}

pub trait Vector<T>
where
    T: Default + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + PartialOrd<T> + Copy,
{
    const DIMS: usize;

    fn at(&self, d: usize) -> T;

    fn magnitude2(&self) -> T {
        let mut acc = T::default();
        for i in 0..Self::DIMS {
            let v = self.at(i);
            acc = acc + v * v;
        }
        acc
    }

    fn manh_dist(&self, other: &Self) -> T {
        let mut acc = T::default();
        for i in 0..Self::DIMS {
            let a = self.at(i);
            let b = other.at(i);
            if a > b {
                acc = acc + (a - b);
            } else {
                acc = acc + (b - a);
            }
        }
        acc
    }
}

impl<const D: usize> Vector<i64> for IntVector<D> {
    const DIMS: usize = D;

    fn at(&self, d: usize) -> i64 {
        self.xs[d]
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Matrix<const D: usize> {
    pub xs: [[i64; D]; D],
}

impl<const D: usize> From<&[&[i64]]> for Matrix<D> {
    fn from(src: &[&[i64]]) -> Self {
        let mut xss: [[i64; D]; D] = [[0; D]; D];
        for i in 0..D {
            xss[i].copy_from_slice(src[i])
        }

        Matrix { xs: xss }
    }
}

impl<const D: usize> Mul<&IntVector<D>> for &Matrix<D> {
    type Output = IntVector<D>;

    fn mul(self, rhs: &IntVector<D>) -> Self::Output {
        let mut ret = IntVector { xs: [0; D] };

        for i in 0..D {
            for j in 0..D {
                ret.xs[i] += self.xs[i][j] * rhs.xs[j];
            }
        }

        ret
    }
}

impl<const D: usize> Mul<Matrix<D>> for Matrix<D> {
    type Output = Matrix<D>;

    fn mul(self, rhs: Matrix<D>) -> Self::Output {
        let mut ret = Matrix { xs: [[0; D]; D] };
        for i in 0..D {
            for j in 0..D {
                for k in 0..D {
                    ret.xs[i][j] += self.xs[i][k] * rhs.xs[k][j];
                }
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::{IntVector, Matrix, Vector};

    #[test]
    fn it_works() {
        let a = IntVector::from([2, 1]);
        let b = IntVector::from([1, 2]);
        assert_eq!(2, a.manh_dist(&b));
    }

    #[test]
    fn rotate_3d() {
        let x = IntVector::from([1, 0, 0]);
        let r = Matrix {
            xs: [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
        };

        let result = &r * &x;
        assert_eq!(IntVector::from([0, 1, 0]), result);
    }
}
