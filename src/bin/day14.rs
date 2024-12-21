use aoc2024::{read_input, InputType};
use deflate::deflate_bytes;
use timed::timed;

const DAY: u8 = 14;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[timed]
fn part1(input: &str) -> usize {
    let mut pos_vel = input
        .lines()
        .map(|line| {
            let binding = line.replace("p=", "").replace("v=", "");
            let mut parts = binding.split(" ");
            let pos = parts
                .next()
                .unwrap()
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let vel = parts
                .next()
                .unwrap()
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            ((pos[0], pos[1]), (vel[0], vel[1]))
        })
        .collect::<Vec<_>>();
    let max_x = *pos_vel.iter().map(|((x, _), _)| x).max().unwrap() + 1;
    let max_y = *pos_vel.iter().map(|((_, y), _)| y).max().unwrap() + 1;
    let iterations = 100;
    let mut res = [0; 4];
    for i in 0..pos_vel.len() {
        let ((x, y), (vx, vy)) = pos_vel[i];
        let nx = (x + vx * iterations).rem_euclid(max_x);
        let ny = (y + vy * iterations).rem_euclid(max_y);
        pos_vel[i] = ((nx, ny), (vx, vy));
        if nx < max_x / 2 && ny < max_y / 2 {
            res[0] += 1;
        } else if nx > max_x / 2 && ny < max_y / 2 {
            res[1] += 1;
        } else if nx < max_x / 2 && ny > max_y / 2 {
            res[2] += 1;
        } else if nx > max_x / 2 && ny > max_y / 2 {
            res[3] += 1;
        }
    }
    res[0] * res[1] * res[2] * res[3]
}

fn part2(input: &str) -> usize {
    let mut pos_vel = input
        .lines()
        .map(|line| {
            let binding = line.replace("p=", "").replace("v=", "");
            let mut parts = binding.split(" ");
            let pos = parts
                .next()
                .unwrap()
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let vel = parts
                .next()
                .unwrap()
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            ((pos[0], pos[1]), (vel[0], vel[1]))
        })
        .collect::<Vec<_>>();
    let max_x = *pos_vel.iter().map(|((x, _), _)| x).max().unwrap() + 1;
    let max_y = *pos_vel.iter().map(|((_, y), _)| y).max().unwrap() + 1;
    let mut i = 0;
    loop {
        i += 1;
        for i in 0..pos_vel.len() {
            let ((x, y), (vx, vy)) = pos_vel[i];
            let nx = (x + vx).rem_euclid(max_x);
            let ny = (y + vy).rem_euclid(max_y);
            pos_vel[i] = ((nx, ny), (vx, vy));
        }
        let mut map = vec![vec![' '; max_x as usize]; max_y as usize];
        for ((x, y), _) in pos_vel.iter() {
            map[*y as usize][*x as usize] = '#';
        }
        let compressed = deflate_bytes(&map.iter().flatten().collect::<String>().as_bytes());
        if compressed.len() < 500 {
            return i;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input(input_type: InputType) -> String {
        read_input(DAY, input_type).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 12;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
