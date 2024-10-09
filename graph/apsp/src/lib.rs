const INF: i64 = std::i64::MAX / 2;

pub struct APSP {
    dist: Vec<Vec<i64>>,
    prev: Vec<Vec<usize>>,
    edge: Vec<Vec<Option<usize>>>,
}

impl APSP {
    pub fn build(n: usize, e: &[(usize, usize, i64)]) -> Self {
        let mut edge = vec![vec![None; n]; n];
        let mut memo = vec![vec![INF; n]; n];
        for (i, &(u, v, c)) in e.iter().enumerate() {
            if c < memo[u][v] {
                edge[u][v] = Some(i);
                memo[u][v] = c;
            }
        }
        let (dist, prev) = floyd_warshall(n, e);
        Self { dist, prev, edge }
    }

    pub fn dist_between(&self, u: usize, v: usize) -> Option<i64> {
        if self.dist[u][v] == INF {
            return None;
        }
        Some(self.dist[u][v])
    }

    pub fn path_between(&self, u: usize, v: usize) -> Option<(Vec<usize>, Vec<usize>)> {
        if self.dist[u][v] == INF {
            return None;
        }
        let mut vertex = vec![v];
        let mut path = vec![];
        let mut pos = v;
        while pos != u {
            let next = pos;
            pos = self.prev[u][pos];
            vertex.push(pos);
            path.push(self.edge[pos][next].unwrap());
        }
        vertex.reverse();
        path.reverse();
        Some((vertex, path))
    }
}

pub fn floyd_warshall(n: usize, e: &[(usize, usize, i64)]) -> (Vec<Vec<i64>>, Vec<Vec<usize>>) {
    let mut dist = vec![vec![INF; n]; n];
    for v in 0..n {
        dist[v][v] = 0;
    }
    for &(u, v, c) in e {
        dist[u][v] = dist[u][v].min(c);
    }
    let mut prev = (0..n).map(|i| vec![i; n]).collect::<Vec<_>>();
    for j in 0..n {
        for i in 0..n {
            for k in 0..n {
                if dist[i][j] < INF && dist[j][k] < INF && dist[i][j] + dist[j][k] < dist[i][k] {
                    dist[i][k] = dist[i][j] + dist[j][k];
                    prev[i][k] = prev[j][k];
                }
            }
        }
    }
    (dist, prev)
}
