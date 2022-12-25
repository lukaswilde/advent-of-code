use std::{cmp::Reverse, collections::HashSet, error::Error, fmt::Display, str::FromStr};

use priority_queue::PriorityQueue;
use utils::parse_text;

fn main() {
    let text = parse_text();
    let mut map = text.parse::<Map>().expect("Should be convertible");

    println!("The resulting map is\n{}", map);
    let first_way = astar(&mut map);
    let second_way = astar(&mut map);
    let third_way = astar(&mut map);
    println!("The shortest path takes {} minutes", first_way);
    println!(
        "Going back and reaching the goal again takes {} minutes",
        first_way + second_way + third_way
    );
}

fn evolve(blizzards: &[Blizzard], map: &Map) -> Vec<Blizzard> {
    blizzards
        .iter()
        .map(|b| match b.facing {
            Direction::Left => {
                let mut new_x = b.position.0 - 1;
                if new_x == 0 {
                    new_x = map.width - 2;
                }
                Blizzard {
                    position: Point(new_x, b.position.1),
                    facing: b.facing.clone(),
                }
            }
            Direction::Right => {
                let mut new_x = b.position.0 + 1;
                if new_x == map.width - 1 {
                    new_x = 1;
                }
                Blizzard {
                    position: Point(new_x, b.position.1),
                    facing: b.facing.clone(),
                }
            }
            Direction::Up => {
                let mut new_y = b.position.1 - 1;
                if new_y == 0 {
                    new_y = map.height - 2;
                }
                Blizzard {
                    position: Point(b.position.0, new_y),
                    facing: b.facing.clone(),
                }
            }
            Direction::Down => {
                let mut new_y = b.position.1 + 1;
                if new_y == map.height - 1 {
                    new_y = 1;
                }
                Blizzard {
                    position: Point(b.position.0, new_y),
                    facing: b.facing.clone(),
                }
            }
        })
        .collect()
}

fn astar(map: &mut Map) -> usize {
    let mut closed_list = HashSet::new();
    let mut open_list: PriorityQueue<State, Reverse<usize>> = PriorityQueue::new();
    let start_state = State {
        g_cost: 0,
        h_cost: map.start.manhattan_dist(&map.end),
        position: map.start.clone(),
        blizzards: map.blizzards.clone(),
    };
    let f_value = start_state.combine_cost();
    open_list.push(start_state, Reverse(f_value));

    while let Some(next) = open_list.pop() {
        if next.0.position == map.end {
            map.blizzards = next.0.blizzards.clone();
            (map.start, map.end) = (map.end.clone(), map.start.clone());
            return next.0.g_cost;
        }
        closed_list.insert(ClosedIdentifier {
            position: next.0.position.clone(),
            blizzards: next.0.blizzards.clone(),
        });
        let next_blizzards = evolve(&next.0.blizzards, map);
        let successors: Vec<Point> = ['u', 'd', 'l', 'r', 'w']
            .iter()
            .filter_map(|&c| next.0.position.move_direction(c, map, &next_blizzards))
            .filter(|p| {
                !closed_list.contains(&ClosedIdentifier {
                    position: p.clone(),
                    blizzards: next_blizzards.clone(),
                })
            })
            .collect();

        for successor in successors {
            // map.clear();
            // map.set_blizzards(&next_blizzards);
            // map.set_point(successor.0, successor.1);
            // println!("At position {:?}, minute {}", successor, next.0.g_cost + 1);
            // println!("The map is\n{}", map);
            let succ_state = State {
                g_cost: next.0.g_cost + 1,
                h_cost: successor.manhattan_dist(&map.end),
                position: successor,
                blizzards: next_blizzards.clone(),
            };
            let f_value = Reverse(succ_state.combine_cost());

            let duplicate = open_list
                .iter_mut()
                .find(|(s, _)| s.position == succ_state.position);

            if let Some(dup) = duplicate {
                if dup.0.g_cost > succ_state.g_cost {
                    *dup.0 = succ_state;
                    *dup.1 = f_value;
                }
            } else {
                open_list.push(succ_state, f_value);
            }
        }
    }
    // No goal state found
    usize::MAX
}

