use std::io;

fn discount(day_of_month: u8) {
    let amount = if day_of_month % 2 == 0 {
        50
    } else {
        10
    };

    println!("Your discount is {}!", amount);
}

fn secret() {
    loop {
        println!("What is the secret word?");
        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read line");

        if word.trim() == "rust" {
            break
        }
    }
}

fn secret_with_while() {
    let mut word = String::new();
    while word.trim() != "rust" {
        println!("What is the secret word?");
        word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read line");
    }
}

fn for_range() {
    for i in 1..11 {
        println!("Now serving number {}", i);
    }
}

fn number_match() {
    let x = 3;

    match x {
        1 => println!("One is the loneliest number."),
        2 => println!("Two's a company"),
        3 => println!("Three's a crowd"),
        _ => println!("Some other number"),
    }
}

fn dice_roll() {
    let die1 = 1;
    let die2 = 5;

    match (die1, die2) {
        (1, 1) => println!("Snake Eyes! Go back to the beginning!"),
        (5, _) | (_, 5) => {
            println!("You rolled at least one 5!");
            println!("Move and then roll again!");
        }
        _ => println!("Move your piece!"),
    }
}

fn account_check() {
    let is_confirmed = true;
    let is_active = false;

    match (is_confirmed, is_active) {
        (true, true) => println!("This account is in good standing!"),
        (false, true) => println!("You need to confirm your account!"),
        (false, false) => println!("This account will be deactivated!"),
        _ => {},
    }
}

fn main() {
    account_check();
}
