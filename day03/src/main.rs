use std::{env, fs, str};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_config(&args);

    println!("Reading from: {}", filename);

    let contents = fs::read_to_string(filename).expect("Could not read data file");
    let lines: Vec<&[u8]> = contents.split("\n").map(|s| s.as_bytes()).collect();

    // let ls = vec![
    //     "1010101".as_bytes(),
    //     "0101010".as_bytes()
    // ];

    calc_solution_1(&lines);
    calc_solution_2(&lines);
}

fn calc_solution_1(lines: &Vec<&[u8]>) {
    let mut gamma = 0u32;
    let mut epsilon = 0u32;

    let line_len = lines[0].len();
    let line_count = lines.len();

    for i in 0..line_len {
        let mut one_count = 0;

        for line in lines {
            if line[i] == b'1' {
                one_count += 1;
            }
        }

        gamma = gamma << 1;
        epsilon = epsilon << 1;

        if one_count > line_count / 2 {
            gamma = gamma | 1;
        } else {
            epsilon = epsilon | 1;
        }
    }

    println!("Part 1 answer: {}", gamma * epsilon);
}

fn calc_solution_2(lines: &Vec<&[u8]>) {
    let oxy = find_life_support(lines, true);
    let co2 = find_life_support(lines, false);
    println!("Part 2 answer: {}", oxy * co2);
}

fn find_life_support(lines: &Vec<&[u8]>, is_oxy: bool) -> u32 {
    let line_len = lines[0].len();
    let mut lines = lines.clone();

    for i in 0..line_len {
        let init: (Vec<&[u8]>, Vec<&[u8]>) = (Vec::new(), Vec::new());

        let res = lines.iter().fold(init, |mut acc, line| {
            if line[i] == b'1' {
                acc.1.push(line);
            } else {
                acc.0.push(line);
            }
            acc
        });

        lines = if is_oxy {
            select_gt(res.0, res.1)
        } else {
            select_lte(res.0, res.1)
        };

        if lines.len() == 1 {
            return parse_binary_str(str::from_utf8(lines[0]).unwrap());
        }
    }

    return 0;
}

fn select_gt<'a>(fst: Vec<&'a [u8]>, snd: Vec<&'a [u8]>) -> Vec<&'a [u8]> {
    if fst.len() > snd.len() {
        fst
    } else {
        snd
    }
}

fn select_lte<'a>(fst: Vec<&'a [u8]>, snd: Vec<&'a [u8]>) -> Vec<&'a [u8]> {
    if fst.len() <= snd.len() {
        fst
    } else {
        snd
    }
}

fn parse_config(args: &[String]) -> &str {
    &args[1]
}

fn parse_binary_str(s: &str) -> u32 {
    u32::from_str_radix(s, 2).expect("Line was not a binary number!")
}
