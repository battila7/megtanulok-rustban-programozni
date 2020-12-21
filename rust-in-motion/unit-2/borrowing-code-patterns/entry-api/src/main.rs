use std::collections::HashMap;

fn main() {
    let text = "hello world hello";

    let mut freqs = HashMap::new();

    // Works on my version of Rust.
    for word in text.split_whitespace() {
        match freqs.get_mut(word) {
            Some(value) => *value += 1,
            None => {
                freqs.insert(word, 1);
            }
        }
    }

    println!("Word frequencies: {:#?}", freqs);

    freqs.clear();

    // Using the Entry API.
    for word in text.split_whitespace() {
        *freqs.entry(word).or_insert(0) += 1;
    }

    println!("Word frequencies: {:#?}", freqs);
}
