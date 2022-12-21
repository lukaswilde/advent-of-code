use std::{collections::HashMap, error::Error, fmt::Display, str::FromStr};

use utils::parse_text;

fn main() {
    let text = parse_text();

    let monkeys = create_monkeys(&text);
    let monkey_table = create_monkey_table(&monkeys);
    let root_monkey = monkey_table.get("root").expect("Must exist");
    let root_res = root_monkey.evaluate(&monkey_table);
    let human_res = find_human_res(&monkey_table, root_monkey);

    println!("The root monkey will yell {}", root_res);
    println!("We need to yell {}", human_res);
}

fn find_human_res(lookup_table: &HashMap<String, Monkey>, root_monkey: &Monkey) -> i64 {
    // Construct new root monkey with Equals
    let root_left = root_monkey.left.as_ref().unwrap().clone();
    let root_right = root_monkey.right.as_ref().unwrap().clone();
    let new_root = Monkey {
        name: "root".to_string(),
        result: None,
        left: Some(root_left.clone()),
        right: Some(root_right.clone()),
        operator: Some(Operator::Equals),
    };

    // Check whether the left or right hand side of the root monkey is dependent on humn
    let template_human = Monkey {
        name: "humn".to_string(),
        result: Some(0.0),
        left: None,
        right: None,
        operator: None,
    };
    let comp_human = Monkey {
        result: Some(10000.0),
        ..template_human.clone()
    };
    let mut base_lookup = lookup_table.clone();
    base_lookup.insert("humn".to_string(), template_human.clone());
    let mut comp_lookup = lookup_table.clone();
    comp_lookup.insert("humn".to_string(), comp_human);

    let mut to_reach = lookup_table.get(&root_left).unwrap().clone();
    let mut variable = lookup_table.get(&root_right).unwrap().clone();

    if to_reach.evaluate(&base_lookup) != to_reach.evaluate(&comp_lookup) {
        (variable, to_reach) = (to_reach, variable);
    }
    assert!(variable.evaluate(&base_lookup) != variable.evaluate(&comp_lookup));
    assert!(to_reach.evaluate(&base_lookup) == to_reach.evaluate(&comp_lookup));

    let to_reach = to_reach.evaluate(&base_lookup);
    // We do not know how changing the value of humn changes the value of the variable monkey that depends on humn
    // Therefore, we change the way we reduce the search space in our binary search to the opposite in a consecutive run
    for i in [-1.0, 1.0] {
        let mut low = 0;
        let mut high = i64::MAX / 2;
        while low + 1 < high {
            let mid = (low + high) / 2;
            let human = Monkey {
                result: Some(mid as f64),
                ..template_human.clone()
            };
            base_lookup.insert("humn".to_string(), human);
            let result = variable.evaluate(&base_lookup);

            if result * i < to_reach * i {
                high = mid;
            } else if result * i > to_reach * i {
                low = mid;
            } else {
                assert!(new_root.evaluate(&base_lookup) == 1.0);
                return mid;
            }
        }
    }
    unreachable!();
}

fn create_monkey_table(monkeys: &[Monkey]) -> HashMap<String, Monkey> {
    let mut result = HashMap::new();
    monkeys.iter().for_each(|m| {
        result.insert(m.name.clone(), m.clone());
    });
    result
}

fn create_monkeys(text: &str) -> Vec<Monkey> {
    text.lines()
        .map(|line| line.parse().expect("Should be possible"))
        .collect()
}

#[derive(Debug)]
struct OperatorParseError(String);

impl Display for OperatorParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not parse Operator, found {}", self.0)
    }
}

impl Error for OperatorParseError {}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
}

impl FromStr for Operator {
    type Err = OperatorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Subtract),
            "*" => Ok(Operator::Multiply),
            "/" => Ok(Operator::Divide),
            x => Err(OperatorParseError(x.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    result: Option<f64>,
    left: Option<String>,
    right: Option<String>,
    operator: Option<Operator>,
}

impl FromStr for Monkey {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rest) = s.split_once(':').expect(": should exist");
        if let Ok(result) = rest.trim().parse::<f64>() {
            return Ok(Monkey {
                name: name.to_string(),
                result: Some(result),
                left: None,
                right: None,
                operator: None,
            });
        }
        let operation: Vec<&str> = rest.split_whitespace().collect();
        assert_eq!(operation.len(), 3);
        let left = operation[0];
        let right = operation[2];
        let operator = operation[1]
            .parse::<Operator>()
            .expect("Should be possible");
        Ok(Monkey {
            name: name.to_string(),
            result: None,
            left: Some(left.to_string()),
            right: Some(right.to_string()),
            operator: Some(operator),
        })
    }
}

impl Monkey {
    fn evaluate(&self, lookup_table: &HashMap<String, Monkey>) -> f64 {
        if let Some(res) = self.result {
            return res;
        }
        // Use the underlying invariant here:
        // If result == None, then left, right and operator are not None
        // Also, all monkeys are contained in the lookup table, therefore we can use unwrap()
        let left_monkey = lookup_table.get(self.left.as_ref().unwrap()).unwrap();
        let right_monkey = lookup_table.get(self.right.as_ref().unwrap()).unwrap();
        let operator = self.operator.as_ref().unwrap();

        let left_res = left_monkey.evaluate(lookup_table);
        let right_res = right_monkey.evaluate(lookup_table);

        match operator {
            Operator::Add => left_res + right_res,
            Operator::Subtract => left_res - right_res,
            Operator::Multiply => left_res * right_res,
            Operator::Divide => left_res / right_res,
            Operator::Equals => (left_res == right_res) as usize as f64,
        }
    }
}
