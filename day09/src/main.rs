use std::{
    collections::HashSet,
    error::Error,
    ops::{Add, Sub},
    str::FromStr,
};

use utils::parse_text;

fn main() {
    let text = parse_text();
    let instructions = collect_instructions(&text);
    let mut problem = Problem::new(instructions.clone(), 2);
    let mut problem_alt = Problem::new(instructions, 10);

    problem.execute_instructions();
    problem_alt.execute_instructions();

    let num_visited = problem.get_unique_pos();
    let num_visited_alt = problem_alt.get_unique_pos();
    println!(
        "The number of visited positions with 2 knots is {}",
        num_visited
    );
    println!(
        "The number of visited positions with 10 knots is {}",
        num_visited_alt
    );
}

fn collect_instructions(text: &str) -> Vec<Instruction> {
    text.lines()
        .map(|l| {
            l.parse::<Instruction>()
                .expect("Should be convertible to instructions")
        })
        .collect()
}

#[derive(Clone, Debug)]
struct Instruction {
    direction: String,
    steps: usize,
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let args = s
            .split_once(' ')
            .expect("Splitting instruction should be possible");

        Ok(Self {
            direction: String::from(args.0),
            steps: args
                .1
                .parse()
                .expect("Second line value should be a step count"),
        })
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Point(isize, isize);

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Clone)]
struct Problem {
    instructions: Vec<Instruction>,
    pos_visited: HashSet<Point>,
    positions: Vec<Point>,
}

impl Problem {
    fn new(instructions: Vec<Instruction>, num_knots: usize) -> Self {
        Self {
            instructions,
            pos_visited: HashSet::from([Point(0, 0)]),
            positions: vec![Point(0, 0); num_knots],
        }
    }

    fn step(&mut self, direction: &str) {
        let head = &mut self.positions[0];
        *head = match direction {
            "L" => Point(head.0, head.1 - 1),
            "R" => Point(head.0, head.1 + 1),
            "U" => Point(head.0 - 1, head.1),
            "D" => Point(head.0 + 1, head.1),
            _ => panic!("Not a valid move instruction"),
        };

        let mut prev = head.clone();
        for i in 1..self.positions.len() {
            let knot = &mut self.positions[i];
            let to_move = match knot.clone() - prev.clone() {
                Point(2, 2) => Point(1, 1),
                Point(-2, -2) => Point(-1, -1),
                Point(2, -2) => Point(1, -1),
                Point(-2, 2) => Point(-1, 1),
                Point(2, _) => Point(1, 0),
                Point(-2, _) => Point(-1, 0),
                Point(_, 2) => Point(0, 1),
                Point(_, -2) => Point(0, -1),
                x => x,
            };
            *knot = prev.clone() + to_move;
            prev = knot.clone();
        }

        self.pos_visited
            .insert(self.positions.last().expect("Must be set").clone());
    }

    fn execute_instructions(&mut self) {
        for instruction in self.instructions.clone() {
            for _ in 0..instruction.steps {
                self.step(&instruction.direction);
            }
        }
    }

    fn get_unique_pos(&self) -> usize {
        self.pos_visited.len()
    }
}
