fn main() {
    let n = 10;
    println!("{}", nth_fibonacci(n))
}

fn nth_fibonacci(n: i32) -> i32 {
    if (n <= 1) {
        return n;
    } else {
        return nth_fibonacci(n - 1) + nth_fibonacci(n - 2);
    }
}

// ----------------------------------------

// fn fib(num: i32) -> i32 {
//     if num == 0 {
//         return 0;
//     }
//     if num == 1 {
//         return 1;
//     }

//     let mut first = 0;
//     let mut second = 1;

//     for _ in 2..=num {
//         let temp = second;
//         second = second + first;
//         first = temp;
//     }

//     second
// }

