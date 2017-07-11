extern crate read_input;

fn main() {
    let n = read_input::read_u32();
    let b = format!("{:b}", n);
    let vec: Vec<&str> = b.split("").collect();
    let mut c_ones = 0;
    let mut max_ones = 0;

    for v in vec {
        if v == "1" {
            c_ones += 1;
        } else {
            c_ones = 0;
        }

        if c_ones > max_ones {
            max_ones = c_ones;
        }
    }

    println!("{}", max_ones);
}
