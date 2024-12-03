use aoc2024::{read_input, InputType};
use timed::timed;

const DAY: u8 = 3;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[timed]
fn part1(input: &str) -> i32 {
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|cap| {
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();
            x * y
        })
        .sum()
}

#[timed]
fn part2(input: &str) -> i32 {
    let rx = regex::Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut can_mul = true;
    let mut res = 0;

    for cap in rx.captures_iter(input) {
        match &cap[0] {
            "do()" => {
                can_mul = true;
            }
            "don't()" => {
                can_mul = false;
            }
            _ => {
                if can_mul {
                    let x: i32 = cap[1].parse().unwrap();
                    let y: i32 = cap[2].parse().unwrap();
                    res += x * y;
                }
            }
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
        let expected = 161;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test() {
        let expected = 48;
        let result = part2(&get_test_input(InputType::Other("test2")));
        assert_eq!(result, expected);
    }
}
