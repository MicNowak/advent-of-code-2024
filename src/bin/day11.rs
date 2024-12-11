use std::collections::HashMap;

use aoc2024::{read_input, InputType};
use timed::timed;

const DAY: u8 = 11;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn iterate_mem(stone: usize, n: usize, mem: &mut HashMap<(usize, usize), usize>) -> usize {
    if mem.contains_key(&(stone, n)) {
        return *mem.get(&(stone, n)).unwrap();
    }
    if n == 0 {
        return 1;
    }
    if stone == 0 {
        let res = iterate_mem(1, n - 1, mem);
        mem.insert((stone, n), res);
        return res;
    }
    let digits = (stone as f64).log10().floor() as i32 + 1;
    if digits % 2 == 0 {
        let left = (stone as f64 / 10.0_f64.powi(digits / 2)).floor() as usize;
        let right = (stone as f64 % 10.0_f64.powi(digits / 2)).floor() as usize;
        let res = iterate_mem(left, n - 1, mem) + iterate_mem(right, n - 1, mem);
        mem.insert((stone, n), res);
        return res;
    } else {
        let res = iterate_mem((stone * 2024) as usize, n - 1, mem);
        mem.insert((stone, n), res);
        return res;
    }
}

#[timed]
fn part1(input: &str) -> usize {
    let mut mem: HashMap<(usize, usize), usize> = HashMap::new();
    input
        .split(' ')
        .map(|s| iterate_mem(s.parse().unwrap(), 25, &mut mem))
        .sum()
}

#[timed]
fn part2(input: &str) -> usize {
    let mut mem: HashMap<(usize, usize), usize> = HashMap::new();
    input
        .split(' ')
        .map(|s| iterate_mem(s.parse().unwrap(), 75, &mut mem))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input(input_type: InputType) -> String {
        read_input(DAY, input_type).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 55312;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
