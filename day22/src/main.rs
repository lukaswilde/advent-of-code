use std::{error::Error, fmt::Display, str::FromStr};

use utils::parse_text;

fn main() {
    let text = parse_text();
    let (map_text, instruction_text) = split_sections(&text);
    let instructions = parse_instructions(instruction_text);
    let mut map = map_text.parse::<Map>().expect("Should be convertible");
    println!("The map is\n{}", map);
    map.execute_instructions(&instructions);
    let password = map.get_final_password();

    println!("The final password is {}", password);
}

fn split_sections(text: &str) -> (&str, &str) {
    text.split_once("\n\n")
        .expect("These sections should exist")
}

fn parse_instructions(text: &str) -> Vec<Instruction> {
    let move_instructions = text
        .split(&['L', 'R'])
        .map(|c| Instruction::Move(c.parse::<usize>().expect("Should be integers")));
    let turn_instructions = text
        .chars()
        .filter_map(|c| (c == 'L' || c == 'R').then_some(Instruction::Rotate(c == 'R')));
    move_instructions
        .zip(turn_instructions)
        .flat_map(|(move_instruction, turn_instruction)| vec![move_instruction, turn_instruction])
        .collect()
}

enum Instruction {
    Move(usize),
    Rotate(bool),
}

enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    fn rotate(&self, clockwise: bool) -> Self {
        match self {
            Direction::Left => {
                if clockwise {
                    Self::Up
                } else {
                    Self::Down
                }
            }
            Direction::Right => {
                if clockwise {
                    Self::Down
                } else {
                    Self::Up
                }
            }
            Direction::Up => {
                if clockwise {
                    Self::Right
                } else {
                    Self::Left
                }
            }
            Direction::Down => {
                if clockwise {
                    Self::Left
                } else {
                    Self::Right
                }
            }
        }
    }
}

struct Map {
    grid: Vec<Vec<char>>,
    position: (usize, usize),
    facing: Direction,
    width: usize,
    height: usize,
}

impl Map {
    fn execute_instructions(&mut self, instructions: &[Instruction]) {
        instructions
            .iter()
            .for_each(|instruction| self.execute_instruction(instruction));
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        match *instruction {
            Instruction::Move(n) => {
                (0..n).for_each(|_| self.try_move());
            }
            Instruction::Rotate(clockwise) => self.facing = self.facing.rotate(clockwise),
        }
    }

    fn move_horizontally(&mut self, new_x: usize) {
        let content = self.grid[self.position.1][new_x];
        match content {
            '#' => (),
            '.' => self.position = (new_x, self.position.1),
            ' ' => {
                let wrap_idx = self.find_next();
                match self.grid[self.position.1][wrap_idx] {
                    '#' => (),
                    '.' => self.position = (wrap_idx, self.position.1),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }

    fn move_vertically(&mut self, new_y: usize) {
        let content = self.grid[new_y][self.position.0];
        match content {
            '#' => (),
            '.' => self.position = (self.position.0, new_y),
            ' ' => {
                let wrap_idx = self.find_next();
                match self.grid[wrap_idx][self.position.0] {
                    '#' => (),
                    '.' => self.position = (self.position.0, wrap_idx),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }

    fn try_move(&mut self) {
        match self.facing {
            Direction::Left => {
                let new_x = if self.position.0 == 0 {
                    self.find_next()
                } else {
                    self.position.0 - 1
                };
                self.move_horizontally(new_x);
            }
            Direction::Right => {
                let new_x = if self.position.0 == self.width - 1 {
                    self.find_next()
                } else {
                    self.position.0 + 1
                };
                self.move_horizontally(new_x)
            }
            Direction::Up => {
                let new_y = if self.position.1 == 0 {
                    self.find_next()
                } else {
                    self.position.1 - 1
                };
                self.move_vertically(new_y);
            }

            Direction::Down => {
                let new_y = if self.position.1 == self.height - 1 {
                    self.find_next()
                } else {
                    self.position.1 + 1
                };
                self.move_vertically(new_y);
            }
        }
    }

    fn find_next(&self) -> usize {
        match self.facing {
            Direction::Left => {
                let relevant_row = &self.grid[self.position.1];
                let rev_idx = relevant_row
                    .iter()
                    .rev()
                    .position(|c| *c != ' ')
                    .expect("Must exist");
                self.width - 1 - rev_idx
            }
            Direction::Right => {
                let relevant_row = &self.grid[self.position.1];
                let idx = relevant_row
                    .iter()
                    .position(|c| *c != ' ')
                    .expect("Must exist");
                idx
            }
            Direction::Up => {
                let relevant_col: Vec<char> =
                    self.grid.iter().map(|row| row[self.position.0]).collect();
                let rev_idx = relevant_col
                    .iter()
                    .rev()
                    .position(|c| *c != ' ')
                    .expect("Must exist");
                self.height - 1 - rev_idx
            }
            Direction::Down => {
                let relevant_col: Vec<char> =
                    self.grid.iter().map(|row| row[self.position.0]).collect();
                let idx = relevant_col
                    .iter()
                    .position(|c| *c != ' ')
                    .expect("Must exist");
                idx
            }
        }
    }

    fn get_final_password(self) -> usize {
        1000 * (self.position.1 + 1) + 4 * (self.position.0 + 1) + self.facing as usize
    }
}

impl FromStr for Map {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s
            .lines()
            .map(|l| l.len())
            .max()
            .expect("Should have maximal value");
        let height = s.lines().count();
        let mut grid = vec![vec![' '; width]; height];
        s.lines().enumerate().for_each(|(row, line)| {
            line.chars()
                .enumerate()
                .for_each(|(col, c)| grid[row][col] = c)
        });
        let position_x = s
            .lines()
            .next()
            .expect("Should have one line")
            .chars()
            .position(|c| c == '.')
            .expect("Should have a free position in the first row");
        let position = (position_x, 0);
        let facing = Direction::Right;

        Ok(Map {
            grid,
            position,
            facing,
            width,
            height,
        })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push(self.grid[y][x]);
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}
