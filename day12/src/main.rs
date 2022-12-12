use std::{cmp::Reverse, collections::HashSet, error::Error, str::FromStr};

use priority_queue::PriorityQueue;
use utils::parse_text;

fn main() {
    let text = parse_text();
    let mut map = text.parse::<Map>().expect("Should be convertible");
    let possible_starts = map.get_possible_starts();

    let cheapest_path_cost = astar(&map);
    let shortest_path_cost = get_shortest_path_cost(possible_starts, &mut map);
    println!(
        "The cheapest path to the goal has cost {}",
        cheapest_path_cost
    );
    println!(
        "The shortest path from any start point to the goal has cost {}",
        shortest_path_cost
    );
}

fn get_shortest_path_cost(starts: Vec<Point>, map: &mut Map) -> usize {
    starts
        .iter()
        .map(|start| {
            map.start = start.clone();
            astar(map)
        })
        .min()
        .expect("There should be a minimal value")
}

fn astar(map: &Map) -> usize {
    let mut closed_list = HashSet::new();
    let mut open_list: PriorityQueue<State, Reverse<usize>> = PriorityQueue::new();
    let start_state = State {
        g_cost: 0,
        h_cost: map.start.manhattan_dist(&map.end),
        position: map.start.clone(),
    };
    let f_value = start_state.combine_cost();
    open_list.push(start_state, Reverse(f_value));

    while let Some(next) = open_list.pop() {
        if next.0.position == map.end {
            return next.0.g_cost;
        }
        closed_list.insert(next.0.position.clone());
        let successors: Vec<Point> = ['u', 'd', 'l', 'r']
            .iter()
            .filter_map(|&c| next.0.position.move_direction(c, map))
            .filter(|p| !closed_list.contains(p))
            .collect();

        for successor in successors {
            let succ_state = State {
                g_cost: next.0.g_cost + 1,
                h_cost: successor.manhattan_dist(&map.end),
                position: successor,
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

#[derive(Debug, Eq, PartialEq, Hash)]
struct State {
    g_cost: usize,
    h_cost: usize,
    position: Point,
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

    fn move_direction(&self, direction: char, map: &Map) -> Option<Point> {
        let d_vec = match direction {
            'u' => (-1, 0),
            'd' => (1, 0),
            'r' => (0, 1),
            'l' => (0, -1),
            _ => unreachable!(),
        };
        let result = (self.0 as isize + d_vec.0, self.1 as isize + d_vec.1);
        (result.0 >= 0
            && result.0 < map.height as isize
            && result.1 >= 0
            && result.1 < map.width as isize
            && map.grid[result.0 as usize][result.1 as usize]
                <= map.grid[self.0 as usize][self.1 as usize] + 1)
            .then_some(Point(result.0 as usize, result.1 as usize))
    }
}

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<usize>>,
    start: Point,
    end: Point,
    width: usize,
    height: usize,
}

impl Map {
    fn get_possible_starts(&self) -> Vec<Point> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(j, &val)| (val == 0).then_some(Point(i, j)))
                    .collect::<Vec<_>>()
            })
            .collect(
    }
}

impl FromStr for Map {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = Point(0, 0);
        let mut end = Point(0, 0);
        let grid = s
            .lines()
            .enumerate()
            .map(|(i, l)| {
                l.chars()
                    .collect::<Vec<char>>()
                    .into_iter()
                    .enumerate()
                    .map(|(j, c)| match c {
                        'S' => {
                            start = Point(i, j);
                            0
                        }
                        'E' => {
                            end = Point(i, j);
                            25
                        }
                        c => c as usize - 'a' as usize,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(Map {
            grid,
            start,
            end,
            width: s.lines().next().unwrap().chars().count(),
            height: s.lines().count(),
        })
    }
}
