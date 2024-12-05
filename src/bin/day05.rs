use aoc2024::{read_input, InputType};
use timed::timed;

const DAY: u8 = 5;
const NEW_LINE: &str = "\r\n";

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn check_if_valid(rules: &Vec<Vec<usize>>, order: &Vec<usize>) -> bool {
    for rule in rules {
        let first = order.iter().position(|x| x == &rule[0]);
        let second = order.iter().position(|x| x == &rule[1]);
        if first.is_none() || second.is_none() {
            continue;
        }
        if first.unwrap() > second.unwrap() {
            return false;
        }
    }
    true
}
#[timed]
fn part1(input: &str) -> usize {
    let new_line_repeat = NEW_LINE.repeat(2);
    let mut input = input.split(&new_line_repeat);
    let rules: Vec<Vec<usize>> = input
        .next()
        .unwrap()
        .lines()
        .map(|x| x.split('|').map(|x| x.parse().unwrap()).collect())
        .collect();
    let orders: Vec<Vec<usize>> = input
        .next()
        .unwrap()
        .lines()
        .map(|x| x.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    let mut res = 0;
    for order in orders {
        if check_if_valid(&rules, &order) {
            res += order[order.len() / 2];
        }
    }
    res
}

fn fix_order(rules: &Vec<Vec<usize>>, order: &mut Vec<usize>) {
    while !check_if_valid(rules, order) {
        for rule in rules {
            let first = order.iter().position(|x| x == &rule[0]);
            let second = order.iter().position(|x| x == &rule[1]);
            if first.is_none() || second.is_none() {
                continue;
            }
            if first.unwrap() > second.unwrap() {
                let temp = order[first.unwrap()];
                order[first.unwrap()] = order[second.unwrap()];
                order[second.unwrap()] = temp;
            }
        }
    }
}
#[timed]
fn part2(input: &str) -> usize {
    let new_line_repeat = NEW_LINE.repeat(2);
    let mut input = input.split(&new_line_repeat);
    let rules: Vec<Vec<usize>> = input
        .next()
        .unwrap()
        .lines()
        .map(|x| x.split('|').map(|x| x.parse().unwrap()).collect())
        .collect();
    let mut orders: Vec<Vec<usize>> = input
        .next()
        .unwrap()
        .lines()
        .map(|x| x.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    let mut res = 0;
    for order in orders.iter_mut() {
        if !check_if_valid(&rules, order) {
            fix_order(&rules, order);
            res += order[order.len() / 2];
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
        let expected = 143;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test() {
        let expected = 123;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
