use std::collections::HashSet;

use aoc2024::{read_input, InputType};
use timed::timed;

const DAY: u8 = 6;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn find_start(input: &Vec<Vec<char>>) -> (i32, i32) {
    for (y, line) in input.iter().enumerate() {
        let x = line.iter().position(|&x| x == '^');
        if let Some(x) = x {
            return (x as i32, y as i32);
        }
    }
    (0, 0)
}
#[timed]
fn part1(input: &str) -> usize {
    let input: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let max_pos = (input[0].len() as i32, input.len() as i32);
    let mut pos = find_start(&input);
    let dir = [
        (0, -1), // up
        (1, 0),  // right
        (0, 1),  // down
        (-1, 0), // left
    ];
    let mut current_dir = 0;
    let mut visited = HashSet::new();

    while (0..max_pos.0).contains(&(pos.0 + dir[current_dir].0))
        && (0..max_pos.1).contains(&(pos.1 + dir[current_dir].1))
    {
        let next_pos = (pos.0 + dir[current_dir].0, pos.1 + dir[current_dir].1);
        if input[next_pos.1 as usize][next_pos.0 as usize] == '#' {
            current_dir = (current_dir + 1) % 4;
        } else {
            pos = next_pos;
            visited.insert(pos);
        }
    }
    visited.len()
}

#[timed]
fn part2(input: &str) -> usize {
    let input: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let max_pos = (input[0].len() as i32, input.len() as i32);
    let start = find_start(&input);
    let dir = [
        (0, -1), // up
        (1, 0),  // right
        (0, 1),  // down
        (-1, 0), // left
    ];
    let mut current_dir;
    let mut res = 0;

    for x in 0..max_pos.0 {
        for y in 0..max_pos.1 {
            if input[y as usize][x as usize] == '#' && input[y as usize][x as usize] == '^' {
                continue;
            }
            let mut pos = start;
            current_dir = 0;
            let mut new_input = input.clone();
            new_input[y as usize][x as usize] = '#';
            let mut visited = HashSet::new();
            visited.insert((pos, 0));

            while (0..max_pos.0).contains(&(pos.0 + dir[current_dir].0))
                && (0..max_pos.1).contains(&(pos.1 + dir[current_dir].1))
            {
                let next_pos = (pos.0 + dir[current_dir].0, pos.1 + dir[current_dir].1);
                if new_input[next_pos.1 as usize][next_pos.0 as usize] == '#' {
                    current_dir = (current_dir + 1) % 4;
                } else {
                    pos = next_pos;
                    if visited.contains(&(pos, current_dir)) {
                        res += 1;
                        break;
                    } else {
                        visited.insert((pos, current_dir));
                    }
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
        let expected = 41;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test() {
        let expected = 6;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
