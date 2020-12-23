fn is_yelled(message: &str) -> bool {
    let has_at_least_one_alphabetic = message.contains(char::is_alphabetic);

    return has_at_least_one_alphabetic && message.to_uppercase() == message
}

fn is_question(message: &str) -> bool {
    message.ends_with('?')
}

pub fn reply(message: &str) -> &str {
    let trimmed_message = message.trim();

    if trimmed_message.is_empty() {
        return "Fine. Be that way!";
    }

    match (is_yelled(trimmed_message), is_question(trimmed_message)) {
        (false, true) => "Sure.",
        (true, false) => "Whoa, chill out!",
        (true, true) => "Calm down, I know what I'm doing!",
        _ => "Whatever."
    }
}
