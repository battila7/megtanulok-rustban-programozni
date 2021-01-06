use std::collections::HashMap;

fn main() {
    let numbers = vec![3, 5, 2, 2, 5];

    println!(
        "Mean: {}\nMedian: {:?}\nMode: {:?}",
        mean(&numbers),
        median(&numbers),
        mode(&numbers),
    );
}

fn mean(numbers: &[i32]) -> f32 {
    if numbers.is_empty() {
        0.0
    } else {
        let sum: i32 = numbers.iter().sum();

        (sum as f32) / (numbers.len() as f32)
    }
}

fn median(numbers: &[i32]) -> Option<i32> {
    if numbers.is_empty() {
        None
    } else {
        let mut copied_numbers = numbers.to_owned();

        copied_numbers.sort();

        copied_numbers.get(copied_numbers.len() / 2).map(|x| *x)
    }
}

fn mode(numbers: &[i32]) -> Option<i32> {
    if numbers.is_empty() {
        None
    } else {
        let mut counter = HashMap::new();

        for number in numbers {
            let count_of_number = counter.entry(number).or_insert(0);

            *count_of_number += 1;
        }

        counter.iter()
            .max_by(|x, y| x.0.cmp(y.0))
            .map(|entry| **entry.0)
    }
}