#[derive(Eq, PartialEq, Hash)]
struct ClosedIdentifier {
    position: Point,
    blizzards: Vec<Blizzard>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct State {
    g_cost: usize,
    h_cost: usize,
    position: Point,
    blizzards: Vec<Blizzard>,
}

impl State {
    fn combine_cost(&self) -> usize {
        self.g_cost + self.h_cost
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point(usize, usize);

impl Point {
    fn manhattan_dist(&self, other: &Point) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }

    fn move_direction(&self, direction: char, map: &Map, blizzards: &[Blizzard]) -> Option<Point> {
        let d_vec = match direction {
            'u' => (0, -1),
            'd' => (0, 1),
            'r' => (1, 0),
            'l' => (-1, 0),
            'w' => (0, 0),
            _ => unreachable!(),
        };
        let blizzard_map: HashSet<Point> =
            HashSet::from_iter(blizzards.iter().map(|b| b.position.clone()));
        let result = (self.0 as isize + d_vec.0, self.1 as isize + d_vec.1);
        (result.0 >= 0
            && result.0 < map.width as isize
            && result.1 >= 0
            && result.1 < map.height as isize
            && map.grid[result.0 as usize][result.1 as usize] != '#'
            && !blizzard_map.contains(&Point(result.0 as usize, result.1 as usize)))
        .then_some(Point(result.0 as usize, result.1 as usize))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Blizzard {
    position: Point,
    facing: Direction,
}

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<char>>,
    blizzards: Vec<Blizzard>,
    start: Point,
    end: Point,
    width: usize,
    height: usize,
}

impl FromStr for Map {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blizzards = vec![];
        let width = s.lines().next().unwrap().chars().count();
        let height = s.lines().count();
        let mut grid = vec![vec!['.'; height]; width];

        s.lines().enumerate().for_each(|(i, l)| {
            l.chars()
                .collect::<Vec<char>>()
                .into_iter()
                .enumerate()
                .for_each(|(j, c)| {
                    match c {
                        '>' => blizzards.push(Blizzard {
                            position: Point(j, i),
                            facing: Direction::Right,
                        }),
                        '<' => blizzards.push(Blizzard {
                            position: Point(j, i),
                            facing: Direction::Left,
                        }),
                        '^' => blizzards.push(Blizzard {
                            position: Point(j, i),
                            facing: Direction::Up,
                        }),
                        'v' => blizzards.push(Blizzard {
                            position: Point(j, i),
                            facing: Direction::Down,
                        }),
                        _ => (),
                    };
                    grid[j][i] = c;
                })
        });

        let start_x = s
            .lines()
            .next()
            .expect("First line should expect")
            .chars()
            .position(|c| c == '.')
            .expect("Start point should exist in first line");
        let end_x = s
            .lines()
            .last()
            .expect("Last line should expect")
            .chars()
            .position(|c| c == '.')
            .expect("End point should exist in last line");
        let end_y = s.lines().count() - 1;
        let start = Point(start_x, 0);
        let end = Point(end_x, end_y);

        Ok(Map {
            grid,
            blizzards,
            start,
            end,
            width,
            height,
        })
    }
}

// Functions for easier debugging, by drawing the current map state during search
impl Map {
    #[allow(dead_code)]
    fn clear(&mut self) {
        for h in 1..self.height - 1 {
            for w in 1..self.width - 1 {
                self.grid[w][h] = '.';
            }
        }
        self.grid[self.start.0][self.start.1] = '.';
        self.grid[self.end.1][self.end.1] = '.';
    }

    #[allow(dead_code)]
    fn set_point(&mut self, x: usize, y: usize) {
        self.grid[x][y] = 'E';
    }

    #[allow(dead_code)]
    fn set_blizzards(&mut self, blizzards: &[Blizzard]) {
        println!(
            "Width of grid: {}, height of grid: {}",
            self.grid.len(),
            self.grid[0].len()
        );
        blizzards.iter().for_each(|b| {
            let content = self.grid[b.position.0][b.position.1];
            match content {
                '.' => {
                    let new_content = match b.facing {
                        Direction::Left => '<',
                        Direction::Right => '>',
                        Direction::Up => '^',
                        Direction::Down => 'v',
                    };
                    self.grid[b.position.0][b.position.1] = new_content;
                }
                '>' | '<' | 'v' | '^' => self.grid[b.position.0][b.position.1] = '2',
                x if x.is_ascii_digit() => {
                    println!("x is {}", x);
                    self.grid[b.position.0][b.position.1] =
                        char::from_digit(x.to_digit(10).unwrap() + 1, 10).unwrap()
                }
                _ => unreachable!(),
            }
        })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for h in 0..self.height {
            for w in 0..self.width {
                s.push(self.grid[w][h]);
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}
