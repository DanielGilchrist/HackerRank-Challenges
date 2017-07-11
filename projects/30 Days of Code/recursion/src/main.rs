extern crate read_input;

fn main() {
    let n = read_input::read_i32();
    println!("{}", factorial(n));
}

fn factorial(n: i32) -> i32 {
    if n >= 1 {
        n * factorial(n - 1)
    } else {
        1
    }
}
