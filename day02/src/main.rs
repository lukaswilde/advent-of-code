fn main() {
    let text = parse_text();
    let rounds = split_in_rounds(&text);
    let rounds_alt = split_in_rounds_alt(&text);
    let final_score = calculate_score(rounds);
    let final_score_alt = calculate_score(rounds_alt);
    println!("The final score is {}", final_score);
    println!("The final alternative score is {}", final_score_alt);
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

            std::fs::read_to_string(filename).expect("The file should exist")
        }
        _ => panic!("Either we have one argument (the problem text) or 2 (where it is -i file)"),
    }
}

fn split_in_rounds(text: &str) -> Vec<Round> {
    text.split('\n').into_iter().map(Round::new).collect()
}

fn split_in_rounds_alt(text: &str) -> Vec<Round> {
    text.split('\n').into_iter().map(Round::new_alt).collect()
}

fn calculate_score(rounds: Vec<Round>) -> u32 {
    rounds.into_iter().map(|r| r.outcome()).sum()
}

#[derive(Debug, PartialEq, Clone)]
enum Choice {
    Paper,
    Scissors,
    Rock,
}

impl Choice {
    fn new(encoding: &str) -> Self {
        match encoding {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Expected either A-C or X-Z, but got {}", encoding),
        }
    }

    fn score(&self) -> u32 {
        match *self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn winning_against(&self) -> Self {
        match *self {
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
            Choice::Rock => Choice::Scissors,
        }
    }

    fn losing_against(&self) -> Self {
        match *self {
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
            Choice::Rock => Choice::Paper,
        }
    }
}

#[derive(Debug)]
struct Round {
    own_choice: Choice,
    opp_choice: Choice,
}

impl Round {
    fn new(encoding: &str) -> Self {
        let mut choices: Vec<Choice> = encoding.split_whitespace().map(Choice::new).collect();

        assert!(choices.len() == 2);
        Self {
            own_choice: choices
                .pop()
                .expect("First item should be a choice for the opponent"),
            opp_choice: choices
                .pop()
                .expect("Second vector item should be a choice oneself"),
        }
    }

    fn new_alt(encoding: &str) -> Self {
        let two_symbols: Vec<_> = encoding.split_ascii_whitespace().collect();
        assert!(two_symbols.len() == 2);

        let opp_choice = Choice::new(two_symbols[0]);
        let own_choice = match two_symbols[1] {
            "X" => opp_choice.winning_against(),
            "Y" => opp_choice.clone(),
            "Z" => opp_choice.losing_against(),
            _ => panic!("Own choice should be between X-Z, but was {}", encoding),
        };
        Self {
            own_choice,
            opp_choice,
        }
    }

    fn outcome(&self) -> u32 {
        let own_score = self.own_choice.score();
        let round_score = match (&self.own_choice, &self.opp_choice) {
            (Choice::Paper, Choice::Paper)
            | (Choice::Scissors, Choice::Scissors)
            | (Choice::Rock, Choice::Rock) => 3,
            (Choice::Paper, Choice::Scissors) => 0,
            (Choice::Paper, Choice::Rock) => 6,
            (Choice::Scissors, Choice::Paper) => 6,
            (Choice::Scissors, Choice::Rock) => 0,
            (Choice::Rock, Choice::Paper) => 0,
            (Choice::Rock, Choice::Scissors) => 6,
        };
        own_score + round_score
    }
}
