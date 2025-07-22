fn main() {
    let name = String::from("ashish");
    let len = get_str(name);
    println!("the length  of the string is {} ",len)
}

fn get_str(str: String)-> usize {
    str.chars().count()
}
//the length  of the string is 6 