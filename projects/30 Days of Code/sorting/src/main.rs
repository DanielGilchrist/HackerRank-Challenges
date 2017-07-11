extern crate read_input;

fn main() {
    read_input::read_str();
    let v = read_input::read_u32_vec();

    sort(v);
}

fn sort(mut v: Vec<u32>) {
    let mut number_of_swaps = 0;
    for _ in 0..v.len() {
        for j in 0..v.len() - 1 {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
                number_of_swaps += 1;
            }
        }

        if number_of_swaps == 0 {
            break;
        }
    }

    print!("Array is sorted in {} swaps.\nFirst Element: {}\nLast Element: {}",
             number_of_swaps, v[0], v[v.len() - 1]);
}
