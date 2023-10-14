fn main() {
    // numbers

    let a = 42;
    println!(
        "{} is type i32, by default",
        a
    );

    let b: i64 = -3;
    println!("{} is type i64", b);

    let c: f64 = 1.2;
    println!("{} is type f64", c);

    println!("0.1 + 0.2 = {}",
        0.1 + 0.2
    );

    const PI: f32 = 3.14159;
    println!("PI = {}", PI);

    println!(
        "PI with 2 digits is {:.2}",
        PI
    );

    println!(
        "1000000 == 1_000_000 is {}",
        1000000 == 1_000_000
    );

    let d = f32::sqrt(4.0);
    println!(
        "The square root of 4 is {}",
        d
    );

    // from chapter on basic math
    println!("4 / 2 = {}", 4 / 2);
    println!("5 / 2 = {}", 5 / 2);

    println!(
        "5.0 / 2.0 = {}",
        5.0 / 2.0
    );

}
