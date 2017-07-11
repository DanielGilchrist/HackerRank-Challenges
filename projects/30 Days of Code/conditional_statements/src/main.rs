use std::io;

fn main() {
    let stdin = io::stdin();

    let mut n = String::new();
    stdin.read_line(&mut n).expect("Can't read input to 'n'");
    let n: u32 = n.trim().parse().expect("Can't convert 'n' from &str to u32");

    let result =
        if (n % 2 != 0) || (n % 2 == 0 && n >= 6 && n <= 20) {
            "Weird"
        } else if (n % 2 == 0) && (n >= 2 && n <= 5) || (n > 20) {
            "Not Weird"
        } else {
            "Invalid input"
        };

        println!("{}", result);
}
