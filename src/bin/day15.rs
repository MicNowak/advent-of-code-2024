use aoc2024::{read_input, InputType};
use timed::timed;

const DAY: u8 = 15;
const NEW_LINE: &str = "\r\n";

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn find_robot_position(map: &Vec<Vec<char>>) -> (usize, usize) {
    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '@' {
                return (x, y);
            }
        }
    }
    panic!("No start position found");
}

fn move_robot(
    map: &mut Vec<Vec<char>>,
    position: (usize, usize),
    direction: (i32, i32),
) -> Option<(usize, usize)> {
    let (x, y) = position;
    let (dx, dy) = direction;
    let (nx, ny) = (x as i32 + dx, y as i32 + dy);
    let (nx, ny) = (nx as usize, ny as usize);
    if map[ny][nx] == '#' {
        return None;
    }
    if map[ny][nx] == 'O' {
        move_robot(map, (nx, ny), direction);
    }
    if map[ny][nx] == '.' {
        map[ny][nx] = map[y][x];
        map[y][x] = '.';
        return Some((nx, ny));
    }
    None
}

#[timed]
fn part1(input: &str) -> usize {
    let separator = NEW_LINE.repeat(2);
    let mut input = input.split(separator.as_str());
    let mut map = input
        .next()
        .unwrap()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let moves = input
        .next()
        .unwrap()
        .replace(NEW_LINE, "")
        .chars()
        .map(|c| match c {
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            _ => panic!("Invalid move"),
        })
        .collect::<Vec<_>>();
    let mut position = find_robot_position(&map);
    for dir in moves {
        if let Some(new_pos) = move_robot(&mut map, position, dir) {
            position = new_pos;
        }
    }
    let mut res = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'O' {
                res += 100 * y + x;
            }
        }
    }
    res
}

fn move_robot2(map: &mut Vec<Vec<char>>, position: (usize, usize), direction: (i32, i32)) {
    let (x, y) = position;
    let (dx, dy) = direction;
    let (nx, ny) = (x as i32 + dx, y as i32 + dy);
    let (nx, ny) = (nx as usize, ny as usize);
    let vertical = dx == 0;
    if map[ny][nx] == '[' {
        if vertical {
            move_robot2(map, (nx, ny), direction);
            move_robot2(map, (nx + 1, ny), direction);
        } else {
            move_robot2(map, (nx, ny), direction);
        }
    }
    if map[ny][nx] == ']' {
        if vertical {
            move_robot2(map, (nx - 1, ny), direction);
            move_robot2(map, (nx, ny), direction);
        } else {
            move_robot2(map, (nx, ny), direction);
        }
    }
    if map[ny][nx] == '.' {
        map[ny][nx] = map[y][x];
        map[y][x] = '.';
    }
}

fn can_move(map: &Vec<Vec<char>>, position: (usize, usize), direction: (i32, i32)) -> bool {
    let (x, y) = position;
    let (dx, dy) = direction;
    let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
    if map[ny][nx] == '#' {
        return false;
    }
    if map[ny][nx] == '.' {
        return true;
    }
    let vertical = dx == 0;
    if vertical {
        if map[ny][nx] == '[' {
            return can_move(map, (nx, ny), direction) && can_move(map, (nx + 1, ny), direction);
        } else if map[ny][nx] == ']' {
            return can_move(map, (nx - 1, ny), direction) && can_move(map, (nx, ny), direction);
        }
    }
    return can_move(map, (nx, ny), direction);
}

#[timed]
fn part2(input: &str) -> usize {
    let separator = NEW_LINE.repeat(2);
    let mut input = input.split(separator.as_str());
    let mut map = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.chars().fold(vec![], |mut acc, c| {
                match c {
                    '#' => {
                        acc.push('#');
                        acc.push('#');
                    }
                    'O' => {
                        acc.push('[');
                        acc.push(']');
                    }
                    '@' => {
                        acc.push('@');
                        acc.push('.');
                    }
                    _ => {
                        acc.push('.');
                        acc.push('.');
                    }
                };
                acc
            })
        })
        .collect::<Vec<_>>();
    let moves = input
        .next()
        .unwrap()
        .replace(NEW_LINE, "")
        .chars()
        .map(|c| match c {
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            _ => panic!("Invalid move"),
        })
        .collect::<Vec<_>>();
    let mut position = find_robot_position(&map);
    for dir in moves {
        if can_move(&map, position, dir) {
            move_robot2(&mut map, position, dir);
            let (x, y) = position;
            let (dx, dy) = dir;
            position = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
        }
    }
    let mut res = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '[' {
                res += 100 * y + x;
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
        let expected = 10092;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 9021;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
