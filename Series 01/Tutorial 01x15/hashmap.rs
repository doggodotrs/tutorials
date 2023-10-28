use std::collections::HashMap;

fn main() {
    // initialize empty HashMap
    let mut dog = HashMap::new();
    // add key-value pairs
    dog.insert("name", "eggdog");
    dog.insert("age", "3");
    dog.insert("breed", "egg-dog mix");
    // display HashMap
    println!("dog = {:#?}", dog);

    // access value using key
    println!("\nbreed = {:#?}",
        dog.get("breed")
    );

    // mutate value using key
    dog.insert("name", "Doggo");
    println!("\ndog = {:#?}", dog);

    // insert new key-value pair
    dog.insert(
        "email", "doggo@doggo.doggo"
    );
    println!("\ndog = {:#?}", dog);

    // remove key-value pair
    dog.remove("breed");
    println!("\ndog = {:#?}", dog);

}
