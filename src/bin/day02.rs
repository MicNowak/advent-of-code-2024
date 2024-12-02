use aoc2024::{read_input, InputType};
use timed::timed;

const DAY: u8 = 2;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn check_raport(raport: &Vec<i32>) -> bool {
    let mut increasing = Option::None;
    for i in 1..raport.len() {
        let prev = raport[i - 1];
        let next = raport[i];
        if increasing.is_none() {
            increasing = Some(next > prev);
        }
        if (Some(true) == increasing && prev > next)
            || (Some(false) == increasing && prev < next)
            || prev == next
            || (prev - next).abs() > 3
        {
            return false;
        }
    }
    true
}

#[timed]
fn part1(input: &str) -> i32 {
    let mut res = 0;
    input.lines().for_each(|line| {
        let raport = line.split(' ').map(|x| x.parse().unwrap()).collect();
        if check_raport(&raport) {
            res += 1;
        }
    });
    res
}

#[timed]
fn part2(input: &str) -> i32 {
    let mut res = 0;
    input.lines().for_each(|line| {
        let raport = line.split(' ').map(|x| x.parse().unwrap()).collect();
        if check_raport(&raport) {
            res += 1;
            return;
        }
        let mut raport_copy = raport.clone();
        let mut is_safe = false;
        for i in 0..raport.len() {
            raport_copy.remove(i);
            if check_raport(&raport_copy) {
                is_safe = true;
                break;
            }
            raport_copy = raport.clone();
        }
        if is_safe {
            res += 1;
        }
    });
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
        let expected = 2;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test() {
        let expected = 4;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
