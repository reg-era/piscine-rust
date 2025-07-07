pub fn prev_prime(nbr: u64) -> u64 {
    if nbr <= 2 {
        return 0;
    }
    fn is_prime(nb: u64) -> bool {
        if nb == 2 {
            return true;
        }
        if nb <= 1 || nb % 2 == 0 {
            return false;
        }

        let mut i = 3;
        while i * i < nb {
            if nb % i == 0 {
                return false;
            }
            i += 2;
        }
        true
    }

    let mut nbr = nbr - 1;
    while !is_prime(nbr) {
        if nbr == 0 {
            break;
        }
        nbr -= 1;
    }

    nbr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("The previous prime number before 34 is: {}", prev_prime(34));
        assert_eq!(0, prev_prime(0));
        assert_eq!(0, prev_prime(2));
        assert_eq!(2, prev_prime(3));
        assert_eq!(3, prev_prime(5));
        assert_eq!(31, prev_prime(34));
        assert_eq!(631, prev_prime(633));
        assert_eq!(478139, prev_prime(478152));
    }
}
