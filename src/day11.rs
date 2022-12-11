use crate::util::*;

enum Target {
    Scalar(u32),
    Old,
}

enum Operation {
    Add(Target),
    Subtract(Target),
    Multiply(Target),
    Divide(Target),
}

struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    div_test: u32,
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let mut ret = vec![];

    ret
}

pub fn get_parts() -> DayFunc {
    (|| {}, || {})
}

#[test]
fn test_part1() {
    let input: &str = include_str!("input/input_day11_sample.txt");
    let monkeys = parse_input(input);
}
