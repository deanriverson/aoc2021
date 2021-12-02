use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_config(&args);

    println!("Reading from: {}", filename);

    let contents = fs::read_to_string(filename).expect("Could not read data file");

    let navs: Vec<&str> = contents.split("\n").collect();

    pilot_sub_1(&navs);
    pilot_sub_2(&navs);
}

fn parse_config(args: &[String]) -> &str {
    &args[1]
}

fn pilot_sub_1(navs: &Vec<&str>) {
    let mut pos = 0u32;
    let mut depth = 0u32;

    for nav in navs {
        let parts: Vec<&str> = nav.split(" ").collect();
        let cmd = parts[0];
        let dist: u32 = parts[1].parse().expect("Expected number here!");

        match cmd {
            "forward" => pos += dist,
            "down" => depth += dist,
            "up" => depth -= dist,
            _ => (),
        }
    }

    println!("Part 1 answer: {}", pos * depth);
}

fn pilot_sub_2(navs: &Vec<&str>) {
    let mut aim = 0u32;
    let mut pos = 0u32;
    let mut depth = 0u32;

    for nav in navs {
        let parts: Vec<&str> = nav.split(" ").collect();
        let cmd = parts[0];
        let dist: u32 = parts[1].parse().expect("Expected number here!");

        match cmd {
            "forward" => {
                pos += dist;
                depth += aim * dist;
            }
            "down" => aim += dist,
            "up" => aim -= dist,
            _ => (),
        }
    }

    println!("Part 2 answer: {}", pos * depth);
}
