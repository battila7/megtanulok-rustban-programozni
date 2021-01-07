const ALPHABET_OFFSET: u8 = 'a' as u8;
const ALPHABET_END_INDEX: u8 = 25;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain.chars()
        .filter_map(|ch| cipher_char(ch))
        .enumerate()
        .fold(String::new(), |mut result, (index, ch)| {
            if index % 5 == 0 && index > 0 {
                result.push(' ');
            }

            result.push(ch);

            result
        })
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars()
        .filter_map(|ch| cipher_char(ch))
        .collect()
}

fn cipher_char(ch: char) -> Option<char> {
    match ch {
        c if c.is_ascii_digit() => Some(c),
        c if c.is_ascii_alphabetic() => {
            let ch_index = (ch.to_ascii_lowercase() as u8) - ALPHABET_OFFSET;

            Some((ALPHABET_END_INDEX - ch_index + ALPHABET_OFFSET) as char)
        },
        _ => None
    }
}
