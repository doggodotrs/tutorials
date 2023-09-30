// this is a single-line comment

// keyboard shortcut for comments is Ctrl+/

// this is a
// multi-line
// comment

fn main() {
    // println!("1 + 1 = {}", 2);
    println!("1 + 1 = {}", 1 + 1);
    // println!("1 + 1 = {}", 1+1);
    println!("2 - 1 = {}", 2 - 1);
    println!("2 * 3 = {}", 2 * 3);
    println!("4 / 2 = {}", 4 / 2);
    println!("5 / 2 = {}", 5 / 2);
    println!("5 % 2 = {}", 5 % 2);
    // exponents
    println!("2^3 = {}", i32::pow(2, 3));
    // order of operations
    println!("1 + (2 - 3)^4 * 5 = {}",
        1 + i32::pow(2 - 3, 4) * 5
    );
}
