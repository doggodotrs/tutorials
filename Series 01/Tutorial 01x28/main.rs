fn main() {
    println!("Hello, world!");

    println!("---");

    let x: i32 = 1;
    let y: i32 = 2;
    let z: i32 = x + y;

    println!("{} + {} = {}",
        x, y, z
    );

    println!("---");

    for i in 1..10 {
        println!("{}", i);
    }

    println!("---");

    println!("2 plus 3 is {}",
        add_int(2, 3)
    );
    
}

fn add_int(x: i32, y: i32) -> i32 {
    x + y
}
