const EMPTY_CHAR: char = '_';

pub fn encode(source: &str) -> String {
    let mut current_char = EMPTY_CHAR;
    let mut current_char_count: usize = 0;

    let mut result = String::from("");

    for ch in source.chars() {
        if ch != current_char {
            result = write_encoded(current_char, current_char_count, result);
            current_char = ch;
            current_char_count = 0;
        }

        current_char_count += 1;
    }

    write_encoded(current_char, current_char_count, result)
}

pub fn decode(source: &str) -> String {
    let mut result = String::from("");
    let mut count_as_str = String::new();

    for ch in source.chars() {
        if ch.is_numeric() {
            count_as_str.push(ch);
        } else {
            let count: usize = count_as_str.parse().unwrap_or(1);
            result.extend(ch.to_string().repeat(count).chars());

            count_as_str.clear();
        }
    }
    
    result
}

fn write_encoded(ch: char, count: usize, target: String) -> String {
    if ch == EMPTY_CHAR {
        return target;
    }

    match count {
        1 => format!("{}{}", target, ch),
        _ => format!("{}{}{}", target, count, ch)
    }
}
