use std::{
    cmp::{max, min},
    collections::{HashMap, VecDeque},
    error::Error,
    fmt::Display,
    str::FromStr,
};

use utils::parse_text;

// This indicates the padding around the original map to give room
// for the elves to spread out. In my puzzle, this number was sufficient, but it
// might have to be increased for other peoples puzzle.
const MAP_OFFSET: usize = 60;

fn main() {
    let text = parse_text();
    let mut map = text.parse::<Map>().expect("Should be convertible");
    let mut map_alt = map.clone();

    println!("The map is\n{}", map);
    map.execute_turns(Some(10));
    let empty_tiles = map.get_number_empty_tiles();
    let convergence = map_alt
        .execute_turns(None)
        .expect("Should exist in the second part");

    println!("The map after convergence is\n{}", map_alt);

    println!("The number of empty tiles is {}", empty_tiles);
    println!("The first round no elve moves is {}", convergence);
}

#[derive(Clone, Debug)]
enum Direction {
    East,
    South,
    West,
    North,
}

#[derive(Clone)]
struct Map {
    grid: Vec<Vec<char>>,
    elve_positions: Vec<(usize, usize)>,
    width: usize,
    height: usize,
}

impl Map {}

impl FromStr for Map {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s
            .lines()
            .map(|l| l.len())
            .max()
            .expect("Should have maximal value")
            + 2 * MAP_OFFSET;
        let height = s.lines().count() + 2 * MAP_OFFSET;
        let mut grid = vec![vec!['.'; height]; width];

        let mut elve_positions = Vec::new();
        s.lines().enumerate().for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, c)| {
                if c == '#' {
                    elve_positions.push((col + MAP_OFFSET, row + MAP_OFFSET));
                    grid[col + MAP_OFFSET][row + MAP_OFFSET] = c
                }
            })
        });

        Ok(Map {
            grid,
            elve_positions,
            width,
            height,
        })
    }
}
impl Map {
    fn execute_turns(&mut self, number_turns: Option<usize>) -> Option<usize> {
        let mut direction_priorities = VecDeque::from(vec![
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]);
        if let Some(number) = number_turns {
            (0..number).for_each(|_| {
                self.execute_turn(&direction_priorities);
                direction_priorities.rotate_left(1);
            });
            None
        } else {
            let mut i = 0;
            while !self.execute_turn(&direction_priorities) {
                direction_priorities.rotate_left(1);
                i += 1;
            }
            Some(i + 1)
        }
    }

    fn execute_turn(&mut self, direction_priorities: &VecDeque<Direction>) -> bool {
        // First half: Each elve makes a proposal for his next position
        let proposals = self
            .elve_positions
            .iter()
            .map(|(x, y)| {
                if self.is_free(*x, *y) {
                    (*x, *y)
                } else {
                    let mut directions = direction_priorities.clone();
                    while !directions.is_empty() {
                        let next_direction = directions.pop_front().expect("Should not be empty");
                        if let Some((new_x, new_y)) = self.try_move(*x, *y, next_direction) {
                            return (new_x, new_y);
                        }
                    }
                    (*x, *y)
                }
            })
            .collect::<Vec<_>>();

        // Second half: Move, if you were the only one to propose that tile
        let mut counter: HashMap<(usize, usize), usize> = HashMap::new();
        proposals.iter().for_each(|(x, y)| {
            counter.entry((*x, *y)).and_modify(|c| *c += 1).or_insert(1);
        });
        let duplicates = counter
            .into_iter()
            .filter(|(_, c)| *c >= 2)
            .collect::<HashMap<_, _>>();
        let updated_positions = proposals
            .into_iter()
            .zip(self.elve_positions.iter())
            .map(|(proposed, original)| {
                if duplicates.contains_key(&proposed) {
                    *original
                } else {
                    proposed
                }
            })
            .collect::<Vec<_>>();

        // No update: convergence!
        if updated_positions == self.elve_positions {
            return true;
        }
        self.clear_positions();
        self.set_positions(&updated_positions);
        self.elve_positions = updated_positions;

        false
    }

    fn clear_positions(&mut self) {
        self.elve_positions.iter().for_each(|(x, y)| {
            self.grid[*x][*y] = '.';
        })
    }

    fn set_positions(&mut self, positions: &[(usize, usize)]) {
        positions.iter().for_each(|(x, y)| {
            self.grid[*x][*y] = '#';
        })
    }

    fn try_move(&self, x: usize, y: usize, direction: Direction) -> Option<(usize, usize)> {
        let (first_look, second_look, third_look) = match direction {
            Direction::East => ((x + 1, y + 1), (x + 1, y), (x + 1, y - 1)),
            Direction::South => ((x - 1, y + 1), (x, y + 1), (x + 1, y + 1)),
            Direction::West => ((x - 1, y - 1), (x - 1, y), (x - 1, y + 1)),
            Direction::North => ((x - 1, y - 1), (x, y - 1), (x + 1, y - 1)),
        };

        if self.grid[first_look.0][first_look.1] != '#'
            && self.grid[second_look.0][second_look.1] != '#'
            && self.grid[third_look.0][third_look.1] != '#'
        {
            return Some(second_look);
        }
        None
    }

    fn is_free(&self, x: usize, y: usize) -> bool {
        self.grid[x + 1][y] != '#'
            && self.grid[x - 1][y] != '#'
            && self.grid[x][y + 1] != '#'
            && self.grid[x][y - 1] != '#'
            && self.grid[x + 1][y + 1] != '#'
            && self.grid[x + 1][y - 1] != '#'
            && self.grid[x - 1][y + 1] != '#'
            && self.grid[x - 1][y - 1] != '#'
    }

    fn get_number_empty_tiles(&self) -> usize {
        let (min_x, max_x, min_y, max_y) = self.elve_positions.iter().fold(
            (usize::MAX, 0, usize::MAX, 0),
            |(min_x, max_x, min_y, max_y), cur| {
                (
                    min(min_x, cur.0),
                    max(max_x, cur.0),
                    min(min_y, cur.1),
                    max(max_y, cur.1),
                )
            },
        );

        let truncated_grid = &self.grid[min_x..=max_x];
        truncated_grid
            .iter()
            .map(|v| v[min_y..=max_y].iter().filter(|&x| *x == '.').count())
            .sum()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push(self.grid[x][y]);
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}
