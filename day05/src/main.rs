use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::{env, fs, str};

type PointMap = HashMap<i32, HashMap<i32, i32>>;

#[derive(Debug)]
struct Point(i32, i32);

#[derive(Debug)]
struct Span(Point, Point);

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_config(&args);

    println!("Reading from: {}", filename);

    let contents = fs::read_to_string(filename).expect("Could not read data file");

    calc_solution_1(&contents);
    calc_solution_2(&contents);
}

fn calc_solution_1(lines: &str) {
    let map = parse_points(lines, false);
    let count = count_danger_points(map);

    println!("Part 1 answer: {}", count);
}

fn calc_solution_2(lines: &str) {
    let map = parse_points(lines, true);
    let count = count_danger_points(map);

    println!("Part 2 answer: {}", count);
}

fn parse_config(args: &[String]) -> &str {
    &args[1]
}

fn parse_points(lines: &str, include_diagonals: bool) -> PointMap {
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

    re.captures_iter(lines)
        .map(build_span)
        .filter(|s| include_diagonals || !is_diagonal(&s))
        .fold(PointMap::new(), add_span)
}

fn to_i32(s: &str) -> i32 {
    s.parse().unwrap()
}

fn build_span(cap: regex::Captures) -> Span {
    Span(
        Point(to_i32(&cap[1]), to_i32(&cap[2])),
        Point(to_i32(&cap[3]), to_i32(&cap[4])),
    )
}

fn add_span(mut map: PointMap, span: Span) -> PointMap {
    let pt0 = &span.0;
    let pt1 = &span.1;

    let delta_x = calc_delta(pt0.0, pt1.0);
    let delta_y = calc_delta(pt0.1, pt1.1);

    let mut x = pt0.0;
    let mut y = pt0.1;

    loop {
        add_point(&mut map, x, y);

        if x == pt1.0 && y == pt1.1 {
            return map;
        }

        x += delta_x;
        y += delta_y;
    }
}

fn add_point(map: &mut PointMap, x: i32, y: i32) {
    let col = map.entry(x).or_insert(HashMap::new());
    let val = col.entry(y).or_insert(0);
    *val += 1;
}

fn calc_delta(a: i32, b: i32) -> i32 {
    match a.cmp(&b) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

fn is_diagonal(span: &Span) -> bool {
    span.0 .0 != span.1 .0 && span.0 .1 != span.1 .1
}

fn count_danger_points(map: PointMap) -> i32 {
    map.into_values()
        .map(|m| m.into_values())
        .flatten()
        .fold(0, |acc, val| if val > 1 { acc + 1 } else { acc })
}
