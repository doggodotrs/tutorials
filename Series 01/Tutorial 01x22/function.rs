fn main() {
    println!("2 plus 3 is {}",
        add_int(2, 3)
    );

    println!("2.1 plus 3.2 is {}",
        add_float(2.1, 3.2)
    );
    
}

fn add_int(x: i32, y: i32) -> i32 {
    x + y
}

fn add_float(x: f32, y: f32) -> f32 {
    x + y
}
