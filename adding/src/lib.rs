pub fn add_curry(fs: i32) -> impl Fn(i32) -> i32 {
    move |sn| fs + sn
}
