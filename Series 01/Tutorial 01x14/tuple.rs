fn main() {
    // construct mutable tuple
    let mut dog = (
        "eggdog", 3, "egg-dog mix"
    );
    println!(
        "{:?}\nis a mutable tuple",
        dog
    );

    // access tuple element
    println!("---");
    println!("name is {}", dog.0);

    // mutate tuple element
    dog.0 = "doggo";
    println!(
        "new name is {}",
        dog.0
    );

    // access tuple elements
    // using variables names
    let (name, age, breed) = dog;
    println!("---");
    println!(
        "name: {}\nage: {}\nbreed: {}",
        name, age, breed
    );

}
