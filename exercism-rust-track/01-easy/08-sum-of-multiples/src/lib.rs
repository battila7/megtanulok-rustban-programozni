use std::collections::HashSet;

fn multiples_of_up_to_exclusive(n: u32, limit: u32) -> Vec<u32> {
    if n == 0 {
        return vec![];
    }

    let number_of_multiples = ((limit as f32) / (n as f32)).ceil() as u32;

    (1..number_of_multiples)
        .map(|i| i * n)
        .collect()
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut unique_multiples: HashSet<u32> = HashSet::new();

    factors
        .iter()
        .map(|n| multiples_of_up_to_exclusive(*n, limit))
        .flatten()
        .for_each(|n| {
            unique_multiples.insert(n);
        });

    unique_multiples
        .iter()
        .fold(0, |acc, n| acc + n)
}
