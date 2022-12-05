use std::str::FromStr;

use utils::parse_text;

fn main() {
    let text = parse_text();
    let (problem, instructions) = split_instructions(&text);
    let instructions = get_instructions(instructions);

    let mut problem: Problem = problem.parse().expect("Should be convertible");
    let mut problem_alt = problem.clone();
    problem.execute_instructions(&instructions);
    problem_alt.execute_instructions_alt(&instructions);
    let output = problem.output();
    let output_alt = problem_alt.output();
    println!("The crates on top are {}", output);
    println!("The alternative crates on top are {}", output_alt);
}

fn get_instructions(instructions: &str) -> Vec<Instruction> {
    instructions
        .split('\n')
        .map(|x| x.parse::<Instruction>().unwrap())
        .collect()
}

fn split_instructions(text: &str) -> (&str, &str) {
    text.split_once("\n\n")
        .expect("Should be able to split problem from instructions")
}

struct Instruction {
    from: usize,
    to: usize,
    amount: usize,
}

impl FromStr for Instruction {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<usize> = s
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();

        assert_eq!(numbers.len(), 3);

        // Problem is 1-indexed, here we are 0-indexed
        Ok(Instruction {
            from: numbers[1] - 1,
            to: numbers[2] - 1,
            amount: numbers[0],
        })
    }
}

#[derive(Debug, Clone)]
struct Problem {
    stacks: Vec<Vec<char>>,
}

impl FromStr for Problem {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let length = s
            .lines()
            .next()
            .expect("There should be at least one problem line")
            .len();

        let num_stacks = (length + 1) / 4;
        let mut stacks: Vec<Vec<char>> = Vec::with_capacity(num_stacks);

        for _ in 0..stacks.capacity() {
            stacks.push(Vec::new());
        }

        s.lines().rev().skip(1).for_each(|s| {
            (0..num_stacks).for_each(|i| {
                let index = 4 * i + 1;
                let content = s
                    .chars()
                    .nth(index)
                    .expect("At this position should be a char");
                if content != ' ' {
                    stacks[i].push(content);
                }
            });
        });
        Ok(Problem { stacks })
    }
}

impl Problem {
    fn execute_instruction(&mut self, instruction: &Instruction) {
        let iters = instruction.amount;
        let from = instruction.from;
        let to = instruction.to;

        for _ in 0..iters {
            let val = self.stacks[from].pop().expect("This should be possible");
            self.stacks[to].push(val);
        }
    }

    fn execute_instruction_alt(&mut self, instruction: &Instruction) {
        let amount = instruction.amount;
        let from = instruction.from;
        let to = instruction.to;

        let end_index = self.stacks[from].len() - amount;

        let mut items = self.stacks[from].drain(end_index..).collect();
        self.stacks[to].append(&mut items);
    }

    fn execute_instructions(&mut self, instructions: &Vec<Instruction>) {
        for instruction in instructions {
            self.execute_instruction(instruction);
        }
    }

    fn execute_instructions_alt(&mut self, instructions: &Vec<Instruction>) {
        for instruction in instructions {
            self.execute_instruction_alt(instruction);
        }
    }

    fn output(&self) -> String {
        let mut output = String::new();
        for vec in &self.stacks {
            let content = vec[vec.len() - 1];
            output.push(content);
        }
        output
    }
}
