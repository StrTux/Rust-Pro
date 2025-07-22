fn split_at_index (s: &str, mid :usize) ->  (&str, &str){
    if mid > s.len()  {
        return (s, "");
    }
    s.split_at(mid)
}


/// Splits a given string at a given index and prints the two parts.
/// This program shows how to split a string and print the two parts.
fn main() {
 let my_string = "Hello, world!";
 let (first, second) = split_at_index(my_string, 5);
 println!("First part: {}", first);
 println!("Second part: {}", second);
}