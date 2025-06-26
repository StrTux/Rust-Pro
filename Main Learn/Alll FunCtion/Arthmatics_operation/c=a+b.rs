fn main() {
    let x = 42;
    let y = 52;  // integral value we can copy from the another variable in this case  we  can  change the 52 and write there x   then y  will  the value from the x variable
    let z = add_numbers(x, y); 

    println!("The sum of {} and {} is {}", x, y, z);
}

fn add_numbers(a: i32, b: i32) -> i32 {
    let c = a + b;
    c
}