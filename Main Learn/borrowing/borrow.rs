// give  one example of  borrowing here

fn main() {
    let s1 = String::from("Hello, world!");
    
    // Borrowing s1
    let len = calculate_length(&s1); // using& to borrow s1
    // s1 is still valid here, we can use it again
    
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}