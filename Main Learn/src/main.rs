use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: grep-lite <query> <filename>");
        return;
    }

    let (query, filename) = parse_config(&args);

    // --snip--

}

fn parse_config(args: &[String]) -> (&str, &str) {

    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
