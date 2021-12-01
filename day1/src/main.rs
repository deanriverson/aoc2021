use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_config(&args);

    println!("Reading from: {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let depths: Vec<i32> = contents
        .split("\n")
        .map(|d_str| d_str.parse().unwrap())
        .collect();

    count_depths_1(&depths);
    count_depths_2(&depths);
}

fn parse_config(args: &[String]) -> &str {
    &args[1]
}

fn count_depths_1(depths: &Vec<i32>) {
    let mut incs = 0;

    for window in depths[..].windows(2) {
        if window[1] > window[0] {
            incs += 1;
        }
    }

    println!("Part 1 increases: {}", incs);
}

fn count_depths_2(depths: &Vec<i32>) {
    let mut incs = 0;
    let mut last = 0;

    for window in depths[..].windows(3) {
        let sum: i32 = window.iter().sum();
        if last > 0 {
            if sum > last {
                incs += 1;
            }
        }

        last = sum;
    }

    println!("Part 2 increases: {}", incs);
}
