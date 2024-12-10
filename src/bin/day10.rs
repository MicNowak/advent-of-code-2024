use std::collections::VecDeque;

use aoc2024::{read_input, InputType};
use timed::timed;

const DAY: u8 = 10;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn find_zeros(input: &Vec<Vec<i32>>) -> Vec<(i32, i32)> {
    let mut zeros = Vec::new();
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == 0 {
                zeros.push((i as i32, j as i32));
            }
        }
    }
    zeros
}

fn count_trails(input: &Vec<Vec<i32>>, starting_pos: (i32, i32)) -> usize {
    let dir = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut queue = VecDeque::new();
    queue.push_back(starting_pos);
    let mut visited = vec![vec![false; input[0].len()]; input.len()];
    visited[starting_pos.0 as usize][starting_pos.1 as usize] = true;
    let mut count = 0;
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        for d in dir.iter() {
            let new_pos = (current.0 + d.0, current.1 + d.1);
            if new_pos.0 >= 0
                && new_pos.0 < input.len() as i32
                && new_pos.1 >= 0
                && new_pos.1 < input[0].len() as i32
                && !visited[new_pos.0 as usize][new_pos.1 as usize]
                && input[new_pos.0 as usize][new_pos.1 as usize]
                    == input[current.0 as usize][current.1 as usize] + 1
            {
                queue.push_back(new_pos);
                visited[new_pos.0 as usize][new_pos.1 as usize] = true;
                if input[new_pos.0 as usize][new_pos.1 as usize] == 9 {
                    count += 1;
                }
            }
        }
    }
    count
}

#[timed]
fn part1(input: &str) -> usize {
    let input = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let zeros = find_zeros(&input);
    let mut trails = 0;
    for z in zeros {
        trails += count_trails(&input, z);
    }
    trails
}

fn count_all_trails(input: &Vec<Vec<i32>>, starting_pos: (i32, i32)) -> usize {
    let dir = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut queue = VecDeque::new();
    queue.push_back(starting_pos);
    let mut visited = vec![vec![0; input[0].len()]; input.len()];
    visited[starting_pos.0 as usize][starting_pos.1 as usize] = 1;
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        for d in dir.iter() {
            let new_pos = (current.0 + d.0, current.1 + d.1);
            if new_pos.0 >= 0
                && new_pos.0 < input.len() as i32
                && new_pos.1 >= 0
                && new_pos.1 < input[0].len() as i32
                && input[new_pos.0 as usize][new_pos.1 as usize]
                    == input[current.0 as usize][current.1 as usize] + 1
            {
                queue.push_back(new_pos);
                visited[new_pos.0 as usize][new_pos.1 as usize] += 1;
            }
        }
    }
    let mut count = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == 9 {
                count += visited[i][j];
            }
        }
    }
    count
}

#[timed]
fn part2(input: &str) -> usize {
    let input = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let zeros = find_zeros(&input);
    let mut trails = 0;
    for z in zeros {
        trails += count_all_trails(&input, z);
    }
    trails
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input(input_type: InputType) -> String {
        read_input(DAY, input_type).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 36;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test() {
        let expected = 81;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
