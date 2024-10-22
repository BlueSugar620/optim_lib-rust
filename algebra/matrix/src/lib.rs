use std::ops::{Add, AddAssign, Mul, MulAssign};

pub trait Op {
    type Value: Clone + Copy;
    fn zero() -> Self::Value;
    fn one() -> Self::Value;
    fn add(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
    fn mul(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
}

pub struct Matrix<O: Op> {
    values: Vec<Vec<O::Value>>,
}
impl<O: Op> Clone for Matrix<O> {
    fn clone(&self) -> Self {
        let values = self.values.clone();
        Self { values }
    }
}
impl<O: Op> Matrix<O> {
    pub fn new(values: &Vec<Vec<O::Value>>) -> Self
    where
        O::Value: Copy,
    {
        let _values = values;
        let m = _values.len();
        let n = _values.iter().map(|values| values.len()).max().unwrap_or(1);
        let mut values = vec![vec![O::zero(); n]; m];
        for (values, _values) in values.iter_mut().zip(_values.iter()) {
            for (value, _value) in values.iter_mut().zip(_values.iter()) {
                *value = *_value;
            }
        }
        Self { values }
    }

    pub fn id(n: usize) -> Self {
        let mut values = vec![vec![O::zero(); n]; n];
        for i in 0..n {
            values[i][i] = O::one();
        }
        Self { values }
    }

    pub fn zero(n: usize) -> Self {
        Self {
            values: vec![vec![O::zero(); n]; n],
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.values.len(), self.values[0].len())
    }

    fn square(&mut self) {
        let n = self.size().0;
        let mut values = vec![];
        for i in 0..n {
            let mut raw = vec![];
            for k in 0..n {
                raw.push(
                    (0..n)
                        .map(|j| O::mul(&self.values[i][j], &self.values[j][k]))
                        .fold(O::zero(), |acc, a| O::add(&acc, &a)),
                );
            }
            values.push(raw);
        }
        self.values = values;
    }

    pub fn pow(&self, mut exp: usize) -> Self {
        assert_eq!(self.size().0, self.size().1);
        let n = self.size().0;
        let mut res = Self::id(n);
        let mut base = self.clone();
        while exp > 0 {
            if exp & 1 == 1 {
                res *= base.clone();
            }
            base.square();
            exp >>= 1;
        }
        res
    }
}

impl<O: Op> AddAssign<Matrix<O>> for Matrix<O> {
    fn add_assign(&mut self, rhs: Matrix<O>) {
        assert_eq!(self.size(), rhs.size());
        for (lhs, rhs) in self.values.iter_mut().zip(rhs.values.iter()) {
            for (lhs, rhs) in lhs.iter_mut().zip(rhs.iter()) {
                *lhs = O::add(lhs, rhs);
            }
        }
    }
}
impl<O: Op> MulAssign<Matrix<O>> for Matrix<O> {
    fn mul_assign(&mut self, rhs: Matrix<O>) {
        assert_eq!(self.size().1, rhs.size().0);
        let (a, b) = self.size();
        let c = rhs.size().1;
        let mut values = vec![];
        for i in 0..a {
            let mut raw = vec![];
            for k in 0..c {
                raw.push(
                    (0..b)
                        .map(|j| O::mul(&self.values[i][j], &rhs.values[j][k]))
                        .fold(O::zero(), |acc, a| O::add(&acc, &a)),
                );
            }
            values.push(raw);
        }
        self.values = values;
    }
}
impl<O: Op> Add for Matrix<O> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size(), rhs.size());
        let values = self
            .values
            .iter()
            .zip(rhs.values)
            .map(|(lhs, rhs)| {
                lhs.iter()
                    .zip(rhs.iter())
                    .map(|(lhs, rhs)| O::add(lhs, rhs))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>();
        Self { values }
    }
}
impl<O: Op> Mul for Matrix<O> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size().1, rhs.size().0);
        let (a, b) = self.size();
        let c = rhs.size().1;
        let mut values = vec![];
        for i in 0..a {
            let mut raw = vec![];
            for k in 0..c {
                raw.push(
                    (0..b)
                        .map(|j| O::mul(&self.values[i][j], &rhs.values[j][k]))
                        .fold(O::zero(), |acc, a| O::add(&acc, &a)),
                );
            }
            values.push(raw);
        }
        Self { values }
    }
}

pub fn act<O: Op>(matrix: Matrix<O>, vector: Vec<O::Value>) -> Vec<O::Value> {
    assert_eq!(matrix.size().1, vector.len());
    matrix
        .values
        .iter()
        .map(|raw| {
            raw.iter()
                .zip(vector.iter())
                .map(|(lhs, rhs)| O::mul(lhs, rhs))
                .fold(O::zero(), |acc, a| O::add(&acc, &a))
        })
        .collect::<Vec<_>>()
}
