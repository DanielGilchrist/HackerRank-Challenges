use std::io;

fn main() {
    let stdin = io::stdin();

    // read input
    let mut input = String::new();
    stdin.read_line(&mut input).expect("Failed to read input");

    // split input up and store into an array and then store converted values
    // into separate variables
    let input1: Vec<&str> = input.split(" ").collect();
    let (n, k) = (input1[0].trim().parse::<usize>().expect("Not a number"),
                  input1[1].trim().parse::<usize>().expect("Not a number"));

    // read input
    let mut input = String::new();
    stdin.read_line(&mut input).expect("Failed to read input");

    // split input up and store into an array and declare an empty array for converted values
    let input2: Vec<&str> = input.split(" ").collect();
    let mut a = vec![0; n];

    // store split input into the 'a' array after converting value to i32 integer
    for (i, num) in input2.iter().enumerate() {
        a[i] = num.trim().parse::<i32>().expect("Not a number");
    }

    // n = number of elements in a
    // k = number of shifts
    // a = array of numbers
    left_shift(n, k, a);
}

fn left_shift(n: usize, k: usize, a: Vec<i32>) {
    let mut rotated = vec![0; n];
    let mut i = 0;
    while i < n {
        rotated[(i - k + n) % n] = a[i];
        i += 1;
    }

    let mut srotated = vec![];
    for num in rotated {
        srotated.push(num.to_string());
    }

    println!("{}", srotated.join(" "));
}
