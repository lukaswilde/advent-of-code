use std::collections::HashSet;

use utils::parse_text;

fn main() {
    let text = parse_text();
    let text = text.replace(' ', "");
    let items = get_items(&text);
    let group_badges = get_common_symbol(&text);
    let sum = sum_priorities(&items);
    let badge_sum = sum_priorities(&group_badges);
    println!("The sum of priorities is {}", sum);
    println!("The sum of badge priorities is {}", badge_sum);
}

fn determine_item(line: &str) -> char {
    let mid = line.len() / 2;
    let (left, right) = line.split_at(mid);
    let left_set: HashSet<char> = HashSet::from_iter(left.chars());
    let right_set: HashSet<char> = HashSet::from_iter(right.chars());

    let intersection: Vec<&char> = left_set.intersection(&right_set).collect();
    assert_eq!(intersection.len(), 1);
    *intersection[0]
}

fn get_items(text: &str) -> Vec<char> {
    text.split('\n').map(determine_item).collect()
}

fn intersect_sets(x: &[&str]) -> char {
    let first: HashSet<char> = HashSet::from_iter(x[0].chars());
    let second: HashSet<char> = HashSet::from_iter(x[1].chars());
    let third: HashSet<char> = HashSet::from_iter(x[2].chars());
    let common_char: HashSet<char> = first.intersection(&second).cloned().collect();
    let common_char: Vec<&char> = common_char.intersection(&third).collect();
    assert_eq!(common_char.len(), 1);
    *common_char[0]
}

fn get_common_symbol(text: &str) -> Vec<char> {
    text.split('\n')
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(intersect_sets)
        .collect()
}

fn calculate_priority(item: char) -> u32 {
    if item.is_lowercase() {
        (item as u32) - ('a' as u32) + 1
    } else {
        (item as u32) - ('A' as u32) + 27
    }
}

fn sum_priorities(items: &[char]) -> u32 {
    items.iter().map(|&c| calculate_priority(c)).sum()
}
