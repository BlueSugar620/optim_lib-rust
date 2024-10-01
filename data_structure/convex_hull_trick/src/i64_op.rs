use super::Op;

enum O {}
impl Op for O {
    type Value = i64;
    fn inf() -> Self::Value {
        std::i64::MAX << 1
    }
    fn add(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value {
        lhs + rhs
    }
    fn inv(elm: &Self::Value) -> Self::Value {
        -elm
    }
    fn mul(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value {
        lhs * rhs
    }
    fn intersect(
        lhs: (&Self::Value, &Self::Value),
        rhs: (&Self::Value, &Self::Value),
    ) -> Self::Value {
        if lhs.0 < rhs.0 {
            (lhs.1 - rhs.1).div_euclid(rhs.0 - lhs.0)
        } else if lhs.0 > rhs.0 {
            (rhs.1 - lhs.1).div_euclid(lhs.0 - rhs.0)
        } else {
            panic!()
        }
    }
}
