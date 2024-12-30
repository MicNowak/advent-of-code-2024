use std::collections::HashSet;

use aoc2024::{read_input, InputType};
use timed::timed;

const DAY: u8 = 16;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn find_start_end_position(map: &Vec<Vec<char>>) -> ((usize, usize), (usize, usize)) {
    let mut start = None;
    let mut end = None;
    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                start = Some((x, y));
            }
            if cell == 'E' {
                end = Some((x, y));
            }
            if start.is_some() && end.is_some() {
                return (start.unwrap(), end.unwrap());
            }
        }
    }
    panic!("No start position found");
}

fn value_map_directed(
    map: &Vec<Vec<char>>,
    (start, dir): ((usize, usize), usize),
) -> Vec<Vec<[usize; 4]>> {
    let mut value_map = vec![vec![[usize::MAX; 4]; map[0].len()]; map.len()];
    value_map[start.1][start.0][dir] = 0;
    let mut queue = vec![];
    queue.push((start, dir));
    let dir = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    while let Some(((x, y), current_dir)) = queue.pop() {
        for dir_index in current_dir as i32 - 1..=current_dir as i32 + 1 {
            let dir_index = dir_index.rem_euclid(4) as usize;
            let (dx, dy) = dir[dir_index as usize];
            let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
            if map[ny][nx] == '#' {
                continue;
            }
            let mut new_value = value_map[y][x][current_dir] + 1;
            if current_dir != dir_index {
                new_value += 1000;
            }
            if new_value < value_map[ny][nx][dir_index] {
                value_map[ny][nx][dir_index] = new_value;
                queue.push(((nx, ny), dir_index));
            }
        }
    }
    value_map
}

#[timed]
fn part1(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (start, end) = find_start_end_position(&map);

    value_map_directed(&map, (start, 1))[end.1][end.0]
        .into_iter()
        .min()
        .unwrap()
}

#[timed]
fn part2(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (start, end) = find_start_end_position(&map);
    let value_map = value_map_directed(&map, (start, 1));
    let smallest_path_size = *value_map[end.1][end.0].iter().min().unwrap();
    let dir = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut queue = value_map[end.1][end.0]
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == smallest_path_size)
        .map(|(i, _)| (end, i))
        .collect::<Vec<_>>();
    let mut visited = HashSet::new();
    visited.insert(end);
    while let Some(((x, y), dir_idx)) = queue.pop() {
        let prev_dir = (dir_idx + 2).rem_euclid(4);
        let (dx, dy) = dir[prev_dir];
        let (px, py) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
        if map[py][px] == '#' || visited.contains(&(px, py)) {
            continue;
        }
        let value = value_map[y][x][dir_idx];
        for (prev_dir, &prev_value) in value_map[py][px].iter().enumerate() {
            let diff = value.abs_diff(prev_value);
            if diff == 1 || diff == 1001 {
                queue.push(((px, py), prev_dir));
                visited.insert((px, py));
            }
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input(input_type: InputType) -> String {
        read_input(DAY, input_type).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 7036;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part1_test2() {
        let expected = 11048;
        let result = part1(&get_test_input(InputType::Other("test2")));
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 45;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test2() {
        let expected = 64;
        let result = part2(&get_test_input(InputType::Other("test2")));
        assert_eq!(result, expected);
    }
}
