use std::fs;

fn main() {
    let text = parse_text();
    let blocks = split_in_blocks(&text);
    let numbers_per_block = parse_numbers_in_block(blocks);
    let most_cals = get_most_calories(&numbers_per_block);
    let top_three = get_top_three_calories(&numbers_per_block);
    println!("Most calories that an elve is carrying: {}", most_cals);
    println!("Calories by top three elves are: {}", top_three);
}

fn parse_text() -> String {
    match std::env::args().len() {
        2 => std::env::args()
            .nth(1)
            .expect("If there is only one argument, it should be the problem text"),
        3 => {
            assert!(std::env::args().nth(1).unwrap() == "-i");
            let filename = std::env::args()
                .nth(2)
                .expect("There should be a file as argument");

            fs::read_to_string(filename).expect("The file should exist")
        }
        _ => panic!("Either we have one argument (the problem text) or 2 (where it is -i file)"),
    }
}

fn split_in_blocks(text: &str) -> Vec<String> {
    text.replace(' ', "")
        .split("\n\n")
        .map(String::from)
        .collect()
}

fn parse_numbers_in_block(text: Vec<String>) -> Vec<Vec<u32>> {
    text.iter()
        .map(|s| s.split('\n').map(parse_number).collect())
        .collect()
}

fn parse_number(number: &str) -> u32 {
    number
        .parse::<u32>()
        .unwrap_or_else(|_| panic!("Should have been a number, but was {}", number))
}

fn get_most_calories(numbers_per_block: &[Vec<u32>]) -> u32 {
    numbers_per_block
        .iter()
        .map(|vec| vec.iter().sum())
        .max()
        .expect("Calory list should not be empty")
}

fn get_top_three_calories(numbers_per_block: &[Vec<u32>]) -> u32 {
    let mut calories: Vec<u32> = numbers_per_block
        .iter()
        .map(|vec| vec.iter().sum())
        .collect();

    calories.sort();
    calories.into_iter().rev().take(3).sum()
}
