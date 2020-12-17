fn main() {
    // boolean
    let a = true;
    let b = false;

    if a {
        println!("a is true!");
    }

    if b {
        println!("b is true!");
    }

    // tuples
    let tup = (1, 'c', true);
    let first = tup.0;
    let second = tup.1;
    println!("the first is {}", first);
    println!("the second is {}", second);

    let (x, y, z) = tup;
    println!("x {}, y {}, z {}", x, y, z);

    // arrays
    let arr = [0.0, 3.14, -8.7928];
    let sec = arr[1];
    println!("the second element of arr is {}", sec);

    let mut arr2 = [7.2, 10.4, 345.01];
    arr2[0] = 0.0;
    println!("{:?}", arr2);

    // slices
    let arr3 = [100, 200, 300];
    let slice = &arr3[0..1];
    println!("{:?}", slice);
}
