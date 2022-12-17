use std::{cmp::max, fmt::Display};

use utils::parse_text;

const NUM_ROUNDS: usize = 2022;

fn main() {
    let text = parse_text();
    let directions = parse_directions(&text);

    let mut map = Map::new(NUM_ROUNDS);
    let max_height = execute_drops(&mut map, &directions, NUM_ROUNDS) + 1;
    println!("The map is\n{}", map);
    println!("The maximum height is {}", max_height);
}

fn execute_drops(map: &mut Map, directions: &Vec<Direction>, rounds: usize) -> usize {
    let mut direction_idx = 0;
    let mut height = -1;
    for i in 0..rounds {
        let shape = select_shape(i);
        let (new_height, new_direction_idx) =
            spawn_and_drop(shape, directions, height, direction_idx, map);
        height = max(new_height, height);
        direction_idx = new_direction_idx;
    }
    height as usize
}

fn select_shape(idx: usize) -> Shape {
    match idx % 5 {
        0 => Shape::Horizontal,
        1 => Shape::Cross,
        2 => Shape::Angle,
        3 => Shape::Vertical,
        4 => Shape::Block,
        _ => unreachable!(),
    }
}

// Return (height, new_direction_idx)
fn spawn_and_drop(
    shape: Shape,
    directions: &Vec<Direction>,
    height: isize,
    direction_idx: usize,
    map: &mut Map,
) -> (isize, usize) {
    let piece = match shape {
        Shape::Horizontal => Rock {
            center: (2, height + 4),
            shape,
        },
        Shape::Cross => Rock {
            center: (3, height + 5),
            shape,
        },
        Shape::Angle => Rock {
            center: (4, height + 4),
            shape,
        },
        Shape::Vertical => Rock {
            center: (2, height + 4),
            shape,
        },
        Shape::Block => Rock {
            center: (2, height + 4),
            shape,
        },
    };
    let mut idx = direction_idx;
    let mut landed = false;
    let mut last_rock = piece;
    while !landed {
        let direction = directions[idx];
        idx = (idx + 1) % directions.len();
        let new_pos = map.move_piece(last_rock, direction);
        if let Some(new_piece) = map.drop_piece(new_pos) {
            last_rock = new_piece;
        } else {
            landed = true;
        }
        // println!("The map is\n{}", map);
    }
    (last_rock.max_height(), idx)
}

fn parse_directions(text: &str) -> Vec<Direction> {
    text.chars()
        .map(|c| match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => unreachable!(),
        })
        .collect()
}

#[derive(Copy, Clone, Debug)]
enum Shape {
    Horizontal, // Center is leftmost
    Cross,      // Center is center
    Angle,      // Center is corner
    Vertical,   // Center is lowest
    Block,      // Center is corner left down
}

#[derive(Copy, Clone, Debug)]
struct Rock {
    center: (isize, isize),
    shape: Shape,
}

impl Rock {
    fn get_positions(&self) -> Option<Vec<(isize, isize)>> {
        let (x, y) = self.center;
        let preliminary = match self.shape {
            Shape::Horizontal => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            Shape::Cross => vec![(x, y), (x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)],
            Shape::Angle => vec![(x, y), (x - 1, y), (x - 2, y), (x, y + 1), (x, y + 2)],
            Shape::Vertical => vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            Shape::Block => vec![(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)],
        };
        preliminary
            .iter()
            .all(|(x, y)| x >= &0 && x <= &6 && y >= &0)
            .then_some(preliminary)
    }
    fn max_height(&self) -> isize {
        let positions = self.get_positions().expect("This should be valid");
        positions
            .iter()
            .map(|(_, y)| *y)
            .max()
            .expect("Should have max element")
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
}

struct Map {
    grid: Vec<[char; 7]>,
    height: usize,
}

impl Map {
    fn new(num_rounds: usize) -> Self {
        // 3 is the maximum size of a rock
        let upper_bound = num_rounds * 4;
        let grid = vec![['.'; 7]; upper_bound];

        Self {
            grid,
            height: upper_bound,
        }
    }

    fn move_piece(&self, piece: Rock, direction: Direction) -> Rock {
        let (center_x, y) = piece.center;
        let x = match direction {
            Direction::Left => center_x - 1,
            Direction::Right => center_x + 1,
        };
        let new_piece = Rock {
            center: (x, y),
            ..piece
        };
        let positions = new_piece.get_positions();
        match positions {
            Some(pos) => {
                if self.check_positions(&pos) {
                    return new_piece;
                } else {
                    return piece;
                }
            }
            None => piece,
        }
    }

    fn check_positions(&self, positions: &[(isize, isize)]) -> bool {
        positions
            .iter()
            .all(|(x, y)| self.grid[*y as usize][*x as usize] == '.')
    }

    fn set_positions(&mut self, piece: Rock) {
        let positions = piece
            .get_positions()
            .expect("Setting a piece requries that the piece is valid");
        positions.iter().for_each(|(x, y)| {
            self.grid[*y as usize][*x as usize] = '#';
        })
    }

    fn drop_piece(&mut self, piece: Rock) -> Option<Rock> {
        let (x, center_y) = piece.center;
        let y = center_y - 1;
        let new_piece = Rock {
            center: (x, y),
            ..piece
        };
        let positions = new_piece.get_positions();
        if let Some(vec) = positions {
            if self.check_positions(&vec) {
                return Some(new_piece);
            } else {
                self.set_positions(piece);
                return None;
            }
        } else {
            self.set_positions(piece);
            return None;
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for h in (0..self.height).rev() {
            s.push_str(self.grid[h].iter().collect::<String>().as_str());
            s.push('\n');
        }
        writeln!(f, "{}", s)
    }
}
