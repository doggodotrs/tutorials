fn main() {
    // string slice
    let s = String::from(
        "Hello, World!"
    );

    let slice1 = &s[0..6];
    let slice2 = &s[7..13];

    println!(
        "slice1: {}\nslice2: {}",
        slice1, slice2
    );

    // array slice
    let a = [1, 2, 3, 4, 5];

    let slice3 = &a[0..2];
    let slice4 = &a[3..5];

    println!(
        "slice3: {:?}\nslice4: {:?}",
        slice3, slice4
    );
    
}