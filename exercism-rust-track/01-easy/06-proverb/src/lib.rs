pub fn build_proverb(list: &[&str]) -> String {
    let mut result = String::new();

    for i in 0..list.len() {
        if i == (list.len() - 1) {
            result = format!("{}\nAnd all for the want of a {}.\n", result, list[0]);
        } else {
            result = format!("{}\nFor want of a {} the {} was lost.", result, list[i], list[i + 1]);
        }
    }

    String::from(result.trim())
}
