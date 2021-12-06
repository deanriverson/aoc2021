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
    let fish = parse_numbers_to_array(contents);
    println!("Part 1 answer: {}", age_fish_for_days(fish, 80));
}

fn calc_solution_2(contents: &str) {
    let fish = parse_numbers_to_array(contents);
    println!("Part 2 answer: {}", age_fish_for_days(fish, 256));
}

fn parse_config(args: &[String]) -> &str {
    &args[1]
}

fn parse_numbers_to_array(contents: &str) -> [i64; 9] {
    contents
        .split(",")
        .map(|s| s.parse().unwrap())
        .fold([0i64; 9], |mut a, n: usize| {
            a[n] += 1;
            a
        })
}

fn age_fish_for_days(mut fish: [i64; 9], d: i32) -> i64 {
    for _ in 0..d {
        fish = age_fish(fish);
    }
    fish.iter().sum::<i64>()
}

fn age_fish(fish: [i64; 9]) -> [i64; 9] {
    let new_day = [0i64, 0, 0, 0, 0, 0, fish[0], 0, fish[0]];

    (1..9).fold(new_day, |mut arr, i| {
        arr[i - 1] += fish[i];
        arr
    })
}

/* My original HashMap impl.
use std::collections::HashMap;

fn parse_numbers(contents: &str) -> HashMap<i64, i64> {
    contents
        .split(",")
        .map(|s| s.parse().unwrap())
        .fold(HashMap::new(), |mut map, n| {
            map.entry(n).and_modify(add(1)).or_insert(1);
            map
        })
}

fn age_fish_for_days(mut fish: HashMap<i64, i64>, d: i32) -> i64 {
    for _ in 0..d {
        fish = age_fish(fish);
    }

    fish.iter().sum::<i64>()
}

fn age_fish(mut fish: HashMap<i64, i64>) -> HashMap<i64, i64> {
    fish.iter_mut().fold(HashMap::new(), |mut map, (k, v)| {
        if *k == 0 {
            map.insert(8, *v);
            map.entry(6).and_modify(add(*v)).or_insert(*v);
        } else {
            map.entry(*k - 1).and_modify(add(*v)).or_insert(*v);
        }

        return map;
    })
}

fn add(x: i64) -> Box<dyn Fn(&mut i64)> {
    Box::new(move |i: &mut i64| *i += x)
}
*/
