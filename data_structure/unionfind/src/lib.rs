pub struct Unionfind {
    values: Vec<isize>,
    cnt: usize,
}

impl Unionfind {
    pub fn new(n: usize) -> Self {
        Self {
            values: vec![-1; n],
            cnt: n,
        }
    }

    fn root(&self, mut v: usize) -> usize {
        while self.values[v] >= 0 {
            v = self.values[v] as usize;
        }
        v
    }

    pub fn unite(&mut self, u: usize, v: usize) -> bool {
        let mut u = self.root(u);
        let mut v = self.root(v);
        if u == v {
            return false;
        }
        if self.values[u] > self.values[v] {
            std::mem::swap(&mut u, &mut v);
        }
        self.values[u] += self.values[v];
        self.values[v] = u as isize;
        self.cnt -= 1;
        true
    }

    pub fn is_same(&self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }

    pub fn size(&self, u: usize) -> usize {
        -self.values[self.root(u)] as usize
    }

    pub fn count(&self) -> usize {
        self.cnt
    }
}
