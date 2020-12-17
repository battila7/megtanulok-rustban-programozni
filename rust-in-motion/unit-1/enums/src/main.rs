enum HockeyPosition {
    Center,
    Wing,
    Defense,
    Goalie
}

fn next_player(position: HockeyPosition) {
    // no-op
}

enum Clock {
    Sundial(u8),
    Digital(u8, u8),
    Analog(u8, u8, u8),
}

fn tell_time(clock: Clock) {
    match clock {
        Clock::Sundial(hours) => println!("It is about {} o'clock", hours),
        Clock::Digital(hours, minutes) => println!("It is {} minutes past {}", minutes, hours),
        Clock::Analog(hours, minutes, seconds) => {
            println!(
                "It is {} minutes and {} seconds past {} o'clock",
                minutes, seconds, hours
            );
        }
    }
}

fn main() {
    let position = HockeyPosition::Defense;
    next_player(position);

    tell_time(Clock::Analog(9, 25, 45));
}
