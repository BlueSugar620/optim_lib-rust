use std::ops::{Index, RangeBounds};

pub trait Monoid {
    type Value;
    fn id() -> Self::Value;
    fn op(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
}

pub struct SegmentTree<M: Monoid> {
    values: Vec<M::Value>,
}
impl<M: Monoid> SegmentTree<M> {
    pub fn new(n: usize) -> Self
    where
        M::Value: Clone,
    {
        Self {
            values: vec![M::id(); 2 * n],
        }
    }

    pub fn update_at(&mut self, index: usize, value: M::Value) {
        let n = self.values.len() / 2;
        let mut i = index + n;
        self.values[i] = value;
        i /= 2;
        while i > 0 {
            self.values[i] = M::op(&self.values[2 * i], &self.values[2 * i + 1]);
            i /= 2;
        }
    }

    pub fn fold<R: RangeBounds<usize>>(&self, range: R) -> M::Value {
        let n = self.values.len() / 2;
        let (mut l, mut r) = unzip(range, n);
        l += n;
        r += n;
        let mut left = M::id();
        let mut right = M::id();
        while l < r {
            if l % 2 == 1 {
                left = M::op(&left, &self.values[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                right = M::op(&self.values[r], &right);
            }
            l /= 2;
            r /= 2;
        }
        M::op(&left, &right)
    }

    pub fn collect(&self) -> Vec<M::Value>
    where
        M::Value: Clone,
    {
        self.values[self.values.len() / 2..].to_vec()
    }
}

impl<M: Monoid> Index<usize> for SegmentTree<M> {
    type Output = M::Value;
    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index + self.values.len() / 2]
    }
}
fn unzip<R: RangeBounds<usize>>(range: R, n: usize) -> (usize, usize) {
    use std::ops::Bound;
    let start = match range.start_bound() {
        Bound::Unbounded => 0,
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => x + 1,
    };
    let end = match range.end_bound() {
        Bound::Unbounded => n,
        Bound::Included(&x) => x + 1,
        Bound::Excluded(&x) => x,
    };
    (start, end)
}
