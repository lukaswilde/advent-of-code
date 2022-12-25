use utils::parse_text;

fn main() {
    let text = parse_text();
    let snafu_numbers = split_snafu_numbers(&text);

    let decimal_sum: isize = snafu_numbers.iter().map(|snafu| snafu2decimal(snafu)).sum();
    let snafu_result = decimal2snafu(decimal_sum);

    println!("The snafu number to enter is {}", snafu_result);
}

fn split_snafu_numbers(text: &str) -> Vec<String> {
    text.lines().map(|s| s.to_string()).collect()
}

fn snafu2decimal(snafu: &str) -> isize {
    snafu
        .chars()
        .rfold((0, 0), |(res, power), cur| {
            let coefficient = match cur {
                '=' => -2,
                '-' => -1,
                x => x.to_digit(10).expect("Should be 0, 1 or 2") as isize,
            };
            (res + coefficient * 5isize.pow(power), power + 1)
        })
        .0
}

fn _try_increase(acc: &mut Vec<isize>) {
    let last = acc.pop();
    if let Some(last) = last {
        if last == 2 {
            _try_increase(acc);
            acc.push(-2);
        } else {
            acc.push(last + 1);
        }
    } else {
        acc.push(1);
    }
}
fn _try_decrease(acc: &mut Vec<isize>) {
    let last = acc.pop();
    if let Some(last) = last {
        if last == -2 {
            _try_decrease(acc);
            acc.push(2);
        } else {
            acc.push(last - 1);
        }
    } else {
        acc.push(-1);
    }
}

fn decimal2snafu(number: isize) -> String {
    let mut buffer = vec![];
    _decimal2snafu_helper(&mut buffer, number, None);
    _create_snafu_string(buffer)
}

fn _decimal2snafu_helper(acc: &mut Vec<isize>, number: isize, cur_place: Option<u32>) {
    if number == 0 {
        (0..=cur_place.unwrap_or_default()).for_each(|_| acc.push(0));
    } else {
        let place = cur_place.unwrap_or_else(|| {
            let mut starting_point = 20;
            while number % 5isize.pow(starting_point) == number {
                starting_point -= 1;
            }
            starting_point
        });

        let division = number / 5isize.pow(place);
        if division < -2 {
            _try_decrease(acc);
            let new_number = number + 5isize.pow(place + 1);
            _decimal2snafu_helper(acc, new_number, Some(place))
        } else if division > 2 {
            _try_increase(acc);
            let new_number = number - 5isize.pow(place + 1);
            _decimal2snafu_helper(acc, new_number, Some(place))
        } else {
            acc.push(division);
            let new_number = number % 5isize.pow(place);
            if place != 0 {
                _decimal2snafu_helper(acc, new_number, Some(place - 1))
            } else {
                assert_eq!(number, division);
            }
        }
    }
}

fn _create_snafu_string(numbers: Vec<isize>) -> String {
    numbers
        .into_iter()
        .map(|d| match d {
            -1 => '-',
            -2 => '=',
            x if (0..=2).contains(&x) && !x.is_negative() => {
                char::from_digit(x as u32, 10).expect("Should be convertible")
            }
            _ => unreachable!(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal2snafu_1() {
        let decimals = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 20, 2022, 12345, 314159265,
        ];
        let results = decimals.into_iter().map(decimal2snafu).collect::<Vec<_>>();
        let expected = [
            "1",
            "2",
            "1=",
            "1-",
            "10",
            "11",
            "12",
            "2=",
            "2-",
            "20",
            "1=0",
            "1-0",
            "1=11-2",
            "1-0---0",
            "1121-1110-1=0",
        ];
        assert_eq!(results, expected);
    }

    #[test]
    fn test_decimal2snafu_2() {
        let decimals = [1747, 906, 198, 11, 201, 31, 1257, 32, 353, 107, 7, 3, 37];
        let results = decimals.into_iter().map(decimal2snafu).collect::<Vec<_>>();
        let expected = [
            "1=-0-2", "12111", "2=0=", "21", "2=01", "111", "20012", "112", "1=-1=", "1-12", "12",
            "1=", "122",
        ];
        assert_eq!(results, expected);
    }

    #[test]
    fn test_snafu2decimal_1() {
        let expected = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 20, 2022, 12345, 314159265,
        ];
        let snafu = [
            "1",
            "2",
            "1=",
            "1-",
            "10",
            "11",
            "12",
            "2=",
            "2-",
            "20",
            "1=0",
            "1-0",
            "1=11-2",
            "1-0---0",
            "1121-1110-1=0",
        ];

        let results = snafu.into_iter().map(snafu2decimal).collect::<Vec<_>>();
        assert_eq!(results, expected);
    }

    #[test]
    fn test_snafu2decimal_2() {
        let snafu = [
            "1=-0-2", "12111", "2=0=", "21", "2=01", "111", "20012", "112", "1=-1=", "1-12", "12",
            "1=", "122",
        ];
        let results = snafu.into_iter().map(snafu2decimal).collect::<Vec<_>>();
        let expected = [1747, 906, 198, 11, 201, 31, 1257, 32, 353, 107, 7, 3, 37];
        assert_eq!(results, expected);
    }
}
