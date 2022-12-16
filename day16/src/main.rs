use std::cmp::{max, min};

use utils::parse_text;

fn main() {
    let text = parse_text();
    // Ensures that valve AA is a line 0
    let text = sort_lines(&text);
    let replaced = replace_idents(text);

    let (flows, connections) = parse_flows_and_connections(&replaced);
    let dist = all_shortest_costs(&connections);
    let to_visit = get_relevant_valves(&flows);
    let simple_max_flow = find_max_flow(0, 30, &dist, &flows, to_visit.clone());
    let elephant_max_flow = find_alt_max_flow(&dist, &flows, to_visit);

    println!("The maximal achievable flow is {}", simple_max_flow);
    println!(
        "The maximal achievable flow with elephant is {}",
        elephant_max_flow
    );
}

fn sort_lines(text: &str) -> String {
    let mut text = text.lines().collect::<Vec<_>>();
    text.sort();
    text.join("\n")
}

fn find_max_flow(
    current: usize,
    time: usize,
    dist: &Vec<Vec<usize>>,
    flows: &Vec<usize>,
    to_visit: Vec<usize>,
) -> usize {
    let mut max_value = usize::MIN;

    for i in 0..to_visit.len() {
        let mut to_visit = to_visit.clone();
        let selected = to_visit.remove(i);

        if dist[current][selected] < time {
            let remaining_time = time - dist[current][selected] - 1;
            let new_val = flows[selected] * remaining_time
                + find_max_flow(selected, remaining_time, dist, flows, to_visit);
            max_value = max(max_value, new_val);
        }
    }
    max_value
}

// Approach for Part 2 inspired by https://github.com/pauldraper/advent-of-code-2022/blob/main/problems/day-16/part_2.py
// Look at all pairwise disjoint sets of valves that need to be visited
// and determine weather doing each one indpendent (you + the elephant) yields together a higher value as encountered before
fn find_alt_max_flow(dist: &Vec<Vec<usize>>, flows: &Vec<usize>, to_visit: Vec<usize>) -> usize {
    let mut best = usize::MIN;
    for partition in 0..1 << (to_visit.len() - 1) {
        let a = to_visit
            .iter()
            .enumerate()
            .filter_map(|(i, &n)| (partition & (1 << i) != 0).then_some(n))
            .collect();
        let b = to_visit
            .iter()
            .enumerate()
            .filter_map(|(i, &n)| (partition & (1 << i) == 0).then_some(n))
            .collect();
        best = max(
            best,
            find_max_flow(0, 26, dist, flows, a) + find_max_flow(0, 26, dist, flows, b),
        );
    }
    best
}

fn get_relevant_valves(flows: &[usize]) -> Vec<usize> {
    flows
        .iter()
        .enumerate()
        .filter_map(|(i, &f)| (f > 0).then_some(i))
        .collect()
}

fn replace_idents(mut text: String) -> String {
    for i in 0..text.lines().count() {
        let line = text.lines().nth(i).expect("Should exist");
        let to_replace = line
            .split_whitespace()
            .nth(1)
            .expect("Should be the valve identifier");
        text = text.replace(to_replace, &i.to_string());
    }
    text
}

fn parse_flows_and_connections(text: &str) -> (Vec<usize>, Vec<Vec<usize>>) {
    let mut flows = vec![];
    let mut connections = vec![];

    text.lines().for_each(|l| {
        let second_part = l.split('=').nth(1).expect("There should be a =");
        let flow = second_part
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<usize>()
            .expect("Flow should be an integer");
        flows.push(flow);
        let second_part = second_part.replace(',', "");
        let conns = second_part
            .split_whitespace()
            .flat_map(|x| x.parse::<usize>())
            .collect::<Vec<_>>();
        connections.push(conns);
    });
    (flows, connections)
}

fn all_shortest_costs(connections: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = connections.len();
    let mut dist = vec![vec![usize::MAX; n]; n];

    for (i, vec) in connections.iter().enumerate() {
        for &j in vec {
            dist[i][j] = 1;
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = min(dist[i][j], dist[i][k].saturating_add(dist[k][j]));
            }
        }
    }
    dist
}
