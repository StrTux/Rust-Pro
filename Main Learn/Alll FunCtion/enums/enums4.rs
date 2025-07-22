enum Shape {
    Rectangle(f64,f64),
    Circle(f64)
}
fn main() {
    let react = Shape::Rectangle(2.0, 3.0);
    let area1 = calculate_area(react);
    println!("Area of rectangle: {}", area1);

    let circle = Shape::Circle(5.0);
    let area2 = calculate_area(circle);
    println!("Area of circle: {}", area2);
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Rectangle(a,b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
    }
}
