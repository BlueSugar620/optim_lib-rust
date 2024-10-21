use std::iter::FromIterator;

const UNIT: usize = std::mem::size_of::<usize>();

pub struct StaticBitVec {
    len: usize,
    bytes: Vec<u8>,
    sum: Vec<usize>,
}

impl StaticBitVec {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get_at(&self, i: usize) -> bool {
        let (q, r) = (i / UNIT, i % UNIT);
        self.bytes[q] >> r & 1 == 1
    }

    pub fn rank(&self, e: usize) -> usize {
        let (q, r) = (e / UNIT, e % UNIT);
        self.sum[q] + (self.bytes[q] & ((1 << r) - 1)).count_ones() as usize
    }
}

impl FromIterator<bool> for StaticBitVec {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let mut bytes = Vec::new();
        let mut sum = Vec::new();
        let mut cnt = 0;
        let mut len = 0;
        'OUTER: loop {
            sum.push(cnt);
            let mut byte = 0;
            for i in 0..UNIT {
                match iter.next() {
                    None => {
                        bytes.push(byte);
                        sum.push(cnt);
                        break 'OUTER;
                    }
                    Some(false) => (),
                    Some(true) => {
                        byte |= 1 << i;
                        cnt += 1;
                    }
                }
                len += 1;
            }
            bytes.push(byte);
        }
        Self { len, bytes, sum }
    }
}
