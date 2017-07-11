extern crate read_input;
use std::collections::BTreeMap;

fn main() {
    let n = read_input::read_usize();
    let phone_nums = read_hashmap(n);
    let names = read_vec_str();

    print_phone_book(phone_nums, names);
}

fn print_phone_book(phone_nums: BTreeMap<String, u32>, names: Vec<String>) {
    for i in 0..names.len() {
        let immutable: &str = &names[i].trim();
        if phone_nums.contains_key(immutable) {
            println!("{}={}", immutable, phone_nums[immutable]);
        } else {
            println!("Not found");
        }
    }
}

fn read_hashmap(n: usize) -> BTreeMap<String, u32> {
    let mut phone_nums = BTreeMap::new();

    for _ in 0..n {
        let temp = read_input::read_str();
        let v: Vec<_> = temp.split(" ").collect();
        phone_nums.insert(v[0].to_string(),
                          v[1].trim().parse::<u32>().expect("Unable to parse as u32"));
    }

    phone_nums
}

fn read_vec_str() -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    loop {
        let input = read_input::read_str().trim().to_string();
        if input.is_empty() { break }
        v.push(input);
    }

    v
}
