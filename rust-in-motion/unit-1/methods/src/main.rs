enum HockeyPosition {
    Center,
    Wing,
}

struct HockeyPlayer {
    name: String,
    number: u8,
    position: HockeyPosition,
    goals_ytd: u8,
}

impl HockeyPlayer {
    fn new(name: String, number: u8, position: HockeyPosition) -> HockeyPlayer {
        HockeyPlayer {
            name: name,
            number: number,
            position: position,
            goals_ytd: 0
        }
    }

    fn shoot_puck(&mut self, seconds_remaining: u16) {
      if seconds_remaining < 300 {
          match self.position {
              HockeyPosition::Center => {
                  self.goals_ytd += 1;
                  println!("Goal!");
              },
              _ => println!("Miss!"),
          } 
      } else {
          self.goals_ytd += 1;
          println!("Goal!");
      }
    }
}

fn main() {
    let mut player = HockeyPlayer::new(
        String::from("Bryan Rust"),
        17,
        HockeyPosition::Wing,
    );

    player.shoot_puck(1000);
    player.shoot_puck(900);
}
