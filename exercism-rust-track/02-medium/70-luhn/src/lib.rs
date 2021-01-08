const MIN_LENGTH: usize = 2;

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let without_whitespace = code.replace(" ", "");

    if without_whitespace.len() < MIN_LENGTH {
        return false
    }

    if without_whitespace.chars().any(|ch| !ch.is_digit(10)) {
        return false
    }

    let sum: usize = without_whitespace.chars()
        .map(|ch| ch.to_digit(10).map(|ch| ch as usize).unwrap())
        .rev()
        .enumerate()
        .map(|(index, n)| n * (2 - ((index + 1) % 2)))
        .map(|n| if n > 9 {
            n - 9
        } else {
            n
        })
        .sum();

    sum % 10 == 0
}
