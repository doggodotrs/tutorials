// declare struct
struct Dog {
    name: String,
    age: u32,
    breed: String,
}

fn main() {
    // create struct instance
    let mut dog = Dog {
        name: "eggdog".to_string(),
        age: 3,
        breed: String::from(
            "egg-dog mix"
        ),
    };

    // access struct fields
    println!(
        "name: {}\nage: {}\nbreed: {}",
        dog.name,
        dog.age,
        dog.breed
    );

    // mutate struct field value
    dog.name = String::from("Doggo");
    println!("\nnew name is {}",
        dog.name
    );
    
}
