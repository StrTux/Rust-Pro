//  used the move function 

fn create_string() {
    let s1 = String::from("hello");
    let s2 =  &s1;

    println!("print  the s1 :{} , s2 {} ",s1, s2);
}

fn main() {
    create_string();
}