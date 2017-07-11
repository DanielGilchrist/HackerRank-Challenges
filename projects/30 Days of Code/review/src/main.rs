use std::io;

fn main() {
    let n = read_usize();
    let cases = read_cases(n);

    for case in cases {
        print_result(case);
    }
}

fn print_result(string: String) {
    let (mut even, mut odd) = (String::new(), String::new());

    for (i, c) in string.chars().enumerate() {
        if i % 2 == 0 {
            even.push(c);
        } else {
            odd.push(c);
        }
    }

    println!("{} {}", even, odd);
}

fn read_str() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("read_str(): couldn't read input to buffer");
    let result = buffer.trim();

    result.to_string()
}

fn read_usize() -> usize {
    read_str().trim().parse().expect("read_usize(): couldn't convert input to usize")
}

fn read_cases(n: usize) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();

    for _ in 0..n {
        v.push(read_str());
    }

    v
}
