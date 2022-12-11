use crate::util::*;

#[derive(Debug)]
enum Target {
    Scalar(u32),
    Old,
}
#[derive(Debug)]
enum Operation {
    Add(Target),
    Multiply(Target),
}

impl Operation {
    fn new(words: &[&str]) -> Self {
        let target = match words[1] {
            "old" => Target::Old,
            x => Target::Scalar(x.parse::<u32>().unwrap()),
            _ => panic!(),
        };

        match words[0].chars().next().unwrap() {
            '+' => Operation::Add(target),
            '*' => Operation::Multiply(target),
            _ => panic!("Unrecognized operator"),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    div_test: u32,
}

impl Monkey {
    fn new(input: &[&str]) {
        let args = input[1..6]
            .iter()
            .map(|l| {
                let pos = l.chars().position(|c| c == ':').unwrap();
                l.split_at(pos + 2).1
            })
            .collect::<Vec<_>>();

        let items = args[0].split(", ").map(|i| i.parse::<u32>().unwrap());

        let op_arr = args[1].split(' ').skip(3).collect::<Vec<_>>();
        let operation = Operation::new(&op_arr);

        println!("{:?}", args);
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let mut ret = vec![];
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(7)
        .map(Monkey::new)
        .collect::<Vec<_>>();

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
