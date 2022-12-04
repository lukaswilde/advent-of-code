use utils::parse_text;

#[derive(Debug, Clone)]
struct Range {
    begin: u32,
    end: u32,
}

impl Range {
    fn includes(&self, other: &Range) -> bool {
        other.begin >= self.begin && other.end <= self.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        other.begin <= self.end && other.end >= self.begin
    }
}

fn main() {
    let text = parse_text();
    let text = text.replace(' ', "");
    let range_groups = create_range_groups(&text);
    let total_overlaps = get_num_total_overlaps(&range_groups);
    let overlaps = get_num_overlaps(&range_groups);
    println!(
        "The number of completely overlapping ranges is {}",
        total_overlaps
    );
    println!("The number of overlapping ranges is {}", overlaps);
}

fn create_range(r_str: &str) -> Range {
    let endpoints: Vec<u32> = r_str
        .split('-')
        .map(|x| x.parse::<u32>().expect("Endpoints should be numbers"))
        .collect();
    assert_eq!(endpoints.len(), 2);
    Range {
        begin: endpoints[0],
        end: endpoints[1],
    }
}

fn split_single_line(line: &str) -> (Range, Range) {
    let ranges: Vec<Range> = line.split(',').map(create_range).collect();
    assert_eq!(ranges.len(), 2);
    (ranges[0].clone(), ranges[1].clone())
}

fn create_range_groups(text: &str) -> Vec<(Range, Range)> {
    text.split('\n').map(split_single_line).collect()
}

fn get_num_total_overlaps(ranges: &[(Range, Range)]) -> u32 {
    ranges
        .iter()
        .map(|(x, y)| (x.includes(y) || y.includes(x)) as u32)
        .sum()
}

fn get_num_overlaps(ranges: &[(Range, Range)]) -> u32 {
    ranges
        .iter()
        .map(|(x, y)| (x.overlaps(y) || y.overlaps(x)) as u32)
        .sum()
}
