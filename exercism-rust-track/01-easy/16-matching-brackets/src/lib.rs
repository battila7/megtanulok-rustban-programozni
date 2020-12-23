const OPENING: &str = "([{";
const CLOSING: &str = ")]}";

fn closing_pair(opening: char) -> char {
    match opening {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        _ => opening,
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = vec![];

    for ch in string.chars() {
        match ch {
            c if OPENING.contains(c) => {
                stack.push(closing_pair(c));
            },
            c if CLOSING.contains(c) => {
                if stack.pop() != Some(c) {
                    return false;
                }
            }
            _ => (),
        }
    }

    stack.is_empty()
}
