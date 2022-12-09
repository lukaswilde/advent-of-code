// Part 2 produces the wrong results

use std::{collections::HashSet, error::Error, str::FromStr};

use utils::parse_text;

fn main() {
    let text = parse_text();
    let mut instructions = collect_instructions(&text);
    let mut problem = Problem::new(instructions);
    let mut problem_orig = problem.clone();

    for i in 0..8 {
        problem.execute_instructions();
        instructions = problem.get_new_instructions();
        println!(
            "Iteration {}: {} endpoint: {:?}, {} endpoint: {:?}",
            i,
            i,
            problem.h_pos,
            i + 1,
            problem.t_pos
        );
        problem = Problem::new(instructions.clone());
    }
    problem.execute_instructions();
    problem_orig.execute_instructions();

    let num_visited_orig = problem_orig.get_unique_pos();
    let num_visited = problem.get_unique_pos();
    println!("The number of visited positions is {}", num_visited_orig);
    println!(
        "The number of visited positions with 10 knots is {}",
        num_visited
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

#[derive(Clone)]
struct Problem {
    instructions: Vec<Instruction>,
    pos_visited: HashSet<(isize, isize)>,
    h_pos: (isize, isize),
    t_pos: (isize, isize),
    output: Vec<Instruction>,
}

impl Problem {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            pos_visited: HashSet::from([(0, 0)]),
            h_pos: (0, 0),
            t_pos: (0, 0),
            output: Vec::new(),
        }
    }

    fn are_touching(&self) -> bool {
        let manhattan = self.h_pos.0.abs_diff(self.t_pos.0) + self.h_pos.1.abs_diff(self.t_pos.1);
        let both_different = self.h_pos.0 != self.t_pos.0 && self.h_pos.1 != self.t_pos.1;
        manhattan <= 1 || (manhattan == 2 && both_different)
    }

    fn step(&mut self, direction: &str) {
        let old_h_pos = self.h_pos;
        match direction {
            "L" => self.h_pos = (self.h_pos.0 - 1, self.h_pos.1),
            "R" => self.h_pos = (self.h_pos.0 + 1, self.h_pos.1),
            "U" => self.h_pos = (self.h_pos.0, self.h_pos.1 + 1),
            "D" => self.h_pos = (self.h_pos.0, self.h_pos.1 - 1),
            "DUR" => self.h_pos = (self.h_pos.0 + 1, self.h_pos.1 + 1),
            "DUL" => self.h_pos = (self.h_pos.0 - 1, self.h_pos.1 + 1),
            "DDR" => self.h_pos = (self.h_pos.0 + 1, self.h_pos.1 - 1),
            "DDL" => self.h_pos = (self.h_pos.0 - 1, self.h_pos.1 - 1),
            _ => panic!("Not a valid move instruction"),
        }
        if !self.are_touching() {
            let old_t_pos = self.t_pos;
            match direction {
                "L" | "R" | "U" | "D" => self.t_pos = old_h_pos,
                "DUR" => self.t_pos = (self.t_pos.0 + 1, self.t_pos.1 + 1),
                "DUL" => self.t_pos = (self.t_pos.0 - 1, self.t_pos.1 + 1),
                "DDR" => self.t_pos = (self.t_pos.0 + 1, self.t_pos.1 - 1),
                "DDL" => self.t_pos = (self.t_pos.0 - 1, self.t_pos.1 - 1),
                _ => panic!("Not a valid move instruction"),
            }
            self.pos_visited.insert(self.t_pos);
            match (self.t_pos.0 - old_t_pos.0, self.t_pos.1 - old_t_pos.1) {
                (-1, 0) => self.output.push(Instruction {
                    direction: "L".to_string(),
                    steps: 1,
                }),
                (0, 0) => (),
                (1, 0) => self.output.push(Instruction {
                    direction: "R".to_string(),
                    steps: 1,
                }),
                (0, 1) => self.output.push(Instruction {
                    direction: "U".to_string(),
                    steps: 1,
                }),
                (0, -1) => self.output.push(Instruction {
                    direction: "D".to_string(),
                    steps: 1,
                }),
                (1, 1) => self.output.push(Instruction {
                    direction: "DUR".to_string(),
                    steps: 1,
                }),
                (-1, 1) => self.output.push(Instruction {
                    direction: "DUL".to_string(),
                    steps: 1,
                }),
                (-1, -1) => self.output.push(Instruction {
                    direction: "DDL".to_string(),
                    steps: 1,
                }),
                (1, -1) => self.output.push(Instruction {
                    direction: "DDR".to_string(),
                    steps: 1,
                }),
                _ => panic!("This movement can't happen"),
            }
        }
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

    fn get_new_instructions(&self) -> Vec<Instruction> {
        self.output.clone()
    }
}
