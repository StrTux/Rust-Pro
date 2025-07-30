fn main() {
    let num = vec![1,2,3,4];

    let even = even_num(&num);

    println!("This is the original vec: {:?}", num);
    println!("These are the even numbers: {:?}", even);
}

fn even_num(numbers : &Vec<i32>) -> Vec<i32> {
    numbers.iter()
        .copied()
        .filter(|x| x % 2 == 0)
        .collect()
}



// fn main() {
//     let vec = vec![1,2,3,4,5,6];



//     let evens: Vec<i32> = vec
//         .iter()
//         .cloned() // Or .copied()
//         .filter(|x| x % 2 == 0)
//         .collect();
        
//     println!("this is done as vec {:?}", vec);
//     println!("this is the done {:?}", evens)
// }'


// fn main() {
//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);
//     vec.push(3);
//     vec.push(4);
//     vec.push(5);
//     vec.push(6);

    
//     let evens: Vec<i32> =  vec
//      .iter()
//      .cloned()
//      .filter(|x| x % 2 == 0)
//      .collect();

//     println!("the  value of the vec is {:?}", vec);
//     println!("the value of the even  is {:?}", evens);

// }