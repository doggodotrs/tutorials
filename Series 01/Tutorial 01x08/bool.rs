fn main() {
    // comparison operators
    println!("3 < 2 is {}", 3 < 2);
    println!("3 >= 3 is {}", 3 >= 3);
    println!("4 == 3 is {}", 4 == 3);
    // && Lazy Boolean AND operator
    println!("1 == 1 && 1 > 1 is {}",
        1 == 1 && 1 > 1
    );
    // || Lazy Boolean OR operator
    println!("1 == 1 || 1 > 1 is {}",
        1 == 1 || 1 > 1
    );
}
