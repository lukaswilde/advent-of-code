use std::{cmp::max, error::Error, fmt::Display, str::FromStr};

use utils::parse_text;

// These constants determine the range of the "infinite floor"
// such that we can still have a readable printing of the Map
// These values worked for my puzzle, but you could have the need to adjust them for other problems
// Remember that sand spawns at height 0, so we do not need a constant MIN_Y.
// Per instructions, the MAX_Y is 2 + the maxium y value of any rock formation.
const MIN_X: usize = 300;
const MAX_X_ADDER: usize = 100;

fn main() {
    let text = parse_text();
    let shapes = parse_shapes(&text);

    let mut map = Map::new(&shapes);
    let mut map2 = map.clone();
    println!("Resulting Map: \n{}", map);

    let rests = get_number_rests(&mut map, false);
    let stop_rests = get_number_rests(&mut map2, true);

    println!("The number of rested sand is {}", rests);
    println!("The number of rests needed for stopping is {}", stop_rests);
}

fn parse_shapes(text: &str) -> Vec<Shape> {
    text.lines()
        .map(|l| l.parse::<Shape>().expect("Conversion should be possible"))
        .collect()
}

fn get_number_rests(map: &mut Map, part2: bool) -> usize {
    let mut counter = 0;
    while map.spawn_and_execute(part2) {
        counter += 1;
        // println!("The map after {} sands is\n{}", counter, map);
    }
    counter
}

#[derive(Debug)]
struct Point(usize, usize);

struct Shape {
    moves: Vec<Point>,
}

impl FromStr for Shape {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s
            .replace(' ', "")
            .split("->")
            .map(|p| {
                let coords = p
                    .split_once(',')
                    .expect("Point Coords should be separated by ,");
                Point(
                    coords.0.parse().expect("Should be integer"),
                    coords.1.parse().expect("Should be integer"),
                )
            })
            .collect();

        Ok(Shape { moves })
    }
}

#[derive(Clone)]
struct Map {
    height: usize,
    width: usize,
    grid: Vec<Vec<char>>,
}

impl Map {
    fn new(shapes: &[Shape]) -> Self {
        let (max_x, mut max_y) = shapes
            .iter()
            .flat_map(|x| x.moves.iter())
            .fold((usize::MIN, usize::MIN), |(max_x, max_y), cur| {
                (max(max_x, cur.0), max(max_y, cur.1))
            });

        // As part of part 2, assume double the width from [0, max_x]
        let max_x = MAX_X_ADDER + max_x;
        // Allow for rim so sand can fall off
        max_y += 1;

        let width = max_x - MIN_X + 1;
        // Given by Part 2
        let height = max_y + 2;
        let mut grid = vec![vec!['.'; height]; width];

        for shape in shapes.iter() {
            for (p1, p2) in shape.moves.iter().zip(shape.moves.iter().skip(1)) {
                let (x1, y1) = (p1.0 - MIN_X, p1.1);
                let (x2, y2) = (p2.0 - MIN_X, p2.1);
                if x1 == x2 {
                    (y1.min(y2)..=y1.max(y2)).for_each(|y| {
                        grid[x1][y] = '#';
                    });
                } else {
                    (x1.min(x2)..=x1.max(x2)).for_each(|x| {
                        grid[x][y1] = '#';
                    });
                }
            }
        }
        Map {
            height,
            width,
            grid,
        }
    }

    fn get(&self, x: usize, y: usize) -> char {
        let x_norm = x - MIN_X;
        self.grid[x_norm][y]
    }

    fn set(&mut self, x: usize, y: usize) {
        let x_norm = x - MIN_X;
        self.grid[x_norm][y] = 'o';
    }

    fn is_clear(&self, x: usize, y: usize) -> bool {
        self.get(x, y) == '.'
    }

    fn drop_sand(&self, sand_pos: &Point, part2: bool) -> Option<Point> {
        if part2 && sand_pos.1 == self.height - 2 {
            return None;
        }
        if sand_pos.0 < MIN_X
            || sand_pos.0 >= MIN_X + self.width - 1
            || sand_pos.1 >= self.height - 1
        {
            return None;
        }

        let down = Point(sand_pos.0, sand_pos.1 + 1);
        let down_left = Point(sand_pos.0 - 1, sand_pos.1 + 1);
        let down_right = Point(sand_pos.0 + 1, sand_pos.1 + 1);

        if self.is_clear(down.0, down.1) {
            Some(down)
        } else if self.is_clear(down_left.0, down_left.1) {
            Some(down_left)
        } else if self.is_clear(down_right.0, down_right.1) {
            Some(down_right)
        } else {
            None
        }
    }

    fn spawn_and_execute(&mut self, part2: bool) -> bool {
        let mut sand_pos = Point(500, 0);
        while let Some(p) = self.drop_sand(&sand_pos, part2) {
            sand_pos = p;
        }
        if !part2 && sand_pos.1 == self.height - 1 {
            return false;
        }
        if sand_pos.0 == 500 && sand_pos.1 == 0 && !self.is_clear(sand_pos.0, sand_pos.1) {
            return false;
        }
        self.set(sand_pos.0, sand_pos.1);
        true
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for h in 0..self.height {
            for w in 0..self.width {
                let c = self.grid[w][h];
                s.push(c);
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}
