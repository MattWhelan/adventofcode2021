use std::ops::{Add, Index, Mul, Sub};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct IntVector<const D: usize> {
    xs: [i64; D]
}

impl <const D: usize> From<[i64; D]> for IntVector<D> {
    fn from(src: [i64; D]) -> Self {
        IntVector {
            xs: src
        }
    }
}

impl<const D: usize> Index<usize> for IntVector<D> {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        Index::index(&self.xs, index)
    }
}

impl<const D: usize> Add for IntVector<D> {
    type Output = IntVector<D>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = self.clone();
        for i in 0..D {
            ret.xs[i] = ret.xs[i] + rhs[i].clone();
        }
        ret
    }
}

pub trait Vector<T>
    where T: Default + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + PartialOrd<T> + Copy {
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

impl <const D: usize> Vector<i64> for IntVector<D> {
    const DIMS: usize = D;

    fn at(&self, d: usize) -> i64 {
        self.xs[d]
    }
}

#[cfg(test)]
mod tests {
    use crate::{IntVector, Vector};

    #[test]
    fn it_works() {
        let a = IntVector::from([2, 1]);
        let b = IntVector::from([1, 2]);
        assert_eq!(2, a.manh_dist(&b));
    }
}
