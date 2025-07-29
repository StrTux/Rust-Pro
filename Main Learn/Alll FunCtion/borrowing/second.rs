fn main() {
    let s1 = String::from("ashish");

    do_some(&s1);
    println!("this is the s1 {}",s1);
}

fn do_some(s2 : &String) {
    println!("this is the s2 = 2 {}",s2)
}