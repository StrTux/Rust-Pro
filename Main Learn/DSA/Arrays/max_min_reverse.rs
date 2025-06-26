// fn min_array(arr: &[i32]) -> i32 {
//     let mut min = arr[0];
//     for &num in arr.iter() {
//         if num < min {
//             min = num;
//         }
//     }
//     min
// }

// fn max_array(arr: &[i32]) -> i32 {
//     let mut max = arr[0];
//     for &num in arr.iter() {
//         if num > max {
//             max = num;
//         }
//     }
//     max
// }


// fn reverse_array(arr: &[i32]) -> Vec<i32> {
//     arr.iter().rev().cloned().collect()
// }


fn main() {
    let arr = [5,6,4,8,9];
    // println!("The minimum is: {}", min_array(&arr));
    // println!("The maximum is: {}", max_array(&arr));
    println!("The reversed array is: {:?}", reverse_array(&arr));
    println!("this is  the max array: {}", max_array(&arr));
}