fn main() {
    // initial owner
    let s = String::from("hello");

    println!(
        "initial owner: {s}"
    );

    // references and borrowing

    references(&s);

    // initial owner still owns value

    println!(
        "initial owner: {s}"
    );

}

fn references(s: &String) {
    println!(
        "borrower: {s}"
    );
}
