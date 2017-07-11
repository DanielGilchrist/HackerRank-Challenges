use std::io;

fn main() {
    let stdin = io::stdin();

    println!("Hello, World.");

    let mut input_string = String::new();
    stdin.read_line(&mut input_string).expect("Couldn't read input");

    println!("{}", input_string);
}
