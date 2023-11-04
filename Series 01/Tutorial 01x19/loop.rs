fn main() {
    let mut i = 1;

    // infinite loop
    loop {
        // task
        println!("{i}");
        i += 1;
        // condition
        if i > 10 {
            break
        }
    }

}