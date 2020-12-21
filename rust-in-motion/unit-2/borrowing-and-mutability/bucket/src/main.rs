#[derive(Debug)]
struct Bucket {
    liters: u32,
}

fn pour(source: &mut Bucket, target: &mut Bucket, amount: u32) {
    source.liters -= amount;
    target.liters += amount;
}

fn main() {
    let mut bucket1 = Bucket { liters: 4 };
    let mut bucket2 = Bucket { liters: 6 };

    pour(&mut bucket1, &mut bucket2, 2);

    println!("Bucket1: {:?}", bucket1);
    println!("Bucket2: {:?}", bucket2);
}
