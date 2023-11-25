fn main() {
    // initial owner
    let s = String::from("hello");

    println!(
        "initial owner: {s}"
    );

    // move ownership

    takes_ownership(s);

    // value dropped from initial owner

    println!(
        "initial owner: {s}"
    );

}

fn takes_ownership(s: String) {
    println!(
        "new owner: {s}"
    );
}
