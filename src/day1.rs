use crate::util::*;
const INPUT: &str = include_str!("input/input_day1.txt");

fn get_carry_amounts(input: &str, buf: &mut Vec<usize>) {
    let lines = input.lines().collect::<Vec<&str>>();

    buf.truncate(0);
    buf.push(0);
    let mut idx = 0;

    for line in &lines {
        if let Ok(val) = line.parse::<usize>() {
            buf[idx] += val;
        } else {
            buf.push(0);
            idx += 1;
        }
    }

    buf.sort();
}

fn total_carry(buf: &Vec<usize>, n_elves: usize) -> usize {
    let len = buf.len() - n_elves;
    buf[len..].iter().sum()
}

fn tally(input: &str, n_elves: usize) -> usize {
    let mut buf: Vec<usize> = vec![];

    get_carry_amounts(input, &mut buf);
    total_carry(&buf, n_elves)
}

pub fn part1() {
    let max = tally(INPUT, 1);
    println!("The top elf is carrying {}", max);
}

pub fn part2() {
    let max = tally(INPUT, 3);
    println!("The top three elves are carrying {}", max);
}

pub fn get_parts() -> DayFunc {
    (part1, part2)
}

#[test]
fn test_part1() {
    let test_input: &str = include_str!("input/input_day1_sample.txt");
    let max = tally(test_input, 1);
    assert_eq!(max, 24000);
}

#[test]
fn test_part2() {
    let test_input: &str = include_str!("input/input_day1_sample.txt");
    let max = tally(test_input, 3);
    assert_eq!(max, 45000);
}
