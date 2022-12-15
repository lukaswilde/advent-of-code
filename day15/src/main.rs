use std::collections::HashSet;
use std::{cmp::max, error::Error, str::FromStr};

use utils::parse_text;

const ROW_MAX: isize = 4_000_000;

fn main() {
    let mut row = 2_000_000;

    let text = parse_text();
    // Less than 20 sensors, we are in example territory
    if text.lines().count() < 20 {
        row = 10;
    }

    let sensors = parse_sensors(&text);

    // Part 1:
    let mut intervals = vec![];
    let mut blocked = HashSet::new();
    create_intervals(&sensors, &mut blocked, &mut intervals, row);
    intervals.sort();

    let mut qualified: Vec<Interval> = vec![];
    refine_intervals(&mut qualified, &mut intervals);
    let places = get_number_blocked(&mut qualified, &blocked);

    println!("The number of blocked places is {}", places);

    // Part 2:
    for cur_row in 0..=ROW_MAX {
        let mut intervals = vec![];
        let mut blocked = HashSet::new();
        create_intervals(&sensors, &mut blocked, &mut intervals, cur_row);
        intervals.sort();

        let mut qualified: Vec<Interval> = vec![];
        refine_intervals(&mut qualified, &mut intervals);
        if let Some(val) = check_score(&mut qualified, cur_row) {
            println!("The tuning frequency of the distress beacon is {}", val);
            return;
        }
    }
}

fn check_score(qualified: &mut [Interval], current_row: isize) -> Option<isize> {
    let mut x = 0;
    for interval in qualified.iter() {
        if x < interval.low {
            return Some(x * ROW_MAX + current_row);
        }
        x = max(x, interval.high + 1);
        if x > ROW_MAX {
            return None;
        }
    }
    None
}

fn get_number_blocked(qualified: &mut [Interval], blocked: &HashSet<isize>) -> usize {
    let mut result = HashSet::new();
    for interval in qualified.iter() {
        for x in interval.low..=interval.high {
            result.insert(x);
        }
    }
    result.difference(blocked).count()
}

fn refine_intervals(qualified: &mut Vec<Interval>, intervals: &mut [Interval]) {
    for interval in intervals.iter() {
        if qualified.is_empty() {
            qualified.push(*interval);
            continue;
        }

        let max_interval = qualified.last().unwrap();
        let (_max_lo, max_hi) = (max_interval.low, max_interval.high);

        if interval.low > max_hi + 1 {
            qualified.push(*interval);
        }

        let mut last = qualified.iter_mut().rev().next().unwrap();
        last.high = max(max_hi, interval.high);
    }
}

fn create_intervals(
    sensors: &[Sensor],
    blocked: &mut HashSet<isize>,
    intervals: &mut Vec<Interval>,
    row_count: isize,
) {
    for sensor in sensors.iter() {
        let overlap = sensor.nearest_beacon_dist - sensor.origin.1.abs_diff(row_count) as isize;
        if overlap < 0 {
            continue;
        }
        let low = sensor.origin.0 - overlap;
        let high = sensor.origin.0 + overlap;
        intervals.push(Interval { low, high });

        if sensor.beacon.1 == row_count {
            blocked.insert(sensor.beacon.0);
        }
    }
}

fn parse_sensors(text: &str) -> Vec<Sensor> {
    text.lines()
        .map(|l| l.parse::<Sensor>().expect("Conversion should be possible"))
        .collect()
}

#[derive(Debug)]
struct Point(isize, isize);

impl Point {
    fn manhattan_dist(&self, other: &Point) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Interval {
    low: isize,
    high: isize,
}

struct Sensor {
    origin: Point,
    beacon: Point,
    nearest_beacon_dist: isize,
}

impl FromStr for Sensor {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<_> = s
            .split('=')
            .skip(1)
            .map(|s| {
                s.chars()
                    .take_while(|&c| c == '-' || c.is_ascii_digit())
                    .collect::<String>()
            })
            .map(|s| s.parse::<isize>().expect("These should be numbers"))
            .collect();
        assert_eq!(coords.len(), 4);

        let (origin, beacon) = (Point(coords[0], coords[1]), Point(coords[2], coords[3]));
        let nearest_beacon_dist = origin.manhattan_dist(&beacon) as isize;

        Ok(Sensor {
            origin,
            beacon,
            nearest_beacon_dist,
        })
    }
}
