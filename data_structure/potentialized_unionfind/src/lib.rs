pub trait Op {
    type Value;
    fn id() -> Self::Value;
    fn inv(elm: &Self::Value) -> Self::Value;
    fn op(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
}

pub struct PotentializedUnionfind<O: Op> {
    values: Vec<isize>,
    dist: Vec<O::Value>,
    cnt: usize,
}

impl<O: Op> PotentializedUnionfind<O>
where
    O::Value: Clone + Copy + Eq,
{
    pub fn new(n: usize) -> Self {
        Self {
            values: vec![-1; n],
            dist: vec![O::id(); n],
            cnt: n,
        }
    }

    fn root(&self, mut v: usize) -> (usize, O::Value) {
        let mut pot = self.dist[v];
        while self.values[v] >= 0 {
            v = self.values[v] as usize;
            pot = O::op(&pot, &self.dist[v]);
        }
        (v, pot)
    }

    pub fn unite(&mut self, from: usize, to: usize, d: O::Value) -> bool {
        let mut from = self.root(from);
        let mut to = self.root(to);
        if from.0 == to.0 {
            return O::op(&to.1, &O::inv(&from.1)) == d;
        }
        let mut d = O::op(&d, &O::inv(&O::op(&to.1, &O::inv(&from.1))));
        if self.values[from.0] > self.values[to.0] {
            std::mem::swap(&mut from, &mut to);
            d = O::inv(&d);
        }

        self.values[from.0] += self.values[to.0];
        self.values[to.0] = from.0 as isize;
        self.dist[to.0] = d;
        self.cnt -= 1;
        true
    }

    pub fn dist(&self, from: usize, to: usize) -> Option<O::Value> {
        let from = self.root(from);
        let to = self.root(to);
        if from.0 == to.0 {
            return Some(O::op(&to.1, &O::inv(&from.1)));
        }
        None
    }

    pub fn size(&self, u: usize) -> usize {
        -self.values[self.root(u).0] as usize
    }

    pub fn count(&self) -> usize {
        self.cnt
    }
}
