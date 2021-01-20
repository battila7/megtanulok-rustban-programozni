enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    ChangeColorEnum(Color),
}

enum Message2 {
    Hello { id: i32 },
}

struct Point {
    x: i32,
    y: i32,
}

struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // ---

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    // ---

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // ---
    
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // ---

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // ---

    let p = Point { x: 0, y: 7};
    let Point { x: a, y: b} = p;
    println!("{} {}", a, b);

    let Point { x, y } = p;
    println!("{} {}", x, y);

    // ---

    let p = Point { x: 0, y: 7};

    match p {
        Point { x, y: 0 } => println!("Point on the x axis at {}.", x),
        Point { x: 0, y } => println!("Point on the y axis at {}.", y),
        Point { x, y } => println!("Point on neither axis: ({}, {}).", x, y),
    }

    // ---

    let msg = Message::ChangeColorEnum(Color::Rgb(0, 160, 255));

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        },
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}.", x, y);
        },
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {} and blue {}.",
            r, g, b
        ),
        Message::ChangeColorEnum(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {} and blue {}.",
            r, g, b
        ),
        _ => (),
    }

    // ---

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: 10 });

    // ---

    foo(3, 4);

    // ---

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value.");
        },
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    // ---

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }

    // ---

    let origin = Point3 { x: 0, y: 0, z: 0 };

    match origin {
        Point3 { x, .. } => println!("x is {}", x),
    }

    // ---

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    // ---

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50!"),
        Some(n) if n == y =>    println!("Matched, n = {}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}.", x, y);

    // ---

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // --

    let msg = Message2::Hello { id: 5 };

    match msg {
        Message2::Hello {id: id_variable @ 3..=7 } => println!(
            "Found an id in range: {}.", id_variable
        ),
        Message2::Hello { id: 10..=12 } => println!(
            "Found an id in another range."
        ),
        Message2::Hello { id } => println!("Found some other id: {}.", id),
    }
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

