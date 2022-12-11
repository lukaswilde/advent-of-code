use utils::parse_text;

fn main() {
    let text = parse_text();
    let mut monkeys = create_monkeys(&text);
    let mut monkeys_alt = create_monkeys(&text);

    (0..20).for_each(|_| execute_turn(&mut monkeys, true));
    (0..10000).for_each(|_| execute_turn(&mut monkeys_alt, false));

    let business = get_monkey_business(monkeys);
    let business_alt = get_monkey_business(monkeys_alt);

    println!("The monkey business after 20 rounds is {}", business);
    println!("The monkey business after 10000 rounds is {}", business_alt);
}

fn create_monkeys(text: &str) -> Vec<Monkey> {
    // Get product of all numbers in the test statements
    let divis_prod = text
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .nth(3)
                .expect("Line with divisor should exist")
        })
        .flat_map(|l| l.split_whitespace().filter_map(|x| x.parse::<usize>().ok()))
        .product();

    text.split("\n\n")
        .map(|x| Monkey::new(x, divis_prod))
        .collect()
}

fn execute_turn(monkeys: &mut Vec<Monkey>, worry_decrease: bool) {
    for i in 0..monkeys.len() {
        let monkey = &mut monkeys[i];
        let mut new_locs: Vec<(usize, usize)> = vec![];

        for _ in 0..monkey.items.len() {
            let mut new_val =
                (monkey.op)(monkey.items.pop().expect("Should be able to remove this"));
            if worry_decrease {
                new_val /= 3;
            }
            let to_monkey = (monkey.test)(new_val);
            new_locs.push((to_monkey, new_val));
            monkey.num_inspections += 1;
        }
        for (to, val) in new_locs.iter().rev() {
            monkeys[*to].items.push(*val);
        }
    }
}

fn get_monkey_business(mut monkeys: Vec<Monkey>) -> usize {
    monkeys.sort_by(|m1, m2| m1.num_inspections.cmp(&m2.num_inspections));
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|x| x.num_inspections)
        .product()
}

struct Monkey {
    items: Vec<usize>,
    op: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> usize>,
    num_inspections: usize,
}

impl Monkey {
    fn new(s: &str, divis_prod: usize) -> Self {
        assert_eq!(s.lines().count(), 6);
        let item_line = s.lines().nth(1).expect("Line 2 should exist");

        let items = item_line
            .replace(',', "")
            .split_whitespace()
            .skip(2)
            .map(|x| x.parse::<usize>().expect("Should be integers"))
            .collect::<Vec<_>>();

        let test_lines: Vec<usize> = s
            .lines()
            .skip(3)
            .flat_map(|l| l.split_whitespace().filter_map(|x| x.parse::<usize>().ok()))
            .collect();
        assert_eq!(test_lines.len(), 3);
        let test_cond = test_lines[0];
        let if_case = test_lines[1];
        let else_case = test_lines[2];

        let test = Box::new(move |x: usize| {
            if x % test_cond == 0 {
                if_case
            } else {
                else_case
            }
        });

        let op_line = s.lines().nth(2).expect("Line 3 should exist");
        let op_args: Vec<&str> = op_line.split_whitespace().skip(4).collect();

        assert_eq!(op_args.len(), 2);
        let template_monkey = Monkey {
            items,
            test,
            op: Box::new(|x| x),
            num_inspections: 0,
        };
        // (x + a) [x * a] is divisible by t iff ((x % t) + a) [(x % t) * a] is divisible by t
        // Because items are shifted between monkeys with different test conditions, we
        // need to choose t as the product of all numbers in the test cases
        match (op_args[0], op_args[1]) {
            ("+", "old") => Monkey {
                op: Box::new(move |x| (x % divis_prod + x)),
                ..template_monkey
            },
            ("*", "old") => Monkey {
                op: Box::new(move |x| (x % divis_prod * x)),
                ..template_monkey
            },
            ("+", val) => {
                let val = val.parse::<usize>().unwrap();
                Monkey {
                    op: Box::new(move |x| (x % divis_prod + val)),
                    ..template_monkey
                }
            }
            ("*", val) => {
                let val = val.parse::<usize>().unwrap();
                Monkey {
                    op: Box::new(move |x| (x % divis_prod * val)),
                    ..template_monkey
                }
            }
            _ => panic!("Should only see + or *"),
        }
    }
}
