use std::io;

fn main() {
    let n = read_usize();
    let arr = read_u32_vec(n);

    println!("{}", sum_u32(arr));
}

fn sum_u32(v: Vec<u32>) -> u32 {
    let mut sum = 0 as u32;

    for n in v {
        sum += n;
    }

    return sum;
}

fn read_usize() -> usize {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("read_usize(): couldn't read input to buffer");

    return buffer.trim().parse().expect("read_usize(): couldn't convert to usize");
}

fn read_u32_vec(n: usize) -> Vec<u32> {
    let mut result = vec![0 as u32; n];
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).expect("read_u32_vec(n: usize): couldn't read input to buffer");
    let buffer_arr: Vec<&str> = buffer.split(" ").collect();

    for (i, x) in buffer_arr.iter().enumerate() {
        result[i] = x.trim().parse::<u32>().expect("read_u32_vec(n: usize): couldn't convert to u32");
    }

    return result;
}
