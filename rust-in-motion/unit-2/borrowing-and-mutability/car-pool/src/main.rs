#[derive(Debug)]
struct CarPool {
    passengers: Vec<String>,
}

impl CarPool {
    fn pick_up(&mut self, passenger: String) {
        self.passengers.push(passenger);
    }
}

fn main() {
    let mut monday_car_pool = CarPool {
        passengers: vec![],
    };

    monday_car_pool.pick_up(String::from("Attila"));
    println!("Car pool state: {:?}", monday_car_pool);

    monday_car_pool.pick_up(String::from("Lajos"));
    println!("Car pool state: {:?}", monday_car_pool);
}
