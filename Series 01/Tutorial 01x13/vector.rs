fn main() {
    // construct mutable vector
    let mut vector = vec![1, 2, 3];
    println!("vector = {:?}", vector);

    // add element to
    // end of vector
    vector.push(100);
    println!("vector = {:?}", vector);

    // remove element from
    // end of vector
    vector.pop();
    println!("vector = {:?}", vector);

}
