use std::collections::HashSet;

use utils::parse_text;

fn main() {
    let text = parse_text();
    let packet_pos = find_packet_marker_pos(&text, 4);
    let message_pos = find_packet_marker_pos(&text, 14);
    println!("The first position after a packet marker is {}", packet_pos);
    println!(
        "The first position after a message marker is {}",
        message_pos
    );
}

fn is_unique(sequence: impl Iterator<Item = char>) -> bool {
    let mut already_seen: HashSet<char> = HashSet::new();
    for c in sequence {
        if already_seen.contains(&c) {
            return false;
        }
        already_seen.insert(c);
    }
    true
}

fn find_marker_pos(text: &str, num_distinct: usize) -> usize {
    text.chars()
        .collect::<Vec<char>>()
        .windows(num_distinct)
        .enumerate()
        .find(|(_, chunk)| is_unique(chunk.iter().cloned()))
        .expect("There should be a start marker")
        .0
        + num_distinct
}
