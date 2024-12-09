use std::collections::{HashMap, HashSet};

use aoc2024::{read_input, InputType};
use timed::timed;

const DAY: u8 = 8;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn get_antenas(input: &Vec<Vec<char>>) -> HashMap<char, Vec<(i32, i32)>> {
    let mut antenas = HashMap::new();
    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c != '.' {
                antenas
                    .entry(*c)
                    .or_insert_with(Vec::new)
                    .push((j as i32, i as i32));
            }
        }
    }
    antenas
}

#[timed]
fn part1(input: &str) -> usize {
    let input = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let antenas = get_antenas(&input);
    let mut antinodes = HashSet::new();
    for (_antena, positions) in antenas {
        for (i, (x1, y1)) in positions.iter().enumerate() {
            for (j, (x2, y2)) in positions.iter().enumerate() {
                if i == j {
                    continue;
                }
                let dx = x1 - x2;
                let dy = y1 - y2;
                let antinode = (x1 + dx, y1 + dy);
                if (0..input[0].len()).contains(&(antinode.0 as usize))
                    && (0..input.len()).contains(&(antinode.1 as usize))
                {
                    antinodes.insert(antinode);
                }
            }
        }
    }
    antinodes.len()
}

#[timed]
fn part2(input: &str) -> usize {
    let input = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let antenas = get_antenas(&input);
    let mut antinodes = HashSet::new();
    for (_antena, positions) in antenas {
        for (i, (x1, y1)) in positions.iter().enumerate() {
            for (j, (x2, y2)) in positions.iter().enumerate() {
                if i == j {
                    continue;
                }
                let dx = x1 - x2;
                let dy = y1 - y2;
                let mut x = *x1;
                let mut y = *y1;
                while (0..input[0].len() as i32).contains(&x)
                    && (0..input.len() as i32).contains(&y)
                {
                    antinodes.insert((x, y));
                    x += dx;
                    y += dy;
                }
            }
        }
    }
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input(input_type: InputType) -> String {
        read_input(DAY, input_type).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 14;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test() {
        let expected = 34;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
