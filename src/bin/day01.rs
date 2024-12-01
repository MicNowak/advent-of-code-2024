use aoc2024::{read_input, InputType};
use timed::timed;

const DAY: u8 = 1;

fn main() {
    let input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[timed]
fn part1(input: &str) -> i32 {
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];
    input.lines().for_each(|x| {
        let mut tmp = x.split("   ");
        list1.push(tmp.next().unwrap().parse().unwrap());
        list2.push(tmp.next().unwrap().parse().unwrap());
    });
    list1.sort();
    list2.sort();

    let mut res = 0;
    for i in 0..list1.len() {
        res += (list1[i] - list2[i]).abs();
    }
    res
}

#[timed]
fn part2(input: &str) -> i32 {
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];
    input.lines().for_each(|x| {
        let mut tmp = x.split("   ");
        list1.push(tmp.next().unwrap().parse().unwrap());
        list2.push(tmp.next().unwrap().parse().unwrap());
    });
    let mut res = 0;
    for i in list1.into_iter() {
        res += i * list2.iter().filter(|&&x| x == i).count() as i32;
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
        let expected = 11;
        let result = part1(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test() {
        let expected = 31;
        let result = part2(&get_test_input(InputType::Test));
        assert_eq!(result, expected);
    }
}
