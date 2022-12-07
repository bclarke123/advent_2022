use crate::util::*;
const INPUT: &str = include_str!("input/input_day3.txt");

fn char_val(c: char) -> u32 {
    let c_val: u32 = c.into();
    if c_val > 64 && c_val < 91 {
        c_val - 38
    } else {
        c_val - 96
    }
}

fn shared_char(lines: &[&str]) -> u32 {
    let ch1 = lines[0].chars();
    let others = &lines[1..];

    'chars: for c in ch1 {
        for line in others {
            if !line.contains(c) {
                continue 'chars;
            }
        }

        return char_val(c);
    }

    0
}

fn do_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|x| {
            let t = x.split_at(x.len() / 2);
            shared_char(&[t.0, t.1])
        })
        .sum()
}

pub fn part1() {
    let result = do_part1(INPUT);
    println!("The sum of values is {}", result);
}

fn do_part2(input: &str) -> u32 {
    input
        .lines()
        .collect::<Vec<_>>()
        .as_slice()
        .chunks(3)
        .map(shared_char)
        .sum()
}

pub fn part2() {
    let result = do_part2(INPUT);
    println!("The team badge is {}", result);
}

pub fn get_parts() -> DayFunc {
    (part1, part2)
}

#[test]
fn test_part1() {
    let test_input: &str = include_str!("input/input_day3_sample.txt");
    let result = do_part1(test_input);
    assert_eq!(result, 157);
}

#[test]
fn test_part2() {
    let test_input: &str = include_str!("input/input_day3_sample.txt");
    let result = do_part2(test_input);
    assert_eq!(result, 70);
}
