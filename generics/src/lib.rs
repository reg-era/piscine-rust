pub fn identity<T>(v: T) -> T {
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", identity("Hello, world!"));
        println!("{}", identity(3));
    }
}
