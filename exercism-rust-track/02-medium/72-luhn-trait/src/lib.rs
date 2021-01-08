const MIN_LENGTH: usize = 2;

/// Check a Luhn checksum.
fn is_valid_luhn(code: &str) -> bool {
    let without_whitespace = code.replace(" ", "");

    if without_whitespace.len() < MIN_LENGTH {
        return false;
    }

    if without_whitespace.chars().any(|ch| !ch.is_digit(10)) {
        return false;
    }

    let sum: usize = without_whitespace
        .chars()
        .map(|ch| ch.to_digit(10).map(|ch| ch as usize).unwrap())
        .rev()
        .enumerate()
        .map(|(index, n)| n * (2 - ((index + 1) % 2)))
        .map(|n| if n > 9 { n - 9 } else { n })
        .sum();

    sum % 10 == 0
}

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        is_valid_luhn(&self.to_string())
    }
}
