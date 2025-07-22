use std::fs;

fn main() {
    let poem = "poem.txt";

    let contents =
        fs::read_to_string(poem).expect("if there is no  text it  means something went wrong");

    println!("this is the content inside the the text: {}", contents)
}



// use std::fs;

// fn main() {
//     let greeting_file_result = fs::read_to_string("hello.txt");

//     match greeting_file_result {
//         Ok(file_content) => {
//             println!("File read successfully: {:?}", file_content);
//         },
//         Err(error) => {
//             println!("Failed to read file: {:?}", error);
//         }
//     }
// }
