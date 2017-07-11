use std::io;

pub fn read_str() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("read_str(): failed to read input to buffer");

    buffer
}

pub fn read_i8() -> i8 {
    read_str().trim().parse().expect("read_i8(): failed to parse as i8")
}

pub fn read_i16() -> i16 {
    read_str().trim().parse().expect("read_i16(): failed to parse as i16")
}

pub fn read_i32() -> i32 {
    read_str().trim().parse().expect("read_i32(): failed to parse as i32")
}

pub fn read_i64() -> i64 {
    read_str().trim().parse().expect("read_i64(): failed to parse as i64")
}

pub fn read_u8() -> u8 {
    read_str().trim().parse().expect("read_u8(): failed to parse as u8")
}

pub fn read_u16() -> u16 {
    read_str().trim().parse().expect("read_u16(): failed to parse as u16")
}

pub fn read_u32() -> u32 {
    read_str().trim().parse().expect("read_u32(): failed to parse as u32")
}

pub fn read_u64() -> u64 {
    read_str().trim().parse().expect("read_u64(): failed to parse as u64")
}

pub fn read_isize() -> isize {
    read_str().trim().parse().expect("read_isize(): failed to parse as isize")
}

pub fn read_usize() -> usize {
    read_str().trim().parse().expect("read_usize(): failed to parse as usize")
}

pub fn read_f32() -> f32 {
    read_str().trim().parse().expect("read_f32(): failed to parse as f32")
}

pub fn read_f64() -> f64 {
    read_str().trim().parse().expect("read_f64(): failed to parse as f64")
}

pub fn read_i8_vec() -> Vec<i8> {
    let input = read_str();
    let v: Vec<&str> = input.split(" ").collect();
    let mut result: Vec<i8> = Vec::new();

    for n in v {
        result.push(n.trim().parse().expect("read_i8_vec(): failed to parse as i8"));
    }

    result
}

pub fn read_i16_vec() -> Vec<i16> {
    let input = read_str();
    let v: Vec<&str> = input.split(" ").collect();
    let mut result: Vec<i16> = Vec::new();

    for n in v {
        result.push(n.trim().parse().expect("read_i16_vec(): failed to parse as i16"));
    }

    result
}

pub fn read_i32_vec() -> Vec<i32> {
    let input = read_str();
    let v: Vec<&str> = input.split(" ").collect();
    let mut result: Vec<i32> = Vec::new();

    for n in v {
        result.push(n.trim().parse().expect("read_i32_vec(): failed to parse as i32"));
    }

    result
}

pub fn read_i64_vec() -> Vec<i64> {
    let input = read_str();
    let v: Vec<&str> = input.split(" ").collect();
    let mut result: Vec<i64> = Vec::new();

    for n in v {
        result.push(n.trim().parse().expect("read_i64_vec(): failed to parse as i64"));
    }

    result
}

pub fn read_u8_vec() -> Vec<u8> {
    let input = read_str();
    let v: Vec<&str> = input.split(" ").collect();
    let mut result: Vec<u8> = Vec::new();

    for n in v {
        result.push(n.trim().parse().expect("read_u8_vec(): failed to parse as u8"));
    }

    result
}

pub fn read_u16_vec() -> Vec<u16> {
    let input = read_str();
    let v: Vec<&str> = input.split(" ").collect();
    let mut result: Vec<u16> = Vec::new();

    for n in v {
        result.push(n.trim().parse().expect("read_u16_vec(): failed to parse as u16"));
    }

    result
}

pub fn read_u32_vec() -> Vec<u32> {
    let input = read_str();
    let v: Vec<&str> = input.split(" ").collect();
    let mut result: Vec<u32> = Vec::new();

    for n in v {
        result.push(n.trim().parse().expect("read_u32_vec(): failed to parse as u32"));
    }

    result
}

pub fn read_u64_vec() -> Vec<u64> {
    let input = read_str();
    let v: Vec<&str> = input.split(" ").collect();
    let mut result: Vec<u64> = Vec::new();

    for n in v {
        result.push(n.trim().parse().expect("read_u64_vec(): failed to parse as u64"));
    }

    result
}
