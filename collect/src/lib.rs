pub fn bubble_sort(arr: &mut [i32]) {
    let mut is_not_sorted = true;
    while is_not_sorted {
        is_not_sorted = false;
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                is_not_sorted = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v = [3, 2, 4, 5, 1, 7];
        let mut v_clone = v;

        bubble_sort(&mut v);
        println!("{:?}", v);

        v_clone.sort_unstable();
        println!("{:?}", v_clone);
    }
}
