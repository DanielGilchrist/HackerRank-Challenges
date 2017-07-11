use std::io;

fn main() {
    let (a, b) = (read_u32_vec(), read_u32_vec());
    compare_triplets(a, b)
}

fn compare_triplets(a: Vec<u32>, b: Vec<u32>) {
    let (mut alice, mut bob) = (0 as u32, 0 as u32);

    for i in 0..a.len() {
        if a[i] > b[i] {
            alice += 1;
        } else if a[i] < b[i] {
            bob += 1;
        }
    }

    println!("{} {}", alice, bob);
}

fn read_u32_vec() -> Vec<u32> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("read_u32_vec(): couldn't read input to buffer");
    let buffer_arr: Vec<&str> = buffer.split(" ").collect();

    let mut result = vec![0 as u32; buffer_arr.len()];

    for (i, n) in buffer_arr.iter().enumerate() {
        result[i] = n.trim().parse().expect("read_u32_vec(): couldn't convert to u32");
    }

    return result;
}
