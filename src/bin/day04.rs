use aoc2024::{read_input, InputType};
use timed::timed;

const DAY: u8 = 4;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[timed]
fn part1(input: &str) -> usize {
    let input = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut res = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] != 'X' {
                continue;
            }

            // North
            if y >= 3 && input[y - 1][x] == 'M' && input[y - 2][x] == 'A' && input[y - 3][x] == 'S'
            {
                res += 1;
            }
            // North-East
            if y >= 3
                && x <= input[y].len() - 4
                && input[y - 1][x + 1] == 'M'
                && input[y - 2][x + 2] == 'A'
                && input[y - 3][x + 3] == 'S'
            {
                res += 1;
            }
            // East
            if x <= input[y].len() - 4
                && input[y][x + 1] == 'M'
                && input[y][x + 2] == 'A'
                && input[y][x + 3] == 'S'
            {
                res += 1;
            }
            // South-East
            if y <= input.len() - 4
                && x <= input[y].len() - 4
                && input[y + 1][x + 1] == 'M'
                && input[y + 2][x + 2] == 'A'
                && input[y + 3][x + 3] == 'S'
            {
                res += 1;
            }
            // South
            if y <= input.len() - 4
                && input[y + 1][x] == 'M'
                && input[y + 2][x] == 'A'
                && input[y + 3][x] == 'S'
            {
                res += 1;
            }
            // South-West
            if y <= input.len() - 4
                && x >= 3
                && input[y + 1][x - 1] == 'M'
                && input[y + 2][x - 2] == 'A'
                && input[y + 3][x - 3] == 'S'
            {
                res += 1;
            }
            // West
            if x >= 3 && input[y][x - 1] == 'M' && input[y][x - 2] == 'A' && input[y][x - 3] == 'S'
            {
                res += 1;
            }
            // North-West
            if y >= 3
                && x >= 3
                && input[y - 1][x - 1] == 'M'
                && input[y - 2][x - 2] == 'A'
                && input[y - 3][x - 3] == 'S'
            {
                res += 1;
            }
        }
    }
    res
}

#[timed]
fn part2(input: &str) -> usize {
    let input = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut res = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] != 'A' {
                continue;
            }

            // M.S
            // .A.
            // M.S
            if y >= 1
                && x >= 1
                && y <= input.len() - 2
                && x <= input[y].len() - 2
                && input[y - 1][x - 1] == 'M'
                && input[y + 1][x - 1] == 'M'
                && input[y - 1][x + 1] == 'S'
                && input[y + 1][x + 1] == 'S'
            {
                res += 1;
            }
            // M.M
            // .A.
            // S.S
            if y >= 1
                && x >= 1
                && y <= input.len() - 2
                && x <= input[y].len() - 2
                && input[y - 1][x - 1] == 'M'
                && input[y - 1][x + 1] == 'M'
                && input[y + 1][x - 1] == 'S'
                && input[y + 1][x + 1] == 'S'
            {
                res += 1;
            }
            // S.M
            // .A.
            // S.M
            if y >= 1
                && x >= 1
                && y <= input.len() - 2
                && x <= input[y].len() - 2
                && input[y - 1][x + 1] == 'M'
                && input[y + 1][x + 1] == 'M'
                && input[y - 1][x - 1] == 'S'
                && input[y + 1][x - 1] == 'S'
            {
                res += 1;
            }
            // S.S
            // .A.
            // M.M
            if y >= 1
                && x >= 1
                && y <= input.len() - 2
                && x <= input[y].len() - 2
                && input[y + 1][x - 1] == 'M'
                && input[y + 1][x + 1] == 'M'
                && input[y - 1][x - 1] == 'S'
                && input[y - 1][x + 1] == 'S'
            {
                res += 1;
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
        let expected = 18;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part1_test2() {
        let expected = 4;
        let result = part1(&get_test_input(InputType::Other("test2")));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test() {
        let expected = 9;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
