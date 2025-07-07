pub trait Scalar<Item> {
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}
impl Scalar<u32> for u32 {
    type Item = u32;
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}
impl Scalar<u64> for u64 {
    type Item = u64;
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}
impl Scalar<i32> for i32 {
    type Item = i32;
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}
impl Scalar<i64> for i64 {
    type Item = i64;
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}
impl Scalar<f32> for f32 {
    type Item = f32;
    fn zero() -> Self {
        0.
    }
    fn one() -> Self {
        1.
    }
}
impl Scalar<f64> for f64 {
    type Item = f64;
    fn zero() -> Self {
        0.
    }
    fn one() -> Self {
        1.
    }
}
// u32 u64 i32 i64 f32 f64.
