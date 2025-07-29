fn main() {
    let s1 = String::from("ashish");

    do_some(&s1);
    println!("this is the s1 {}",s1);
}

fn do_some(s2 : &String) {
    println!("this is the s2 = 2 {}",s2)
}

//  what  we can  do  other thing is 

// fn main() {
//     let s1 = String::from("ashish");
//     let s2 = &s1; //  which  will  borrow from this

//     println!("this is the s1 {}",s1);
//     println!("this is the s2 {}",s2);
// }

//  this is the also  working phase 