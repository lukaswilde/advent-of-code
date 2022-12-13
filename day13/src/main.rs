use std::{cmp::Ordering, error::Error, fmt::Display, str::FromStr};
use utils::parse_text;

fn main() {
    let text = parse_text();
    let pairs = create_pairs(&text);
    let lists = create_lists(&text);

    let sum_indices = sum_ordered_indices(pairs);
    let decoder_product = order_and_find(lists);
    println!("The sum of the indices of ordered pairs is {}", sum_indices);
    println!("The product of the decoder indices is {}", decoder_product);
}

fn order_and_find(mut lists: Vec<List>) -> usize {
    let two = parse_list("[2]");
    let six = parse_list("[6]");

    lists.sort();
    lists
        .iter()
        .enumerate()
        .filter_map(|(i, x)| {
            if x == &two || x == &six {
                Some(i + 1)
            } else {
                None
            }
        })
        .product()
}

fn sum_ordered_indices(pairs: Vec<Pair>) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter_map(|(i, l)| if l.0 <= l.1 { Some(i + 1) } else { None })
        .sum()
}

fn create_lists(text: &str) -> Vec<List> {
    let mut text = text.to_string();
    text.push_str("\n[[2]]\n[[6]]");
    text.replace("\n\n", "\n")
        .lines()
        .map(|l| parse_list(&l[1..l.len() - 1]))
        .collect()
}

fn create_pairs(text: &str) -> Vec<Pair> {
    text.split("\n\n")
        .flat_map(|block| block.lines().map(|x| parse_list(&x[1..x.len() - 1])))
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|x| Pair(x[0].clone(), x[1].clone()))
        .collect()
}

fn parse_list(text: &str) -> List {
    let mut items = vec![];

    let pass = text
        .chars()
        .fold((0, String::from("")), |(acc, new_text), cur| {
            if acc == 0 && cur == ',' {
                items.push(new_text.parse::<ListNode>().expect("should be possible"));
                (acc, String::from(""))
            } else if cur == '[' {
                (acc + 1, new_text + "[")
            } else if cur == ']' {
                (acc - 1, new_text + "]")
            } else {
                (acc, new_text + cur.to_string().as_str())
            }
        });

    items.push(pass.1.parse::<ListNode>().expect("Should be possible"));
    List { items }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct List {
    items: Vec<ListNode>,
}

#[derive(Debug)]
struct Pair(List, List);

#[derive(Debug, PartialEq, Eq, Clone)]
enum ListNode {
    Integer(usize),
    Empty,
    Body(Box<List>),
}

impl FromStr for ListNode {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = s.parse::<usize>() {
            Ok(ListNode::Integer(num))
        } else if s.is_empty() {
            Ok(ListNode::Empty)
        } else {
            let s = &s[1..s.len() - 1];
            Ok(ListNode::Body(Box::new(parse_list(s))))
        }
    }
}

impl Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListNode::Integer(a) => write!(f, "{}", a),
            ListNode::Empty => write!(f, ""),
            ListNode::Body(l) => write!(f, "[{}]", l),
        }
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        for (i, (ln0, ln1)) in self.items.iter().zip(other.items.iter()).enumerate() {
            let result = match (ln0, ln1) {
                (ListNode::Integer(a), ListNode::Integer(b)) => a.cmp(b),
                (ListNode::Body(a), ListNode::Body(b)) => a.cmp(b),
                (ListNode::Empty, ListNode::Empty) => Ordering::Equal,
                (_, ListNode::Empty) => Ordering::Greater,
                (ListNode::Empty, _) => Ordering::Less,
                (ListNode::Integer(a), _) => List {
                    items: vec![ListNode::Body(Box::new(List {
                        items: vec![ListNode::Integer(*a)],
                    }))],
                }
                .cmp(&List {
                    items: other.items[i..].to_vec(),
                }),
                (_, ListNode::Integer(a)) => List {
                    items: self.items[i..].to_vec(),
                }
                .cmp(&List {
                    items: vec![ListNode::Body(Box::new(List {
                        items: vec![ListNode::Integer(*a)],
                    }))],
                }),
            };
            if result != Ordering::Equal {
                return result;
            }
        }
        self.items.len().cmp(&other.items.len())
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("[");
        for (i, item) in self.items.iter().enumerate() {
            if i == self.items.len() - 1 {
                s = format!("{}{}", s, item);
            } else {
                s = format!("{},{}", s, item);
            }
        }
        write!(f, "{}]", s)
    }
}
