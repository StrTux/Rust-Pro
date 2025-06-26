fn get_length(s: &str) -> usize {
    s.len()
}


fn main() {
    let input = "k";
    println!("The length of '{}' is {}", input, get_length(input));
}