use crate::util::*;
const INPUT: &str = include_str!("input/input_day5.txt");

fn parse_start<'a>(start: &[&'a str]) -> Vec<Vec<&'a str>> {
    let cols = start[0].len() / 4 + 1;
    let mut ret: Vec<Vec<&str>> = vec![vec![]; cols];

    for i in 1..=start.len() {
        let row = start[start.len() - i];
        for (c, buf) in ret.iter_mut().enumerate() {
            let idx = c * 4 + 1;
            let col = &row[idx..idx + 1];
            match col {
                " " => continue,
                x => buf.push(x),
            }
        }
    }

    ret
}

fn parse_input(input: &str) -> (Vec<Vec<&str>>, Vec<&str>) {
    let lines = input.lines();
    let mut start: Vec<&str> = vec![];
    let mut instructions: Vec<&str> = vec![];
    let mut blank = false;

    for line in lines {
        if line.is_empty() {
            blank = true;
            continue;
        }

        if blank {
            instructions.push(line);
        } else {
            start.push(line);
        }
    }

    start.truncate(start.len() - 1);

    let stacks = parse_start(&start);

    (stacks, instructions)
}

fn execute_instruction(instruction: &str, stacks: &mut [Vec<&str>], multiple: bool) {
    let parsed = instruction
        .split(' ')
        .enumerate()
        .filter(|(i, _)| i % 2 != 0)
        .map(|(_, x)| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let n = parsed[0];
    let source = parsed[1] - 1;
    let dest = parsed[2] - 1;
    let len = stacks[dest].len();

    for _ in 0..n {
        if let Some(val) = stacks[source].pop() {
            if multiple {
                stacks[dest].insert(len, val);
            } else {
                stacks[dest].push(val);
            }
        }
    }
}

fn execute(stacks: &mut [Vec<&str>], instructions: &[&str], multiple: bool) -> String {
    for instruction in instructions {
        execute_instruction(instruction, stacks, multiple);
    }

    stacks
        .iter()
        .map(|x| x.last().unwrap())
        .fold("".to_string(), |acc, v| acc + v)
}

pub fn part1() {
    let (mut stacks, instructions) = parse_input(INPUT);
    let ret = execute(&mut stacks, &instructions, false);
    println!("The top crates after rearranging are {}", ret);
}

pub fn part2() {
    let (mut stacks, instructions) = parse_input(INPUT);
    let ret = execute(&mut stacks, &instructions, true);
    println!("The top crates after rearranging are {}", ret);
}

pub fn get_parts() -> DayFunc {
    (part1, part2)
}

#[test]
fn test_part1() {
    let test_input: &str = include_str!("input/input_day5_sample.txt");
    let (mut stacks, instructions) = parse_input(test_input);
    let ret = execute(&mut stacks, &instructions, false);
    assert_eq!(ret, "CMZ");
}

#[test]
fn test_part2() {
    let test_input: &str = include_str!("input/input_day5_sample.txt");
    let (mut stacks, instructions) = parse_input(test_input);
    let ret = execute(&mut stacks, &instructions, true);
    assert_eq!(ret, "MCD");
}
