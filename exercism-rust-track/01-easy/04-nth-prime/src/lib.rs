fn is_prime(n: u32) -> bool {
    if n == 2 {
        true
    } else {
        for i in 2..(n / 2 + 1) {
            if (n % i) == 0 {
                return false
            }
        }

        true
    }
}

pub fn nth(n: u32) -> u32 {
    let mut i = 2;
    let mut primes_found = 0;

    loop {
        if is_prime(i) {
            primes_found += 1;
        }

        // n is zero based, thus:
        // n = 0 -> 2
        // n = 1 -> 3
        // n = 2 -> 5
        if (primes_found - 1) == n {
            return i
        }

        i += 1;
    }
}
