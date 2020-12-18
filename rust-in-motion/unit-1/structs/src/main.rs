enum HockeyPosition {
    Wing,
}

struct HockeyPlayer {
    name: String,
    number: u8,
    position: HockeyPosition,
    goals_ytd: u8,
}

// Tuple struct
struct Triangle(u32, u32, u32);

fn is_equilateral(triangle: Triangle) -> bool {
    triangle.0 == triangle.1 && triangle.1 == triangle.2
}

// Newtype pattern
struct Meters(u8);

// Unit struct
struct UnitStruct;

// Enums with named fields
enum Clock {
    Sundial { hours: u8 },
    Digital { hours: u8, minutes: u8 },
    Analog { hours: u8, minutes: u8, seconds: u8 }
}

fn add_distances(d1: Meters, d2: Meters) -> Meters {
    Meters(d1.0 + d1.0)
}

fn main() {
    let mut player = HockeyPlayer {
        name: String::from("Bryan Rust"),
        number: 17,
        position: HockeyPosition::Wing,
        goals_ytd: 7,
    };

    player.goals_ytd += 1;

    println!(
        "{} has scored {} this season",
        player.name,
        player.goals_ytd,
    );

    let triangle1 = Triangle(3, 4, 5);
    println!("{}", is_equilateral(triangle1));

    let distance1 = Meters(3);
    let distance2 = Meters(7);

    let distance3 = add_distances(distance1, distance2);
    println!("{}", distance3.0);

    let s = UnitStruct;

    let clock = Clock::Analog {
        hours: 9,
        minutes: 25,
        seconds: 46,
    };
}
