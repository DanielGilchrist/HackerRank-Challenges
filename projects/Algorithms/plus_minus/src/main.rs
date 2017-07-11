use std::io;

fn main() {
    let n = read_usize();
    let v = read_i32_vec(n);
    print_ratios(v);
}

fn print_ratios(v: Vec<i32>) {
    let mut pos: u32 = 0;
    let mut neg: u32  = 0;
    let mut zero: u32 = 0;
    let n = v.len() as u32;

    for n in v {
        if n > 0 {
            pos += 1;
        } else if n < 0 {
            neg += 1;
        } else {
            zero += 1;
        }
    }

    println!("{}\n{}\n{}", pos / n, neg / n, zero / n);
}

fn read_usize() -> usize {
    read_str().parse().expect("read_usize(): couldn't convert to usize")
}

fn read_str() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("read_usize(): couldn't read input to buffer");
    let result = buffer.trim();

    result.to_string()
}

fn read_i32_vec(n: usize) -> Vec<i32> {
    let temp: Vec<&str> = read_str().split(" ").collect();
    let mut v: Vec<i32> = Vec::new();

    for (i, n) in temp.iter().enumerate() {

    }

    v
}
