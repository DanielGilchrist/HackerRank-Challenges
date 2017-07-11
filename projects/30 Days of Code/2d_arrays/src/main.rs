extern crate read_input;
use std::str::FromStr;
const NUM_ROWS: usize = 6;

fn main() {
    let input = read_2d_vec();

    print_sum(input);
}

fn print_sum(input: Vec<Vec<i8>>) {
    let hourglass_sums = get_hourglass_sums(&input);
    let max_sum = max_sum(&hourglass_sums);

    println!("{}", max_sum);
}

fn get_hourglass_sums(input: &Vec<Vec<i8>>) -> Vec<i32> {
    let mut hourglass_sums: Vec<i32> = Vec::new();
    let loop_num = NUM_ROWS - 2;
    for c in 0..loop_num {
        for r in 0..loop_num {
            hourglass_sums.push(hourglass_sum(input[c][r], input[c][r + 1], input[c][r + 2],
                                              input[c + 1][r + 1], input[c + 2][r],
                                              input[c + 2][r + 1], input[c + 2][r + 2]));
        }
    }

    hourglass_sums
}

fn hourglass_sum(a: i8, b: i8, c: i8, d: i8, e: i8, f: i8, g: i8) -> i32 {
    (a + b + c + d + e + f + g) as i32
}

fn max_sum(sums: &Vec<i32>) -> i32 {
    let mut max = sums[0];

    for sum in sums {
        if sum > &max {
            max = *sum;
        }
    }

    max
}

fn read_i8_2d_vec() -> Vec<Vec<i8>> {
    let mut result: Vec<Vec<i8>> = Vec::new();

    for _ in 0..NUM_ROWS {
        result.push(read_input::read_i8_vec());
    }

    result
}

fn read_2d_vec<T: FromStr>() -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = Vec::new();

    for _ in 0..NUM_ROWS {
        result.push(read_vec());
    }

    result
}

fn read_vec<T: FromStr>() -> Vec<T> {
    let input = read_input::read_str();
    let v: Vec<&str> = input.split(" ").collect();
    let mut result: Vec<T> = Vec::new();

    for n in v {
        result.push(n.trim().parse::<T>().expect("read_vec(): failed to parse"));
        
    }
    
    result
}
