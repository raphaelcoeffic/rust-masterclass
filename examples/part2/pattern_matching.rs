enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

fn calculate_area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle { radius } => 3.14159 * radius * radius,
        Shape::Rectangle { width, height } => width * height,
        Shape::Triangle { base, height } => 0.5 * base * height,
        // Compiler ensures all cases are handled
    }
}

fn main() {
    let circle = Shape::Circle { radius: 1.0 };
    let rectangle = Shape::Rectangle {
        width: 1.0,
        height: 1.0,
    };
    let triangle = Shape::Triangle {
        base: 1.0,
        height: 1.0,
    };

    println!("Area circle: {}", calculate_area(&circle));
    println!("Area rectangle: {}", calculate_area(&rectangle));
    println!("Area triangle: {}", calculate_area(&triangle));
}
