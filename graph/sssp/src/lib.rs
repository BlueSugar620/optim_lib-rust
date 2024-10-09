const INF: i64 = std::i64::MAX / 2;

pub struct SSSP {
    s: usize,
    dist: Vec<i64>,
    prev: Vec<Option<(usize, usize)>>,
}

impl SSSP {
    pub fn build(n: usize, s: usize, e: &[(usize, usize, i64)]) -> Self {
        if e.iter().all(|e| e.2 >= 0) {
            let (dist, prev) = dijkstra(n, s, e);
            return Self { s, dist, prev };
        }
        let (dist, prev) = moore_bellman_ford(n, s, e);
        Self { s, dist, prev }
    }

    pub fn dist_to(&self, t: usize) -> Option<i64> {
        if t != self.s && self.prev[t] == None {
            return None;
        } else {
            return Some(self.dist[t]);
        }
    }

    pub fn path_to(&self, t: usize) -> Option<(Vec<usize>, Vec<usize>)> {
        if t != self.s && self.prev[t] == None {
            return None;
        }
        let mut vertex = vec![t];
        let mut edge = vec![];
        let mut pos = t;
        while pos != self.s {
            let (prev_v, prev_e) = self.prev[pos].unwrap();
            vertex.push(prev_v);
            edge.push(prev_e);
            pos = prev_v;
        }
        vertex.reverse();
        edge.reverse();
        Some((vertex, edge))
    }
}

pub fn dijkstra(
    n: usize,
    s: usize,
    e: &[(usize, usize, i64)],
) -> (Vec<i64>, Vec<Option<(usize, usize)>>) {
    let _e = e;
    let mut e = vec![Vec::new(); n];
    for (i, &(u, v, c)) in _e.iter().enumerate() {
        e[u].push((v, c, i));
    }

    let mut dist = vec![INF; n];
    let mut prev = vec![None; n];
    let mut heap = std::collections::BinaryHeap::new();

    dist[s] = 0;
    heap.push((!0, s));

    while let Some((d, u)) = heap.pop() {
        let d = !d;
        if dist[u] < d {
            continue;
        }
        for &(v, c, k) in &e[u] {
            let d = d + c;
            if d < dist[v] {
                dist[v] = d;
                prev[v] = Some((u, k));
                heap.push((!d, v));
            }
        }
    }
    (dist, prev)
}

pub fn moore_bellman_ford(
    n: usize,
    s: usize,
    e: &[(usize, usize, i64)],
) -> (Vec<i64>, Vec<Option<(usize, usize)>>) {
    let mut dist = vec![INF; n];
    let mut prev = vec![None; n];
    dist[s] = 0;
    for i in 0..2 * n {
        for (k, &(u, v, c)) in e.iter().enumerate() {
            if dist[u] < INF && dist[u] + c < dist[v] {
                if i < n {
                    dist[v] = dist[u] + c;
                    prev[v] = Some((u, k));
                } else {
                    dist[v] = -INF;
                    prev[v] = Some((u, k));
                }
            }
        }
    }
    (dist, prev)
}
