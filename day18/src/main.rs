use std::collections::{HashSet, VecDeque};

use utils::parse_text;

fn main() {
    let text = parse_text();
    let points = parse_points(&text);
    let points_set = create_hashset(&points);
    let total_sides = get_total_free_sides(&points, &points_set);

    let num_exposed = get_number_exposed(&points_set);
    println!("The number of free sides is {}", total_sides);
    println!("The number of exposed sides to water is {}", num_exposed);
}

fn parse_points(text: &str) -> Vec<(isize, isize, isize)> {
    text.lines()
        .map(|line| {
            let numbers: Vec<isize> = line
                .split(',')
                .map(|coord| coord.parse::<isize>().expect("Should be numbers"))
                .collect();
            assert_eq!(numbers.len(), 3);
            (numbers[0], numbers[1], numbers[2])
        })
        .collect()
}

fn create_hashset(points: &[(isize, isize, isize)]) -> HashSet<(isize, isize, isize)> {
    let mut points_set = HashSet::new();
    points.iter().for_each(|&p| {
        points_set.insert(p);
    });
    points_set
}

fn generate_neighbors(x: isize, y: isize, z: isize) -> [(isize, isize, isize); 6] {
    [
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ]
}

fn get_number_free_sides(
    point: (isize, isize, isize),
    points_set: &HashSet<(isize, isize, isize)>,
) -> usize {
    let neighbors = generate_neighbors(point.0, point.1, point.2);
    let blocked_sides: usize = neighbors
        .iter()
        .map(|p| points_set.contains(p) as usize)
        .sum();
    6 - blocked_sides
}

fn get_total_free_sides(
    points: &[(isize, isize, isize)],
    points_set: &HashSet<(isize, isize, isize)>,
) -> usize {
    points
        .iter()
        .map(|&p| get_number_free_sides(p, points_set))
        .sum()
}

// Approach for Part 2 inspired by https://github.com/jonathanpaulson/AdventOfCode/blob/master/2022/18.py
fn reaches_outside(
    point: (isize, isize, isize),
    point_set: &HashSet<(isize, isize, isize)>,
    outside: &mut HashSet<(isize, isize, isize)>,
    inside: &mut HashSet<(isize, isize, isize)>,
) -> bool {
    let mut seen = HashSet::new();
    if outside.contains(&point) {
        return true;
    }
    if inside.contains(&point) {
        return false;
    }

    let mut to_inspect = VecDeque::new();
    to_inspect.push_back(point);

    while !to_inspect.is_empty() {
        let new_point = to_inspect.pop_front().expect("Must exist");
        if point_set.contains(&new_point) || seen.contains(&new_point) {
            continue;
        }
        seen.insert(new_point);
        if seen.len() > 5000 {
            for point in seen.iter() {
                outside.insert(*point);
            }
            return true;
        }
        to_inspect.push_back((new_point.0 + 1, new_point.1, new_point.2));
        to_inspect.push_back((new_point.0 - 1, new_point.1, new_point.2));
        to_inspect.push_back((new_point.0, new_point.1 + 1, new_point.2));
        to_inspect.push_back((new_point.0, new_point.1 - 1, new_point.2));
        to_inspect.push_back((new_point.0, new_point.1, new_point.2 + 1));
        to_inspect.push_back((new_point.0, new_point.1, new_point.2 - 1));
    }
    for point in seen.iter() {
        inside.insert(*point);
    }
    false
}

fn get_number_exposed(points_set: &HashSet<(isize, isize, isize)>) -> usize {
    let mut result = 0;
    let mut outside = HashSet::new();
    let mut inside = HashSet::new();

    for point in points_set.iter() {
        if reaches_outside(
            (point.0 + 1, point.1, point.2),
            points_set,
            &mut outside,
            &mut inside,
        ) {
            result += 1;
        }
        if reaches_outside(
            (point.0 - 1, point.1, point.2),
            points_set,
            &mut outside,
            &mut inside,
        ) {
            result += 1;
        }
        if reaches_outside(
            (point.0, point.1 + 1, point.2),
            points_set,
            &mut outside,
            &mut inside,
        ) {
            result += 1;
        }
        if reaches_outside(
            (point.0, point.1 - 1, point.2),
            points_set,
            &mut outside,
            &mut inside,
        ) {
            result += 1;
        }
        if reaches_outside(
            (point.0, point.1, point.2 + 1),
            points_set,
            &mut outside,
            &mut inside,
        ) {
            result += 1;
        }
        if reaches_outside(
            (point.0, point.1, point.2 - 1),
            points_set,
            &mut outside,
            &mut inside,
        ) {
            result += 1;
        }
    }
    result
}
