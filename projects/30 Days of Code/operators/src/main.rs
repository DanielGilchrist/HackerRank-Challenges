use std::io;

fn main() {
    let stdin = io::stdin();
    let (mut meal_cost, mut tip_percent, mut tax_percent) = (String::new(), String::new(), String::new());
    stdin.read_line(&mut meal_cost).expect("Can't read input");
    stdin.read_line(&mut tip_percent).expect("Can't read input");
    stdin.read_line(&mut tax_percent).expect("Can't read input");

    let meal_cost: f32 = meal_cost.trim().parse().expect("Can't convert meal_cost to f32");
    let tip_percent: u32 = tip_percent.trim().parse().expect("Can't convert tip_percent to u32");
    let tax_percent: u32 = tax_percent.trim().parse().expect("Can't convert tax_percent to u32");

    println!("The total meal cost is {} dollars.",
            (meal_cost + (meal_cost * tip_percent as f32 / 100.0) + (meal_cost * tax_percent as f32 / 100.0)).round() as u32);
}
