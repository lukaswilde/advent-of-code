use std::collections::VecDeque;

use utils::parse_text;

const DECRYPTION_KEY: isize = 811589153;

fn main() {
    let text = parse_text();
    let mut numbers = parse_numbers(&text);
    let mut numbers_alt: VecDeque<(usize, isize)> = numbers
        .iter()
        .map(|(i, x)| (*i, *x * DECRYPTION_KEY))
        .collect();
    mix(&mut numbers);
    mix_alt(&mut numbers_alt);
    let coords = get_grove_coords(&numbers);
    let coords_alt = get_grove_coords(&numbers_alt);
    println!("The grove coordinates are {}", coords);
    println!(
        "The grove coordinates using the decryption key are {}",
        coords_alt
    );
}

fn parse_numbers(text: &str) -> VecDeque<(usize, isize)> {
    text.lines()
        .enumerate()
        .map(|(i, line)| (i, line.parse().expect("Should be integer")))
        .collect()
}

fn mix_item(item: usize, numbers: &mut VecDeque<(usize, isize)>) {
    while numbers.front().unwrap().0 != item {
        let popped = numbers.pop_front().unwrap();
        numbers.push_back(popped);
    }
    let item_entry = numbers.pop_front().unwrap();
    let num_away = item_entry.1.rem_euclid(numbers.len() as isize);
    (0..num_away).for_each(|_| {
        let popped = numbers.pop_front().unwrap();
        numbers.push_back(popped);
    });
    numbers.push_back(item_entry);
}

fn mix(numbers: &mut VecDeque<(usize, isize)>) {
    (0..numbers.len()).for_each(|i| mix_item(i, numbers));
}

fn mix_alt(numbers: &mut VecDeque<(usize, isize)>) {
    (0..10).for_each(|_| mix(numbers));
}

fn get_grove_coords(numbers: &VecDeque<(usize, isize)>) -> isize {
    let zero_idx = numbers
        .iter()
        .enumerate()
        .find(|(_, (_, x))| *x == 0)
        .expect("Should exist")
        .0;
    let first = (zero_idx + 1000) % numbers.len();
    let second = (zero_idx + 2000) % numbers.len();
    let third = (zero_idx + 3000) % numbers.len();

    numbers[first].1 + numbers[second].1 + numbers[third].1
}
