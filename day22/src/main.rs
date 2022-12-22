use std::{error::Error, fmt::Display, str::FromStr};

use utils::parse_text;

fn main() {
    let text = parse_text();
    let (map_text, instruction_text) = split_sections(&text);

    let instructions = parse_instructions(instruction_text);
    let mut map = map_text.parse::<Map>().expect("Should be convertible");
    let mut map_alt = map.clone();
    println!("The map is\n{}", map);

    map.execute_instructions(&instructions, false);

    let password = map.get_final_password();
    println!("The final password is {}", password);

    // Only execute the alternative strategy for the real puzzle input, as it is hardcoded for
    // Map of size 150 x 200
    if map_alt.width == 150 {
        map_alt.execute_instructions(&instructions, true);
        let password_alt = map_alt.get_final_password();
        println!("The final password when seen as cube is {}", password_alt);
    }
}

fn split_sections(text: &str) -> (&str, &str) {
    text.split_once("\n\n")
        .expect("These sections should exist")
}

fn parse_instructions(text: &str) -> Vec<Instruction> {
    let move_instructions = text
        .split(&['L', 'R'])
        .map(|c| Instruction::Move(c.parse::<usize>().expect("Should be integers")))
        .collect::<Vec<_>>();
    let turn_instructions = text
        .chars()
        .filter_map(|c| (c == 'L' || c == 'R').then_some(Instruction::Rotate(c == 'R')))
        .collect::<Vec<_>>();
    let mut instructions: Vec<Instruction> = move_instructions
        .iter()
        .zip(turn_instructions.iter())
        .flat_map(|(move_instruction, turn_instruction)| {
            vec![move_instruction.clone(), turn_instruction.clone()]
        })
        .collect();
    if move_instructions.len() > turn_instructions.len() {
        instructions.push(move_instructions.last().expect("Should exist").clone());
    }
    instructions
}

#[derive(Clone, Debug)]
enum Instruction {
    Move(usize),
    Rotate(bool),
}

#[derive(Clone, Debug)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl TryFrom<u8> for Direction {
    type Error = Box<dyn Error>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => panic!("Should not happen in this context"),
        })
    }
}

impl Direction {
    fn rotate(&self, clockwise: bool) -> Self {
        if clockwise {
            (self.clone() as u8 + 1)
                .rem_euclid(4)
                .try_into()
                .expect("Conversion should have worked")
        } else {
            // Plus 4 to prevent underflow
            (4 + self.clone() as u8 - 1)
                .rem_euclid(4)
                .try_into()
                .expect("Conversion should have worked")
        }
    }
}

#[derive(Clone)]
struct Map {
    grid: Vec<Vec<char>>,
    position: (usize, usize),
    facing: Direction,
    width: usize,
    height: usize,
}

impl Map {
    fn execute_instructions(&mut self, instructions: &[Instruction], part2: bool) {
        instructions
            .iter()
            .for_each(|instruction| self.execute_instruction(instruction, part2));
    }

    fn execute_instruction(&mut self, instruction: &Instruction, part2: bool) {
        match *instruction {
            Instruction::Move(n) => {
                (0..n).for_each(|_| {
                    if part2 {
                        self.try_move_alt();
                    } else {
                        self.try_move();
                    }
                });
            }
            Instruction::Rotate(clockwise) => self.facing = self.facing.rotate(clockwise),
        }
    }

    fn try_move_horizontally(&mut self, new_x: usize) {
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

    fn try_move_vertically(&mut self, new_y: usize) {
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

    // I assume this only works for my puzzle input...
    // But should also work if your puzzle input has the same shape as mine

    // My puzzle input has the following shape, where I identify the cube sides with IDs:
    //     11112222
    //     11112222
    //     11112222
    //     11112222
    //     3333
    //     3333
    //     3333
    //     3333
    // 44445555
    // 44445555
    // 44445555
    // 44445555
    // 6666
    // 6666
    // 6666
    // 6666
    fn try_move_alt(&mut self) {
        let (new_pos, new_facing) = match self.facing {
            Direction::Right => match self.position {
                // 2 -> 5
                (149, 0..=49) => ((99, 149 - self.position.1), Direction::Left),
                // 3 -> 2
                (99, 50..=99) => ((50 + self.position.1, 49), Direction::Up),
                // 5 -> 2
                (99, 100..=149) => ((149, 149 - self.position.1), Direction::Left),
                // 6 -> 5
                (49, 150..=199) => ((self.position.1 - 100, 149), Direction::Up),
                _ => ((self.position.0 + 1, self.position.1), Direction::Right),
            },
            Direction::Down => match self.position {
                // 2 -> 3
                (100..=149, 49) => ((99, self.position.0 - 50), Direction::Left),
                // 5 -> 6
                (50..=99, 149) => ((49, 100 + self.position.0), Direction::Left),
                // 6 -> 2
                (0..=49, 199) => ((100 + self.position.0, 0), Direction::Down),
                _ => ((self.position.0, self.position.1 + 1), Direction::Down),
            },
            Direction::Left => match self.position {
                // 1 -> 4
                (50, 0..=49) => ((0, 149 - self.position.1), Direction::Right),
                // 3 -> 4
                (50, 50..=99) => ((self.position.1 - 50, 100), Direction::Down),
                // 4 -> 1
                (0, 100..=149) => ((50, 149 - self.position.1), Direction::Right),
                // 6 -> 1
                (0, 150..=199) => ((self.position.1 - 100, 0), Direction::Down),
                _ => ((self.position.0 - 1, self.position.1), Direction::Left),
            },
            Direction::Up => match self.position {
                // 2 -> 6
                (100..=149, 0) => ((self.position.0 - 100, 199), Direction::Up),
                // 1 -> 6
                (50..=99, 0) => ((0, self.position.0 + 100), Direction::Right),
                // 4 -> 3
                (0..=49, 100) => ((50, 50 + self.position.0), Direction::Right),
                _ => ((self.position.0, self.position.1 - 1), Direction::Up),
            },
        };

        let content = self.grid[new_pos.1][new_pos.0];
        match content {
            '.' => {
                self.position = new_pos;
                self.facing = new_facing;
            }
            '#' => (),
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
                self.try_move_horizontally(new_x);
            }
            Direction::Right => {
                let new_x = if self.position.0 == self.width - 1 {
                    self.find_next()
                } else {
                    self.position.0 + 1
                };
                self.try_move_horizontally(new_x)
            }
            Direction::Up => {
                let new_y = if self.position.1 == 0 {
                    self.find_next()
                } else {
                    self.position.1 - 1
                };
                self.try_move_vertically(new_y);
            }

            Direction::Down => {
                let new_y = if self.position.1 == self.height - 1 {
                    self.find_next()
                } else {
                    self.position.1 + 1
                };
                self.try_move_vertically(new_y);
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
