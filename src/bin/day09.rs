use aoc2024::{read_input, InputType};
use timed::timed;

const DAY: u8 = 9;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[derive(Debug, PartialEq)]
enum Space {
    Empty,
    File(usize),
}

#[timed]
fn part1(input: &str) -> usize {
    let mut input = input.chars().enumerate().fold(vec![], |mut acc, (i, c)| {
        for _ in 0..c.to_digit(10).unwrap() {
            if i % 2 == 0 {
                acc.push(Space::File(i / 2));
            } else {
                acc.push(Space::Empty);
            }
        }
        acc
    });
    let mut i = 0;
    while input.contains(&Space::Empty) {
        if input[i] != Space::Empty {
            i += 1;
            continue;
        }
        let mut last_file = input.pop().unwrap();
        while last_file == Space::Empty {
            last_file = input.pop().unwrap();
        }
        input[i] = last_file;
    }
    input.into_iter().enumerate().fold(0, |acc, (i, s)| {
        if let Space::File(id) = s {
            acc + id * i
        } else {
            acc
        }
    })
}

#[derive(Debug, Clone, Copy)]
enum Space2 {
    Empty(u32),
    File(u32, usize, bool),
}

#[timed]
fn part2(input: &str) -> usize {
    let mut input = input.chars().enumerate().fold(vec![], |mut acc, (i, c)| {
        let c = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            acc.push(Space2::File(c, i / 2, false));
        } else {
            acc.push(Space2::Empty(c));
        }
        acc
    });
    'outer: loop {
        let mut found = false;
        let mut i = input.len() - 1;
        let mut fsize = 0;
        let mut fid = 0;
        while i > 0 {
            match input[i] {
                Space2::File(size, id, false) => {
                    fsize = size;
                    fid = id;
                    found = true;
                    break;
                }
                _ => {
                    i -= 1;
                    continue;
                }
            }
        }
        if !found {
            break;
        }
        for j in 0..i {
            match input[j] {
                Space2::Empty(n) => {
                    if n >= fsize {
                        let dif = n - fsize;
                        input.remove(i);
                        input.insert(i, Space2::Empty(fsize));
                        input.remove(j);
                        if dif > 0 {
                            input.insert(j, Space2::Empty(dif));
                        }
                        input.insert(j, Space2::File(fsize, fid, true));
                        continue 'outer;
                    }
                }
                _ => {}
            }
        }
        input[i] = Space2::File(fsize, fid, true);
    }
    let mut res = 0;
    let mut i = 0;
    for s in input {
        match s {
            Space2::File(size, id, _) => {
                res += id * (i..i + size as usize).sum::<usize>();
                i += size as usize;
            }
            Space2::Empty(n) => i += n as usize,
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
        let expected = 1928;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test() {
        let expected = 2858;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
