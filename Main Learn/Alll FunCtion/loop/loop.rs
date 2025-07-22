pub fn main() {
    let str = String::from("Ashish Tiwari");
    println!("First name is {}", get_first_name(&str))
}

pub fn get_first_name(str: &String) -> String {
    let mut first_name = String::from("");
    let mut chars = str.chars();
    while let Some(c) = chars.next() {
        if c == ' ' {
            break;
        }
        first_name.push(c);
    }
    return first_name;
}
