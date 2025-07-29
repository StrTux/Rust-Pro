use chrono::{Local,Utc};

fn main() {
    let now = Utc::now();
    println!("this is the time {}", now);


    //   yaha pe mene bbatay  kis tarike se date and time ko mujhe show akrna hay     
    let formatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("Formatted date and time: {}",formatted);


    // it will  call the local  time 
    let local= Local::now();
    println!("current date and time in local:{}", local)

}