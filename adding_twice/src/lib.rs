use std::ops::Add;

pub fn add_curry(fs: i32) -> impl Fn(i32) -> i32 {
    move |sn| fs + sn
}

pub fn twice<T: Add<Output = T> + Copy>(func: impl Fn(T) -> T) -> impl Fn(T) -> T {
    move |sn| func(func(sn))
}
