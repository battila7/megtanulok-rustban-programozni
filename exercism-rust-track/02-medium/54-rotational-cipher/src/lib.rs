const LOWERCASE_OFFSET: u8 = 'a' as u8;
const UPPERCASE_OFFSET: u8 = 'A' as u8;
const ALPHABET_LENGTH: u8 = 26;

pub fn rotate(input: &str, key: i8) -> String {
    input.chars()
        .map(|ch| match ch {
            c if c.is_ascii_lowercase() => rotate_char(c, LOWERCASE_OFFSET, key),
            c if c.is_ascii_uppercase() => rotate_char(c, UPPERCASE_OFFSET, key),
            _ => ch,
        })
        .collect()
}

fn rotate_char(ch: char, offset: u8, key: i8) -> char {
    let ch_from_offset = (ch as u8) - offset;

    let rotated_char = modulo(ch_from_offset as i8 + key, ALPHABET_LENGTH);
    
    (rotated_char + offset) as char
}

fn modulo(x: i8, m: u8) -> u8 {
    let m_i8 = m as i8;

    (((x % m_i8) + m_i8) % m_i8) as u8
}
