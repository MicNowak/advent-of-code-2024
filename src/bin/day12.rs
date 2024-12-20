use std::collections::{HashSet, VecDeque};

use aoc2024::{read_input, InputType};
use timed::timed;

const DAY: u8 = 12;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn count_walls(input: &mut Vec<Vec<char>>, x: usize, y: usize, character: char) -> usize {
    let mut count = 0;
    input[y][x] = '.';

    let dir = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    for (dx, dy) in dir.iter() {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx < 0 || ny < 0 || nx >= input[0].len() as i32 || ny >= input.len() as i32 {
            count += 1;
            continue;
        }
        if input[ny as usize][nx as usize] == '.' {
            continue;
        }
        if input[ny as usize][nx as usize] != character {
            count += 1;
        } else {
            count += count_walls(input, nx as usize, ny as usize, character);
        }
    }
    count
}

#[timed]
fn part1(input: &str) -> usize {
    let input = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut res = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == '.' {
                continue;
            }
            res += count_walls(&mut input.clone(), x, y, input[y][x]);
        }
    }
    res
}

fn get_region(
    input: &mut Vec<Vec<char>>,
    x: i32,
    y: i32,
    character: char,
) -> (usize, HashSet<(i32, i32)>) {
    let mut size = 0;
    let mut res = HashSet::new();
    let mut stack = VecDeque::new();
    stack.push_back((x, y));
    input[y as usize][x as usize] = '.';

    let dir = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    while let Some((x, y)) = stack.pop_front() {
        res.insert((x * 2, y * 2));
        res.insert((x * 2 + 1, y * 2));
        res.insert((x * 2, y * 2 + 1));
        res.insert((x * 2 + 1, y * 2 + 1));
        size += 1;

        for (dx, dy) in dir.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0
                || ny < 0
                || nx >= input[0].len() as i32
                || ny >= input.len() as i32
                || input[ny as usize][nx as usize] != character
            {
                continue;
            }
            input[ny as usize][nx as usize] = '.';
            stack.push_back((nx, ny));
        }
    }
    (size, res)
}

#[timed]
fn part2(input: &str) -> usize {
    let mut input = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut regions = vec![];
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == '.' {
                continue;
            }
            let character = input[y][x];
            regions.push(get_region(&mut input, x as i32, y as i32, character));
        }
    }
    let dir = [
        (-1, 0),
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, -1),
        (-1, 1),
        (1, 1),
        (1, -1),
    ];
    let mut res = 0;
    for (size, region) in regions {
        let mut corners = 0;
        for (x, y) in region.iter() {
            let mut neighbors = [false; 8];
            for (i, (dx, dy)) in dir.iter().enumerate() {
                let nx = x + dx;
                let ny = y + dy;
                if region.contains(&(nx, ny)) {
                    neighbors[i] = true;
                }
            }
            let n_count = neighbors.iter().filter(|&&b| b).count();
            if !matches!(n_count, 8 | 6 | 5) {
                corners += 1;
            }
        }
        res += size * corners;
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
        let expected = 1930;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 1206;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
