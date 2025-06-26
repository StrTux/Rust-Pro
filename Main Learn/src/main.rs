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