#[derive(Debug)]
enum Example {
    Float(f64),
    Int(i32),
    Str(()), // expects unit type
}

fn main() {
    let r = vec![
        Example::Int(142),
        Example::Float(3.14),
        Example::Str(()), // FIXED: passed the unit type
    ];
    println!("{:?}", r);
    println!("{:?}", r[2]);
}
