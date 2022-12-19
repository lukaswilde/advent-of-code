use std::{collections::HashMap, error::Error, str::FromStr};

use utils::parse_text;

fn main() {
    let text = parse_text();
    let blueprints = create_blueprints(&text);
    let quality_level = get_quality_level(&blueprints);
    let product_of_largest = get_product_of_largest(&blueprints);
    println!("The quality level of the blueprints is {}", quality_level);
    println!(
        "The product of largest nummber of geodes opened is {}",
        product_of_largest
    );
}

fn get_product_of_largest(mut blueprints: &[Blueprint]) -> usize {
    let mut initial_state = State::default();
    initial_state.set_remaining_time(32);
    if blueprints.len() > 3 {
        blueprints = &blueprints[..3];
    }
    blueprints
        .iter()
        .map(|blueprint| {
            let mut seen = HashMap::new();
            get_max_geodes(initial_state, blueprint, &mut seen)
        })
        .product()
}

fn get_quality_level(blueprints: &[Blueprint]) -> usize {
    let initial_state = State::default();
    blueprints
        .iter()
        .map(|blueprint| {
            let mut seen = HashMap::new();
            get_max_geodes(initial_state, blueprint, &mut seen) * blueprint.id
        })
        .sum()
}

fn get_max_geodes(state: State, blueprint: &Blueprint, seen: &mut HashMap<State, usize>) -> usize {
    if state.time_remaining == 0 {
        seen.insert(state, state.get_num_geodes());
        return state.get_num_geodes();
    }
    let successors = state.generate_successors(blueprint);
    assert!(!successors.is_empty());
    successors
        .iter()
        .map(|&succ_state| {
            if seen.contains_key(&succ_state) {
                *seen.get(&succ_state).unwrap()
            } else {
                let max_geodes = get_max_geodes(succ_state, blueprint, seen);
                seen.insert(succ_state, max_geodes);
                max_geodes
            }
        })
        .max()
        .expect("Successors should never be empty")
}

fn create_blueprints(text: &str) -> Vec<Blueprint> {
    text.lines()
        .map(|line| {
            line.parse::<Blueprint>()
                .expect("Each line should be a blueprint")
        })
        .collect()
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_robot_ore_cost: usize,
    clay_robot_ore_cost: usize,
    obsidian_robot_ore_cost: usize,
    obsidian_robot_clay_cost: usize,
    geode_robot_ore_cost: usize,
    geode_robot_obsidian_cost: usize,
}

impl FromStr for Blueprint {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace(':', "");
        let mut numbers = s.split_whitespace().filter_map(|s| s.parse::<usize>().ok());

        Ok(Self {
            id: numbers.next().unwrap(),
            ore_robot_ore_cost: numbers.next().unwrap(),
            clay_robot_ore_cost: numbers.next().unwrap(),
            obsidian_robot_ore_cost: numbers.next().unwrap(),
            obsidian_robot_clay_cost: numbers.next().unwrap(),
            geode_robot_ore_cost: numbers.next().unwrap(),
            geode_robot_obsidian_cost: numbers.next().unwrap(),
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct State {
    time_remaining: usize,
    num_ore: usize,
    num_clay: usize,
    num_obsidian: usize,
    num_geode: usize,
    num_ore_robots: usize,
    num_clay_robots: usize,
    num_obsidian_robots: usize,
    num_geode_robots: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            time_remaining: 24,
            num_ore: 0,
            num_clay: 0,
            num_obsidian: 0,
            num_geode: 0,
            num_ore_robots: 1,
            num_clay_robots: 0,
            num_obsidian_robots: 0,
            num_geode_robots: 0,
        }
    }
}

impl State {
    fn set_remaining_time(&mut self, time: usize) {
        self.time_remaining = time;
    }

    fn can_build_ore_robot(&self, blueprint: &Blueprint) -> bool {
        let cost = blueprint.ore_robot_ore_cost;
        self.num_ore >= cost
    }

    fn can_build_clay_robot(&self, blueprint: &Blueprint) -> bool {
        let cost = blueprint.clay_robot_ore_cost;
        self.num_ore >= cost
    }

    fn can_build_obsidian_robot(&self, blueprint: &Blueprint) -> bool {
        let ore_cost = blueprint.obsidian_robot_ore_cost;
        let clay_cost = blueprint.obsidian_robot_clay_cost;
        self.num_ore >= ore_cost && self.num_clay >= clay_cost
    }

    fn can_build_geode_robot(&self, blueprint: &Blueprint) -> bool {
        let ore_cost = blueprint.geode_robot_ore_cost;
        let obsidian_cost = blueprint.geode_robot_obsidian_cost;
        self.num_ore >= ore_cost && self.num_obsidian >= obsidian_cost
    }

