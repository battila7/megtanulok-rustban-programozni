pub struct Player {
    score: i32,
}

impl Player {
    pub fn set_score(&mut self, new_score: i32) {
        self.score = new_score
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn new() -> Self {
        Player { score: 0 }
    }
}

fn main() {
    // Actually, this works with my Rust version.
    let mut player1 = Player::new();
    
    player1.set_score(player1.score() + 1);

    println!("The score of Player1 is {}", player1.score());

    // Introduction of a temporary variable.
    let mut player2 = Player::new();
    
    let old_score = player2.score();
    player2.set_score(old_score + 1);

    println!("The score of Player2 is {}", player2.score());
}
