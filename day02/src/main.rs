use std::{env, fs, slice::Iter, str};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_config(&args);

    println!("Reading from: {}", filename);

    let contents = fs::read_to_string(filename).expect("Could not read data file");

    let navs: Vec<(&str, u32)> = contents.split("\n").map(line_to_str_u32).collect();

    calc_solution_1(navs.iter());
    calc_solution_2(navs.iter());
}

// fn calc_solution_1(navs: &Vec<(&str, u32)>) {
// let mut pos = 0u32;
// let mut depth = 0u32;

// for nav in navs {
//     match nav {
//         ("forward", dist) => pos += dist,
//         ("down", dist) => depth += dist,
//         ("up", dist) => depth -= dist,
//         _ => (),
//     };
// }
//
// println!("Part 1 answer: {}", pos * depth);
// }

fn calc_solution_1(navs: Iter<(&str, u32)>) {
    let res = navs.fold((0, 0), |(pos, depth), nav| {
        return match nav {
            ("forward", dist) => (pos + dist, depth),
            ("down", dist) => (pos, depth + dist),
            ("up", dist) => (pos, depth - dist),
            _ => (pos, depth),
        };
    });

    println!("Part 1 answer: {}", res.0 * res.1);
}

fn calc_solution_2(navs: Iter<(&str, u32)>) {
    let mut aim = 0u32;
    let mut pos = 0u32;
    let mut depth = 0u32;

    for nav in navs {
        match nav {
            ("forward", dist) => {
                pos += dist;
                depth += aim * dist;
            }
            ("down", dist) => aim += dist,
            ("up", dist) => aim -= dist,
            _ => (),
        }
    }

    println!("Part 2 answer: {}", pos * depth);
}

fn parse_config(args: &[String]) -> &str {
    &args[1]
}

fn line_to_str_u32(line: &str) -> (&str, u32) {
    let mut parts = line.split(" ");
    (
        parts.next().unwrap(),
        parts
            .next()
            .unwrap()
            .parse::<u32>()
            .expect("Expected number here!"),
    )
}
