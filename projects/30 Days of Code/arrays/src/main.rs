use std::io;

fn main() {
    let n = read_usize();
    let v = read_u32_vec(n);

    reverse(v);
}

fn reverse(v: Vec<u32>) {
    let mut rev = v.clone();
    rev.reverse();
    let mut s_vec: Vec<String> = Vec::new();

    for n in rev {
        s_vec.push(n.to_string());
    }

    println!("{}", s_vec.join(" "));
}

fn read_str_single() -> String {
    read_str().trim().to_string()
}

fn read_str() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("read_str(): couldn't read input to buffer");

    buffer
}

fn read_usize() -> usize {
    read_str_single().parse().expect("read_usize(): failed to parse")
}

fn read_u32_vec(n: usize) -> Vec<u32> {
    let stemp = read_str();
    let s: &str = &*stemp;
    let temp: Vec<&str> = s.split(" ").collect();
    let mut v: Vec<u32> = Vec::new();

    for i in 0..n {
        v.push(temp[i].trim().parse::<u32>().expect("read_u32_vec(n: usize): failed to parse"));
    }

    v
}
