fn main() {
    // exclude upper bound
    for i in 1..10 {
        println!("{i}");
    }

    println!("---");

    // include upper bound
    for i in 1..=10 {
        println!("{i}");
    }

    println!("---");

    // rev() method
    for i in (1..=10).rev() {
        println!("{i}");
    }

    println!("---");

    // step_by() method
    for i in (0..=50).step_by(5) {
        println!("{i}");
    }
    
}