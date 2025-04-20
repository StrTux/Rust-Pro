fn main() {
    // Immutable string slice (&str)
    let s1: &str = "This is an &str"; // Cannot be changed

    // Mutable String
    let mut s2: String = String::from("This is a String"); // Can be changed

    // Printing the initial values
    println!("Original &str: {}", s1);
    println!("Original String: {}", s2);

    // Changing the mutable String
    s2.push_str(" which is mutable"); // Append text to s2

    // Printing after modification
    println!("Modified String: {}", s2);

    // Uncommenting the next line will cause an error because &str is immutable
    // s1.push_str(" can't change this"); // This will NOT work
}
