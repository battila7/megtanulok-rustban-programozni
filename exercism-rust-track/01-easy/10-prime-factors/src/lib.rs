pub fn factors(n: u64) -> Vec<u64> {
    let mut factor = 2;
    let mut i = n;
    let mut result = vec![];

    while i > 1 {
        match i % factor {
            0 => {
                result.push(factor);

                i /= factor;
            },
            _ => factor += 1,
        }
    }
    
    result
}
