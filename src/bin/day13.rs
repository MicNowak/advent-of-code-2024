use aoc2024::{read_input, InputType};
use timed::timed;

const DAY: u8 = 13;
const NEW_LINE: &str = "\r\n";

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[timed]
fn part1(input: &str) -> usize {
    let re = regex::Regex::new(r"(\d+), Y.(\d+)").unwrap();
    input
        .split(NEW_LINE.repeat(2).as_str())
        .fold(0, |acc, machine| {
            let mut machine = re.captures_iter(machine).map(|cap| {
                let x: f64 = cap[1].parse().unwrap();
                let y: f64 = cap[2].parse().unwrap();
                (x, y)
            });
            let button_a = machine.next().unwrap();
            let button_b = machine.next().unwrap();
            let result = machine.next().unwrap();
            let times_b = (result.1 * button_a.0 - result.0 * button_a.1)
                / (button_b.1 * button_a.0 - button_b.0 * button_a.1);
            let times_a = (result.0 - times_b * button_b.0) / button_a.0;

            if times_a >= 0.0 && times_b >= 0.0 && times_a.fract() == 0.0 && times_b.fract() == 0.0
            {
                acc + times_a as usize * 3 + times_b as usize
            } else {
                acc
            }
        })
}

#[timed]
fn part2(input: &str) -> usize {
    let re = regex::Regex::new(r"(\d+), Y.(\d+)").unwrap();
    input
        .split(NEW_LINE.repeat(2).as_str())
        .fold(0, |acc, machine| {
            let mut machine = re.captures_iter(machine).map(|cap| {
                let x: f64 = cap[1].parse().unwrap();
                let y: f64 = cap[2].parse().unwrap();
                (x, y)
            });
            let button_a = machine.next().unwrap();
            let button_b = machine.next().unwrap();
            let mut result = machine.next().unwrap();
            result = (result.0 + 10000000000000.0, result.1 + 10000000000000.0);
            let times_b = (result.1 * button_a.0 - result.0 * button_a.1)
                / (button_b.1 * button_a.0 - button_b.0 * button_a.1);
            let times_a = (result.0 - times_b * button_b.0) / button_a.0;

            if times_a >= 0.0 && times_b >= 0.0 && times_a.fract() == 0.0 && times_b.fract() == 0.0
            {
                acc + times_a as usize * 3 + times_b as usize
            } else {
                acc
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input(input_type: InputType) -> String {
        read_input(DAY, input_type).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 480;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
