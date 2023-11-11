fn main() {
    // loop over array
    let myarray = [10, 20, 30];

    for element in myarray {
        println!("{element}");
    }

    println!("---");

    // loop over string literal
    let mystr = "Hello, World!";

    for character in mystr.chars() {
        println!("{character}");
    }
    
}