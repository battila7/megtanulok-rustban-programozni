#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn count_coins(coins: &[Coin]) -> i32 {
    let mut count = 0;

    for coin in coins {
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}", state);
        } else {
            count += 1;
        }
    }

    count
}

fn main() {
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", six);

    if let Some(5) = five {
        println!("Five!");
    }

    count_coins(&[Coin::Penny, Coin::Dime, Coin::Quarter(UsState::Alaska)]);
}
