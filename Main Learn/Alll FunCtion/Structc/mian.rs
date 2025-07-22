struct React {
    width: u32,
    height: u32,
}

impl React {
    fn area(&self) -> u32 {
        self.width * self.height
    }


    fn print_something(&self) {
        println!("This is a React struct");
    }
}

 
fn main() {
    let r = React {
        width: 30,
        height: 50,
    };

    println!("Area: {}", r.area());
    r.print_something();
}


//  where the struct's data is stored whether it is stored in the heap or stack 

//  if data is given inside it, it is stored in the heap and assigned to a variable, then it is stored in the stack
//  if data is not given, it is stored in the stack