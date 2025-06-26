enum Shape {
    Circle(f64),
    Cuboid(f64, f64, f64),
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Cuboid(length, width, height) => {
            // Area of a cuboid is not typically defined as a single value,
            // but we can return the surface area for demonstration.
            2.0 * (length * width + width * height + height * length)
        }
    }
}

fn main() {
    let circle = Shape::Circle(5.0);
    let cuboid = Shape::Cuboid(3.0, 4.0, 5.0);
    let area: f64 = calculate_area(cuboid);
    println!("The area of the cuboid is: {}", area);
    let area = calculate_area(circle);
    println!("The area of the circle is: {}", area);
}

