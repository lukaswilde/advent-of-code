use std::{error::Error, str::FromStr};

use utils::parse_text;

fn main() {
    let text = parse_text();
    let mut instructions = collect_instructions(&text);
    instructions.reverse();
    let mut clock = Clock::new(instructions.clone());
    clock.execute_instructions();
    let signal_strength = clock.get_signal_strength();
    let image = clock.get_image();
    println!("The signal strength is {}", signal_strength);
    println!("The final image is: \n\n{}", image);
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
enum Instruction {
    NoOp,
    AddX(isize),
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("noop") {
            Ok(Self::NoOp)
        } else {
            let args = s
                .split_once(' ')
                .expect("If not NoOp, splitting should be possible");
            assert_eq!(args.0, "addx");
            let number = args
                .1
                .parse::<isize>()
                .expect("Second argument should be integer");
            Ok(Self::AddX(number))
        }
    }
}

#[derive(Clone)]
struct Clock {
    instructions: Vec<Instruction>,
    register: isize,
    cycle: usize,
    buffer: isize,
    signal_strength: isize,
    img_str: String,
}

impl Clock {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            register: 1,
            cycle: 1,
            buffer: 0,
            signal_strength: 0,
            img_str: String::new(),
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        if let Instruction::AddX(a) = instruction {
            self.buffer = a;
        }
        self.cycle += 1;
    }

    fn check_cycle(&mut self) {
        let current_pos = (self.cycle - 1) as isize % 40;
        if self.register.abs_diff(current_pos) < 2 {
            self.img_str.push('#')
        } else {
            self.img_str.push('.');
        }
        if self.cycle % 40 == 0 {
            self.img_str.push('\n');
        }
        (20..=220).step_by(40).for_each(|i| {
            if self.cycle == i {
                self.signal_strength += self.register * (i as isize);
            }
        })
    }

    fn execute_instructions(&mut self) {
        while !self.instructions.is_empty() {
            self.check_cycle();
            if self.buffer != 0 {
                self.cycle += 1;
                self.register += self.buffer;
                self.buffer = 0;
            } else {
                let instruction = self.instructions.pop().expect("Should never be empty here");
                self.execute_instruction(instruction);
            }
        }
    }

    fn get_signal_strength(&self) -> isize {
        self.signal_strength
    }

    fn get_image(self) -> String {
        self.img_str
    }
}
