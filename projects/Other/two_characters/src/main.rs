// https://www.hackerrank.com/challenges/two-characters?h_r=next-challenge&h_v=zen

use std::io;

fn main() {
    let stdin = io::stdin();

    let mut s = String::new();
    stdin.read_line(&mut s).expect("Can't read input");

    two_chars(s);
}

fn two_chars(s: String) {
    let mut c: Vec<&str> = s.trim().split("").collect();
    c.dedup();

    let letters: Vec<&str> = s.trim().split("").collect();
    let mut result = String::new();
    let mut valid = false;

    let mut i = 0;
    while i < c.len() {
        let mut j = 0;
        let mut temp = letters.clone();
        while j < temp.len() {
            if temp[j] == c[i] {
                temp.remove(j);
            }

            j += 1;
        }

        let s = temp.join("");
        if is_valid(&s) {
            result = s;
            valid = true;
            break;
        }

        i += 1;
    }

    if !valid {
        result = 0.to_string();
    }

    println!("{}, {}", result.len(), result);
}

fn is_valid(s: &str) -> bool {
    let letters: Vec<&str> = s.split("").collect();
    let mut prev = letters[0];
    let mut i = 1;
    while i < letters.len() {
        if letters[i] == prev {
            return false;
        }

        i += 1;
        prev = letters[i - 1];
    }

    return true;
}
