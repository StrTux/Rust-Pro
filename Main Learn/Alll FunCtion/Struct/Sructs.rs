//  structc is like objest  in javascript jiske under apn  email  ki  sting int  sab  defin karte hay  user input kya lega woo 

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
    age: u8,
}

fn create_user(username: String, email: String, age: u8) -> Result<User, String> {
    if !email.contains('@') || !email.contains('.') || email.starts_with('@') || email.ends_with('@') {
        return Err(String::from("Invalid email address"));
    }

    Ok(User {
        active: true,
        username,
        email,
        sign_in_count: 1,
        age,
    })
}

fn main() {
    let username = String::from("Ashish Tiwari");
    let email = String::from("ashish03@gmail.com");
    let age = 20;

    match create_user(username, email, age) {
        Ok(user1) => {
            println!("Username: {}, Email: {}", user1.username, user1.email);
            println!("Sign In Count: {}", user1.sign_in_count);
            println!("Age: {}", user1.age);
            println!("Active: {}", user1.active);
        }
        Err(err) => {
            println!("Error creating user: {}", err);
        }
    }
}
