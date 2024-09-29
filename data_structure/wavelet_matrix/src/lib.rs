use static_bit_vec::*;
use std::ops::RangeBounds;

pub struct WaveletMatrix {
    values: Vec<(usize, StaticBitVec)>,
}

impl WaveletMatrix {
    pub fn new(bitlen: usize, mut seq: Vec<u64>) -> Self {
        let len = seq.len();
        let mut values = Vec::new();
        for l in (0..bitlen).rev() {
            let value = seq
                .iter()
                .map(|&x| (x >> l) & 1 == 1)
                .collect::<StaticBitVec>();
            let zeros = len - value.rank(len);
            values.push((zeros, value));
            seq = seq
                .iter()
                .filter(|&x| (x >> l) & 1 == 0)
                .copied()
                .chain(seq.iter().filter(|&x| (x >> l) & 1 == 1).copied())
                .collect();
        }
        values.reverse();
        Self { values }
    }

    pub fn access(&self, mut i: usize) -> u64 {
        let mut res = 0;
        for (zero, bitvec) in self.values.iter().rev() {
            let bit = bitvec.access(i);
            let rank = bitvec.rank(i);
            res = (res << 1) | bit as u64;
            if bit {
                i = zero + rank;
            } else {
                i -= rank;
            }
        }
        res
    }

    pub fn rank<R: RangeBounds<usize>>(&self, range: R, val: u64) -> usize {
        let (mut l, mut r) = unzip(range, self.values[0].1.len());
        for (i, (zero, bitvec)) in self.values.iter().enumerate().rev() {
            let l_cnt = bitvec.rank(l);
            let r_cnt = bitvec.rank(r);
            if (val >> i) & 1 == 1 {
                l = zero + l_cnt;
                r = zero + r_cnt;
            } else {
                l -= l_cnt;
                r -= r_cnt;
            }
        }
        r - l
    }

    pub fn quantile<R: RangeBounds<usize>>(&self, range: R, mut k: usize) -> u64 {
        let (mut l, mut r) = unzip(range, self.values[0].1.len());
        let mut res = 0;
        for (i, (zero, bitvec)) in self.values.iter().enumerate().rev() {
            let l_cnt = bitvec.rank(l);
            let r_cnt = bitvec.rank(r);
            let zeros = r + l_cnt - r_cnt - l;
            if zeros > k {
                l = l - l_cnt;
                r = r - r_cnt;
            } else {
                k -= zeros;
                res |= 1 << i;
                l = zero + l_cnt;
                r = zero + r_cnt;
            }
        }
        res
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
