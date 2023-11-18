struct Triangle {
    a: f32,
    b: f32,
}

// method
impl Triangle {
    fn hypotenuse(&self) -> f32 {
        f32::sqrt(
            self.a * self.a +
            self.b * self.b
        )
    }
}

fn main() {
    let shape = Triangle {
        a: 3.0,
        b: 4.0,
    };

    println!(
        "Side a:\t\t{}",
        shape.a
    );
    println!(
        "Side b:\t\t{}",
        shape.b
    );
    println!(
        "Hypotenuse:\t{}",
        shape.hypotenuse()
    );
    
}