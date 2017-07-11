use std::io;

fn main() {
    let n = read_usize();
    let v = read_u64_vec(n);
    sum(v);
}

fn sum(v: Vec<u64>) {
    let mut sum = 0 as u64;

    for n in v {
        sum += n;
    }

    println!("{}", sum);
}

fn read_usize() -> usize {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("read_usize(): couldn't read input to buffer");

    buffer.trim().parse::<usize>().expect("read_usize(): couldn't convert to u32")
}

fn read_u64_vec(n: usize) -> Vec<u64> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("read_u64_vec(n: usize): couldn't read input to buffer");
    let buffer_arr: Vec<&str> = buffer.split(" ").collect();
    let mut result = vec![0 as u64; n];

    for (i, n) in buffer_arr.iter().enumerate() {
        result[i] = n.trim().parse().expect("read_u64_vec(n: usize): couldn't convert to u64");
    }

    result
}
