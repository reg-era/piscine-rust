pub fn next_prime(nbr: u64) -> u64 {
    fn is_prime(nb: u64) -> bool {
        if nb <= 1 {
            return false;
        }
        if nb == 2 {
            return true;
        }
        if nb % 2 == 0 {
            return false;
        }

        let mut i = 3;
        while i * i <= nb {
            if nb % i == 0 {
                return false;
            }
            i += 2;
        }

        true
    }

    let mut nbr = nbr;
    while !is_prime(nbr) {
        nbr += 1;
    }

    nbr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("The next prime after 4 is: {}", next_prime(4));
        println!("The next prime after 11 is: {}", next_prime(11));
    }
}
