fn main() {
    // characters and strings

    let a = 'a';
    println!("{} is type char", a);

    let b = "b";
    println!("{} is type &str", b);

    let c = "Hello, World!";
    println!(
        "{} is also type &str",
        c
    );

    // how to display quotation marks
    println!("doggo says \"bork!\".");

    // how to display new lines
    println!("1\n2\n3\n");

    // how to display tabs
    println!("1\t2\t3");

    // how to concatenate
    let s1 = "Hello, ";
    let s2 = "World!";

    let s1_s2 = s1.to_string() + s2;
    println!("concatenate example: {}",
        s1_s2
    );

}
