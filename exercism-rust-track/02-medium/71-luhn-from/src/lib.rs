const MIN_LENGTH: usize = 2;

pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let without_whitespace = self.code.replace(" ", "");

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
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> From<T> for Luhn
where
    T: ToString,
{
    fn from(input: T) -> Self {
        Self {
            code: input.to_string(),
        }
    }
}
