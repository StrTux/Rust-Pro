fn main() {
    let x = "ashish";
    for (index, ch) in x.chars().enumerate() {
        if ch == 'a' {
            println!("Found  'a ' at  the index: {}", index);
            return;
        }
    }
    println!("'a' is no found")
}




// fn main () {
//     let  x = "ashish";
//     find_index(x);
// }

// fn find_index (x: &str) {
//     for (index, ch) in x.chars().enumerate() {
//         if ch == 'a' {
//             println!("'a' index is {}", index);
//             return;
//         }
//     }
// }

