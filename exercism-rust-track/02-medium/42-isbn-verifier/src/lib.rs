pub fn is_valid_isbn(isbn: &str) -> bool {
    let without_hyphens = isbn.replace("-", "");
    if without_hyphens.len() != 10 {
        return false
    }

    let (code_as_str, checksum_as_str) = without_hyphens.split_at(without_hyphens.len() - 1);

    if !code_as_str.chars().all(|ch| ch.is_ascii_digit()) {
        return false
    }

    let check_character = checksum_as_str.chars().next().unwrap();
    if !check_character.is_ascii_digit() && check_character != 'X' {
        return false
    }

    let check = check_character.to_digit(10).unwrap_or(10) as usize;
    let sum: usize = code_as_str.chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .enumerate()
        .map(|pair| (10 - pair.0)  * pair.1)
        .sum();

    (sum + check) % 11 == 0
}
