use std::{cmp::max, ops::ControlFlow, str::FromStr};

use utils::parse_text;

fn main() {
    let text = parse_text().trim().replace(' ', "");
    let grid = text
        .parse::<Grid>()
        .expect("Conversion from problem should work");
    let num_visible = grid.get_number_visible();
    let max_score = grid.max_scenic_score();
    println!("The number of visible trees is {}", num_visible);
    println!("The maximum scenic score is {}", max_score);
}

struct Grid {
    width: usize,
    height: usize,
    points: Vec<u8>,
}

impl FromStr for Grid {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points: Vec<u8> = Vec::new();
        s.lines().for_each(|l| {
            l.chars().for_each(|c| {
                points.push(c.to_digit(10).expect("Tree heights should be u8's") as u8)
            })
        });
        let width = s
            .lines()
            .next()
            .expect("There should be at least one line")
            .len();
        let height = s.lines().count();
        Ok(Grid {
            width,
            height,
            points,
        })
    }
}

impl Grid {
    fn is_visible(&self, x: usize, y: usize) -> bool {
        let pos = self.coords2pos(x, y);
        let own_value = self.points[pos];

        let trees = self.get_directional_trees(x, y);

        trees
            .iter()
            .map(|x| x.iter().all(|&h| h < own_value))
            .any(|cur| cur)
    }

    fn get_directional_trees(&self, x: usize, y: usize) -> [Vec<u8>; 4] {
        let pos = self.coords2pos(x, y);

        let left = self.points[y * self.width..pos].to_vec();
        let right = self.points[(pos + 1)..(y + 1) * self.width].to_vec();
        let vertical = self
            .points
            .iter()
            .enumerate()
            // (i + self.width -x) % self.width prevents underflow
            .filter(|(i, _)| (i + self.width - x) % self.width == 0)
            .map(|x| *x.1)
            .collect::<Vec<_>>();

        let split_point = pos / self.width;
        let top = vertical[..split_point].to_vec();
        let bottom = vertical[(split_point + 1)..].to_vec();

        [left, right, top, bottom]
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let pos = self.coords2pos(x, y);
        let own_value = self.points[pos];

        let mut trees = self.get_directional_trees(x, y);
        trees[0].reverse();
        trees[2].reverse();

        let folded = trees.iter().map(|x| {
            x.iter().try_fold(0, |acc, cur| {
                if cur < &own_value {
                    ControlFlow::Continue(acc + 1)
                } else {
                    ControlFlow::Break(acc + 1)
                }
            })
        });

        folded
            .map(|x| match x {
                ControlFlow::Continue(a) => a,
                ControlFlow::Break(a) => a,
            })
            .product()
    }

    fn coords2pos(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn max_scenic_score(&self) -> usize {
        let mut result = 0;
        for x in 1..(self.width - 1) {
            for y in 1..(self.height - 1) {
                result = max(result, self.scenic_score(x, y));
            }
        }
        result
    }

    fn get_number_visible(&self) -> usize {
        let mut result = 2 * (self.height + self.width) - 4;
        for x in 1..(self.width - 1) {
            for y in 1..(self.height - 1) {
                result += self.is_visible(x, y) as usize;
            }
        }
        result
    }
}
