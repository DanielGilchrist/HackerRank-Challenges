use std::io;

fn main() {
    let stdin = io::stdin();
    let mut s = String::new();
    stdin.read_line(&mut s).expect("Can't read input");

    num_words(s);
}

fn num_words(s: String) {
    let letters = s.chars();
    let mut count = 1;

    for letter in letters {
        if letter.is_uppercase() {
            count += 1;
        }
    }

    println!("{}", count);
}
