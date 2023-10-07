fn main() {
    // let x = 1;
    let mut x = 1;
    let y = 2;
    // let z = x + y;
    let mut z = x + y;

    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);

    x = 2;

    println!("---");
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);

    z = x + y;

    println!("---");
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);

    // how to increment

    let mut i = 1;
    i = i + 1;
    i += 1;

    println!("---");
    println!("i = {}", i);

    // how to perform basic math
    // using variables

    let height = 2;
    let width = 3;
    let area = height * width;

    println!("---");
    println!("area = {}", area);
}
