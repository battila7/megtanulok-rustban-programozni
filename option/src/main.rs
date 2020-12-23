fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    // In the case of None, we must explicitly
    // specify the type.
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.expect("Missing number");
}
