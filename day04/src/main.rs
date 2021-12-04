use std::{env, fs, str};

#[derive(Debug)]
struct Square {
    value: u32,
    filled: bool,
}

#[derive(Debug)]
struct Board {
    squares: Vec<Square>,
}

impl Board {
    pub fn new(vals: Vec<u32>) -> Self {
        let squares = vals
            .iter()
            .map(|value| Square {
                value: *value,
                filled: false,
            })
            .collect::<Vec<Square>>();

        Self { squares }
    }

    pub fn record_draw(&mut self, value: u32) {
        let res = self.squares.iter_mut().find(|s| s.value == value);
        if let Some(sq) = res {
            sq.filled = true;
        }
    }

    pub fn has_won(&self) -> bool {
        self.has_winning_row() || self.has_winning_col()
    }

    pub fn sum_unfilled(&self) -> u32 {
        self.squares
            .iter()
            .filter(|s| !s.filled)
            .fold(0, |acc, s| acc + s.value)
    }

    fn has_winning_row(&self) -> bool {
        self.squares[..]
            .chunks(5)
            .any(|ss| ss.iter().all(|s| s.filled))
    }

    fn has_winning_col(&self) -> bool {
        (0..5).any(|x| (0..5).all(|y| self.square_filled_at(x, y)))
    }

    fn square_filled_at(&self, x: usize, y: usize) -> bool {
        self.squares[x + y * 5].filled
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_config(&args);

    println!("Reading from: {}", filename);

    let contents = fs::read_to_string(filename).expect("Could not read data file");
    let lines: Vec<&str> = contents.split("\n").collect();

    calc_solution_1(&lines);
    calc_solution_2(&lines);
}

fn calc_solution_1(lines: &Vec<&str>) {
    let (mut boards, draws) = create_boards_and_draws(lines);

    for draw in draws {
        boards.iter_mut().for_each(|b| b.record_draw(draw));
        let winner = boards.iter().find(|b| b.has_won());

        if let Some(b) = winner {
            println!("Part 1 answer: {}", draw * b.sum_unfilled());
            return;
        }
    }
}

fn calc_solution_2(lines: &Vec<&str>) {
    let (mut boards, draws) = create_boards_and_draws(lines);

    for draw in draws {
        boards.iter_mut().for_each(|b| b.record_draw(draw));

        if boards.len() > 1 {
            boards.retain(|b| !b.has_won());
        } else if boards[0].has_won() {
            println!("Part 2 answer: {}", draw * boards[0].sum_unfilled());
            return;
        }
    }

    println!("Part 2: no answer found!");
}

fn create_boards_and_draws(lines: &Vec<&str>) -> (Vec<Board>, Vec<u32>) {
    let draws = lines[0]
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();

    let boards = lines[2..]
        .chunks(6)
        .map(|rows| {
            let vals = rows
                .iter()
                .map(|row| {
                    row.split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect::<Vec<u32>>()
                })
                .flatten()
                .collect::<Vec<u32>>();

            Board::new(vals)
        })
        .collect::<Vec<Board>>();

    (boards, draws)
}

fn parse_config(args: &[String]) -> &str {
    &args[1]
}

use std::cell::Cell;

/// Here I'm just playing with the idea of creating an object that owns
/// some data and has a reference to a piece of that data in a separate
/// field.  Doesn't look like this kind of thing is well-supported.
#[derive(Debug)]
struct BoardRef<'a> {
    squares: Vec<Square>,
    selected: Cell<&'a Square>,
}

impl<'a> BoardRef<'a> {
    fn new(vals: Vec<u32>) -> BoardRef<'a> {
        let squares = vals
            .iter()
            .map(|value| Square {
                value: *value,
                filled: false,
            })
            .collect::<Vec<Square>>();

        let selected = Cell::new(&Square {
            value: 0,
            filled: false,
        });

        // let selected = Cell::new(&squares[2]);

        Self { squares, selected }
    }

    fn assign_ref(&'a self) {
        self.selected.set(&self.squares[0]);
    }
}