    fn build_ore_robot(&mut self, blueprint: &Blueprint) {
        assert!(self.can_build_ore_robot(blueprint));
        let cost = blueprint.ore_robot_ore_cost;
        self.num_ore -= cost;
    }

    fn build_clay_robot(&mut self, blueprint: &Blueprint) {
        assert!(self.can_build_clay_robot(blueprint));
        let cost = blueprint.clay_robot_ore_cost;
        self.num_ore -= cost;
    }

    fn build_obsidian_robot(&mut self, blueprint: &Blueprint) {
        assert!(self.can_build_obsidian_robot(blueprint));
        let ore_cost = blueprint.obsidian_robot_ore_cost;
        let clay_cost = blueprint.obsidian_robot_clay_cost;
        self.num_ore -= ore_cost;
        self.num_clay -= clay_cost;
    }

    fn build_geode_robot(&mut self, blueprint: &Blueprint) {
        assert!(self.can_build_geode_robot(blueprint));
        let ore_cost = blueprint.geode_robot_ore_cost;
        let obsidian_cost = blueprint.geode_robot_obsidian_cost;
        self.num_ore -= ore_cost;
        self.num_obsidian -= obsidian_cost;
    }

    fn generate_resources(&mut self) {
        self.time_remaining -= 1;
        self.num_ore += self.num_ore_robots;
        self.num_clay += self.num_clay_robots;
        self.num_obsidian += self.num_obsidian_robots;
        self.num_geode += self.num_geode_robots;
    }

    fn get_num_geodes(&self) -> usize {
        self.num_geode
    }

    fn simplify(&mut self, blueprint: &Blueprint) {
        let max_ore_cost = *[
            blueprint.ore_robot_ore_cost,
            blueprint.clay_robot_ore_cost,
            blueprint.obsidian_robot_ore_cost,
            blueprint.geode_robot_ore_cost,
        ]
        .iter()
        .max()
        .unwrap();

        // These reductions are valid because we only have one robot factory.
        // If we have more producing robots than any cost of that resource, reduce the number of robots to that cost
        if self.num_ore_robots >= max_ore_cost {
            self.num_ore_robots = max_ore_cost;
        }
        if self.num_clay_robots >= blueprint.obsidian_robot_clay_cost {
            self.num_clay_robots = blueprint.obsidian_robot_clay_cost;
        }
        if self.num_obsidian_robots >= blueprint.geode_robot_obsidian_cost {
            self.num_obsidian_robots = blueprint.geode_robot_obsidian_cost;
        }

        // If we have more resources than the difference in comsumption and production for the remaining time
        // reduce the number of resources to that difference
        let ore_difference =
            self.time_remaining * max_ore_cost - self.num_ore_robots * (self.time_remaining - 1);
        let clay_difference = self.time_remaining * blueprint.obsidian_robot_clay_cost
            - self.num_clay_robots * (self.time_remaining - 1);
        let obsidian_difference = self.time_remaining * blueprint.geode_robot_obsidian_cost
            - self.num_obsidian_robots * (self.time_remaining - 1);

        if self.num_ore >= ore_difference {
            self.num_ore = ore_difference;
        }
        if self.num_clay >= clay_difference {
            self.num_clay = clay_difference;
        }
        if self.num_obsidian >= obsidian_difference {
            self.num_obsidian = obsidian_difference;
        }
    }

    fn generate_successors(mut self, blueprint: &Blueprint) -> Vec<State> {
        let mut result = vec![];
        // Used to reduce search space
        self.simplify(blueprint);

        if self.can_build_geode_robot(blueprint) {
            let mut state = self;
            state.generate_resources();
            state.build_geode_robot(blueprint);
            state.num_geode_robots += 1;
            result.push(state);
        }
        if self.can_build_obsidian_robot(blueprint) {
            let mut state = self;
            state.generate_resources();
            state.build_obsidian_robot(blueprint);
            state.num_obsidian_robots += 1;
            result.push(state);
        }
        if self.can_build_clay_robot(blueprint) {
            let mut state = self;
            state.generate_resources();
            state.build_clay_robot(blueprint);
            state.num_clay_robots += 1;
            result.push(state);
        }
        if self.can_build_ore_robot(blueprint) {
            let mut state = self;
            state.generate_resources();
            state.build_ore_robot(blueprint);
            state.num_ore_robots += 1;
            result.push(state);
        }
        self.generate_resources();
        result.push(self);

        result
    }
}
