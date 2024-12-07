use aoc2024::{read_input, InputType};
use timed::timed;

const DAY: u8 = 7;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn test(value: usize, numbers: Vec<usize>) -> bool {
    let combinations = 2_usize.pow((numbers.len() - 1) as u32);
    for i in 0..combinations {
        let formatted = format!("{:0width$b}", i, width = numbers.len() - 1);
        let operators = formatted.chars();
        let mut result = numbers[0];
        for (i, operator) in operators.enumerate() {
            result = match operator {
                '1' => result * numbers[i + 1],
                '0' => result + numbers[i + 1],
                _ => unreachable!(),
            };
            if result > value {
                break;
            }
        }
        if result == value {
            return true;
        }
    }
    false
}

#[timed]
fn part1(input: &str) -> usize {
    let input = input.lines().map(|line| {
        let mut parts = line.split(": ");
        (
            parts.next().unwrap().parse().unwrap(),
            parts
                .next()
                .unwrap()
                .split(' ')
                .map(|part| part.parse().unwrap())
                .collect(),
        )
    });
    let mut res = 0;
    for (value, numbers) in input {
        if test(value, numbers) {
            res += value;
        }
    }
    res
}

fn convert_to_base_3(mut n: usize, padding: usize) -> String {
    let mut res = String::new();
    while n > 0 {
        res.push_str(&(n % 3).to_string());
        n /= 3;
    }
    while res.len() < padding {
        res.push('0');
    }
    res.chars().rev().collect()
}
fn test2(value: usize, numbers: Vec<usize>) -> bool {
    let combinations = 3_usize.pow((numbers.len() - 1) as u32);
    for i in 0..combinations {
        let base_3_string = convert_to_base_3(i, numbers.len() - 1);
        let operators = base_3_string.chars();
        let mut result = numbers[0];
        for (i, operator) in operators.enumerate() {
            result = match operator {
                '2' => format!("{}{}", result, numbers[i + 1]).parse().unwrap(),
                '1' => result * numbers[i + 1],
                '0' => result + numbers[i + 1],
                _ => unreachable!(),
            };
            if result > value {
                break;
            }
        }
        if result == value {
            return true;
        }
    }
    false
}

#[timed]
fn part2(input: &str) -> usize {
    let input = input.lines().map(|line| {
        let mut parts = line.split(": ");
        (
            parts.next().unwrap().parse().unwrap(),
            parts
                .next()
                .unwrap()
                .split(' ')
                .map(|part| part.parse().unwrap())
                .collect(),
        )
    });
    let mut res = 0;
    for (value, numbers) in input {
        if test2(value, numbers) {
            res += value;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input(input_type: InputType) -> String {
        read_input(DAY, input_type).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 3749;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test() {
        let expected = 11387;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
