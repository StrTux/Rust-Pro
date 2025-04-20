fn main () {
    let num1: u8 = 5;
    let num2: u8 = 80;
    let sum = add(num1, num2); //  add si the function  call  here  
    println!("The sum is: {}", sum);
}

fn add(num1: u8, num2:u8) -> u8 {
    num1 + num2
}