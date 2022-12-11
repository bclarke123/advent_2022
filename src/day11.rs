use crate::util::*;
use std::cell::RefCell;

const INPUT: &str = include_str!("input/input_day11.txt");

#[derive(Debug)]
enum Target {
    Scalar(u64),
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
            x => Target::Scalar(x.parse::<u64>().unwrap()),
        };

        match words[0].chars().next().unwrap() {
            '+' => Operation::Add(target),
            '*' => Operation::Multiply(target),
            x => panic!("Unrecognized operator {}", x),
        }
    }

    fn apply(&self, worry: u64) -> u64 {
        match &self {
            Operation::Add(target) => match target {
                Target::Old => worry + worry,
                Target::Scalar(n) => worry + n,
            },
            Operation::Multiply(target) => match target {
                Target::Old => worry * worry,
                Target::Scalar(n) => worry * n,
            },
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: RefCell<Vec<u64>>,
    inspections: RefCell<u64>,
    operation: Operation,
    div_test: u64,
    true_monkey: usize,
    false_monkey: usize,
}

impl Monkey {
    fn new(input: &[&str]) -> Monkey {
        let args = input[1..6]
            .iter()
            .map(|l| {
                let pos = l.chars().position(|c| c == ':').unwrap();
                l.split_at(pos + 2).1
            })
            .collect::<Vec<_>>();

        let items = args[0]
            .split(", ")
            .map(|i| i.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let items = RefCell::new(items);

        let op_arr = args[1].split(' ').skip(3).collect::<Vec<_>>();
        let operation = Operation::new(&op_arr);

        let div_test = args[2].rsplit_once(' ').unwrap().1.parse::<u64>().unwrap();

        let true_monkey = args[3]
            .rsplit_once(' ')
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();

        let false_monkey = args[4]
            .rsplit_once(' ')
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();

        Self {
            items,
            operation,
            div_test,
            true_monkey,
            false_monkey,
            inspections: RefCell::new(0),
        }
    }

    fn is_empty(&self) -> bool {
        self.items.borrow().is_empty()
    }

    fn get_worry(&self, chill_out: bool) -> u64 {
        *self.inspections.borrow_mut() += 1;

        let worry = self.items.borrow_mut().remove(0);
        let worry = self.operation.apply(worry);

        if chill_out {
            worry / 3
        } else {
            worry
        }
    }

    fn catch(&self, worry: u64) {
        self.items.borrow_mut().push(worry);
    }

    fn throw(&self, modulo: u64, monkeys: &[Monkey], chill_out: bool) {
        let worry = self.get_worry(chill_out) % modulo;
        if worry % self.div_test == 0 {
            monkeys[self.true_monkey].catch(worry);
        } else {
            monkeys[self.false_monkey].catch(worry);
        }
    }
}

fn parse_input(input: &str) -> (u64, Vec<Monkey>) {
    let ret = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(7)
        .map(Monkey::new)
        .collect::<Vec<_>>();

    let mut modulo = 1;
    for monkey in &ret {
        modulo *= monkey.div_test;
    }

    (modulo, ret)
}

fn next_round(modulo: u64, monkeys: &[Monkey], chill_out: bool) {
    for monkey in monkeys {
        loop {
            if monkey.is_empty() {
                break;
            }

            monkey.throw(modulo, monkeys, chill_out);
        }
    }
}

fn get_most_active(monkeys: &[Monkey], n: usize) -> u64 {
    let mut scores = monkeys.iter().map(|x| &x.inspections).collect::<Vec<_>>();
    scores.sort();
    scores.reverse();

    let mut ret = 1;
    for i in 0..n {
        ret *= *scores[i].borrow();
    }

    ret
}

fn part1() {
    let (modulo, monkeys) = parse_input(INPUT);

    for _ in 0..20 {
        next_round(modulo, &monkeys, true);
    }

    let total = get_most_active(&monkeys, 2);
    println!("The level of monkey business is {}", total);
}

fn part2() {
    let (modulo, monkeys) = parse_input(INPUT);

    for _ in 0..10000 {
        next_round(modulo, &monkeys, false);
    }

    let total = get_most_active(&monkeys, 2);
    println!("The level of monkey business is {}", total);
}
pub fn get_parts() -> DayFunc {
    (part1, part2)
}

#[test]
fn test_part1() {
    let input: &str = include_str!("input/input_day11_sample.txt");
    let (modulo, monkeys) = parse_input(input);

    for _ in 0..20 {
        next_round(modulo, &monkeys, true);
    }

    let total = get_most_active(&monkeys, 2);
    assert_eq!(total, 10605);
}

#[test]
fn test_part2() {
    let input: &str = include_str!("input/input_day11_sample.txt");
    let (modulo, monkeys) = parse_input(input);

    for _ in 0..10000 {
        next_round(modulo, &monkeys, false);
    }

    let total = get_most_active(&monkeys, 2);
    assert_eq!(total, 2713310158);
}
