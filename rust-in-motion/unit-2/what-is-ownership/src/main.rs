fn main() {
    let s = String::from("book");

    let pl = pluralize(s.clone());

    println!(
        "I have one {}, you have two {}.",
        s,
        pl
    );
}

fn pluralize(s: String) -> String {
    return s + "s"
}
