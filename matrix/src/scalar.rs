use std::ops::{Add, Div, Mul, Sub};
pub trait Scalar: Add + Div + Mul + Sub + std::marker::Sized + Clone {
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

impl Scalar for u32 {
    type Item = u32;
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Scalar for u64 {
    type Item = u64;
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Scalar for i32 {
    type Item = i32;
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Scalar for i64 {
    type Item = i64;
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Scalar for f32 {
    type Item = f32;
    fn zero() -> Self {
        0.
    }
    fn one() -> Self {
        1.
    }
}

impl Scalar for f64 {
    type Item = f64;
    fn zero() -> Self {
        0.
    }
    fn one() -> Self {
        1.
    }
}
