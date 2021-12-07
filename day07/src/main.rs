use std::{env, fs, str};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_config(&args);

    println!("Reading from: {}", filename);

    let contents = fs::read_to_string(filename).expect("Could not read data file");

    calc_solution_1(&contents);
    calc_solution_2(&contents);
}

fn calc_solution_1(contents: &str) {
    let xs = parse_numbers(contents);
    println!("Part 1 answer: {}", calculate_cost(&xs, cost));
}

fn calc_solution_2(contents: &str) {
    let xs = parse_numbers(contents);
    println!("Part 2 answer: {}", calculate_cost(&xs, cost_2));
}

fn parse_config(args: &[String]) -> &str {
    &args[1]
}

fn parse_numbers(contents: &str) -> Vec<i64> {
    contents.split(",").map(|s| s.parse().unwrap()).collect()
}

fn calculate_cost(xs: &Vec<i64>, cost_fn: fn(&Vec<i64>, i64) -> i64) -> i64 {
    let mut m = mean(&xs);
    let mut a = cost_fn(&xs, m - 1);
    let mut b = cost_fn(&xs, m);
    let mut c = cost_fn(&xs, m + 1);

    while b > a || b > c {
        if a < b {
            m -= 1;
        } else {
            m += 1;
        }

        a = cost_fn(&xs, m - 1);
        b = cost_fn(&xs, m);
        c = cost_fn(&xs, m + 1);
    }

    return b;
}

fn mean(xs: &Vec<i64>) -> i64 {
    xs.iter().sum::<i64>() / xs.len() as i64
}

fn cost(xs: &Vec<i64>, y: i64) -> i64 {
    xs.iter().fold(0, |acc, x| acc + (x - y).abs())
}

fn cost_2(xs: &Vec<i64>, y: i64) -> i64 {
    xs.iter().fold(0, |acc, x| {
        let delta = (x - y).abs();
        let sum = (delta + 1) * delta / 2;
        acc + sum
    })
}
