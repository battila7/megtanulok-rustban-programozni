struct Noisy {
    id: i32,
}

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("Noisy number {} going out of scope!", self.id);
    }
}

fn main() {
    let _noisy1 = Noisy { id: 1 };
    let _noisy2 = Noisy { id: 2 };

    println!("End of main.");
}
