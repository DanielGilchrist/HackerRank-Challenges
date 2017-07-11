use std::io;

fn main() {
    let n = read_usize();
    let v = read_i32_2d_vec(n);
    diagonal_difference(v);
}

fn diagonal_difference(v: Vec<Vec<i32>>) {
    let l = v[0].len();
    let mut x1 = vec![0 as i32; l];
    let mut x2 = vec![0 as i32; l];

    for i in 0..l {
        x1[i] = v[i][i];
        x2[i] = v[l - 1 - i][i];
    }

    println!("{}", (sum_i32(x1) - sum_i32(x2)).abs());
}

fn sum_i32(v: Vec<i32>) -> i32 {
    let mut sum = 0 as i32;

    for n in v {
        sum += n;
    }

    sum
}

fn read_usize() -> usize {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("read_usize(): couldn't read input to buffer");

    buffer.trim().parse().expect("read_usize(): couldn't convert to usize")
}

fn read_i32_vec(n: usize) -> Vec<i32> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("read_i32_vec(n: usize): couldn't read input to buffer");
    let buffer_arr: Vec<&str> = buffer.split(" ").collect();
    let mut result = vec![0 as i32; n];

    for (i, n) in buffer_arr.iter().enumerate() {
        result[i] = n.trim().parse().expect("read_i32_vec(n: usize) couldn't convert to i32");
    }

    result
}

fn read_i32_2d_vec(n: usize) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![Vec::new(); n];

    for i in 0..n {
        result[i].append(&mut read_i32_vec(n));
    }

    result
}
