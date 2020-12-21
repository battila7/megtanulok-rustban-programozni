pub fn square_of_sum(n: u32) -> u32 {
    let value = (1..n + 1)
        .fold(0, |a, b| a + b);

    value * value
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..n + 1)
        .map(|i| i * i)
        .fold(0, |a, b| a + b)
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
