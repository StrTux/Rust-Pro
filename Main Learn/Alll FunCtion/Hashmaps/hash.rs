// use std::collections::HashMap;


// fn build_db() -> HashMap<String, i32> {
//     let mut  user = HashMap::new();
//     user.insert("ashish".to_string(), 22);
//     user.insert("nikki".to_string(), 32);
//     user.insert("rahul".to_string(), 28);
//     user
// }


// fn print_user_age (user: HashMap<String, i32>,name: &str) {
//     match user.get(name) {
//         Some(age) => println!("✅ {name}'s age is {age}"),
//         None => println!("No data available for user: {name}"),
//     }

// }

// fn main() {
//     let users = build_db();

//     let target_users = vec!["ashish", "nikki", "unknown"];

//     for user in target_users {
//         print_user_age(users.clone(), &user);
//     }

//     println!("\n📊 All Users in DB:");
//     for (name, age) in users {
//         println!("- {name}: {age} years old");
//     }
// }