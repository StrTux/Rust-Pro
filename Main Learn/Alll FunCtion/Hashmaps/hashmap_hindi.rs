use std::collections::HashMap;


fn main() {
    let mut names: HashMap<&str , i32> = HashMap::new();

    names.insert("ashish", 420);
    names.insert("nikki", 360);
    names.insert("soni", 350);
    
    println!("this all the names inside this {:?}", names);

    match names.get("ashish")  {
        Some(name)=> println!("this is the names of {:?}", name),
        None => println!("there is no  data name nikki")
    }

    for (subject, name) in &names {
        println!("for {} you  got names {}", subject, name);
    }
    println!("for the  finding is avaible or not in the names : {:?}", names.contains_key("Sushila"))
}