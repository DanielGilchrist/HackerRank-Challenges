use std::io;

fn main() {
    let n = read_u32();

    for i in 0..10 {
        println!("{} x {} = {}", n, i + 1, n * (i + 1));
    }
}

fn read_u32() -> u32 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("read_u32(): can't read input to buffer");
    let n: u32 = buffer.trim().parse().expect("read_u32(): can't convert to u32");

    return n;
}
