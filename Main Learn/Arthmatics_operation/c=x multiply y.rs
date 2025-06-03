fn main() {
    let x = 5;
    let y = 10;
    let z = multiply_numbers(x, y); // x * y;
    println!("The product of x = {} and y = {} is z = {}", x, y, z);
}


fn multiply_numbers(a: i32, b: i32) -> i32 {
  let c = a * b;
  c
}