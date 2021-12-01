use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = self::parse_config(&args);

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let depths: Vec<i32> = contents
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|d_str| d_str.parse::<i32>().unwrap())
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

    println!("window size 2 increases: {}", incs);
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

    println!("window size 3 increases: {}", incs);
}
