
//  this is using the while loop  
fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7];

    for (index, &value) in arr.iter().enumerate() {
        println!("i: {}, v: {}", index, value);
    }
}
