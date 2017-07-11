use std::io;

fn main() {
    let stdin = io::stdin();
    let mut s = String::new();
    stdin.read_line(&mut s).expect("Can't read input");

    reduce(s);
}

fn reduce(s: String) {
    let mut letters: Vec<&str> = s.trim().split("").collect();

    let mut i = 1;
    let mut prev = letters[0];
    while i < letters.len() {
        if letters[i] == prev {
            letters.remove(i);
            letters.remove(i - 1);
            i = 1;
            prev = letters[0];
        }

        i += 1;
        prev = letters[i - 1];
    }

    let result = letters.join("");
    if result.trim().len() > 0 {
        println!("{}", result);
    } else {
        println!("Empty String");
    }
}
