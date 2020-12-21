pub fn is_armstrong_number(num: u32) -> bool {
    let num_as_string = num.to_string();
    let mut sum = 0;

    for ch in num_as_string.chars() {
        let n = ch.to_digit(10).expect("fail");

        sum += n.pow(num_as_string.len() as u32);
    }

    num == sum
}
