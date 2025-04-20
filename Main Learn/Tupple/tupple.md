fn  main () {
    // fist  way   isme ye direct  data  ek  stored  jagh  se lera hay  pehle uske under  store karaywa 
    // fir waha se stored  jagh se use me lera hay  
    let  emp_info :(&str, u8) = ("Ashish", 20);
    let (name, age  ) = emp_info;

    println!("Employee Name: {}", name);
    println!("Employee Age: {}", age);

    // second way in this way  ye jo hay index value se chije lera hay data information  direct  emp_info se 
    let emp_info2 = ("=Nikki", 25);
    let name2 = emp_info2.0;
    let age2 = emp_info2.1;
    println!("Employee Name: {}", name2);
    println!("Employee Age: {}", age2);
}