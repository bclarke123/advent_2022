use crate::util::*;
const INPUT: &str = include_str!("input/input_day10.txt");

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    fn parse(input: &str) -> Self {
        let instr = &input[0..4];
        match instr {
            "addx" => {
                let idx = input.find(|c| c == ' ').unwrap();
                let num = input[idx + 1..].parse::<i32>().unwrap();
                Instruction::AddX(num)
            }
            _ => Instruction::Noop,
        }
    }
}

struct Display {
    pixels: Vec<bool>,
    sprite: i32,
    cursor: i32,
    signal_strength: i32,
}

impl Display {
    fn new() -> Self {
        Self {
            pixels: vec![false; 240],
            sprite: 1,
            cursor: 0,
            signal_strength: 0,
        }
    }

    fn check_signal(&self) -> i32 {
        if self.cursor % 40 == 20 {
            self.cursor * self.sprite
        } else {
            0
        }
    }

    fn tick(&mut self) {
        let pixel = (self.cursor % 40) - self.sprite;
        let cursor_index: usize = self.cursor.try_into().unwrap();
        let lit = (-1..=1).contains(&pixel);

        self.pixels[cursor_index] = lit;
        self.cursor += 1;

        self.signal_strength += self.check_signal();
    }

    fn move_sprite(&mut self, amt: i32) {
        self.sprite += amt;
    }
}

fn get_pattern(display: Display, buf: &mut Vec<String>) {
    display
        .pixels
        .chunks(40)
        .map(|r| r.iter().map(|b| if *b { '#' } else { '.' }))
        .for_each(|r| buf.push(String::from_iter(r)));
}

fn draw_pattern(display: Display) {
    let mut buf = vec![];
    get_pattern(display, &mut buf);
    println!("{}", buf.join("\n"));
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::parse).collect::<Vec<_>>()
}

fn execute_instructions(display: &mut Display, instructions: &Vec<Instruction>) {
    for instr in instructions {
        display.tick();
        match instr {
            Instruction::AddX(n) => {
                display.tick();
                display.move_sprite(*n);
            }
            Instruction::Noop => {}
        }
    }
}

fn part1() {
    let instructions = parse_input(INPUT);
    let mut display = Display::new();
    execute_instructions(&mut display, &instructions);
    let answer = display.signal_strength;

    println!("The signal strength sum is {}", answer);
}

fn part2() {
    let instructions = parse_input(INPUT);
    let mut display = Display::new();
    execute_instructions(&mut display, &instructions);
    draw_pattern(display);
}

pub fn get_parts() -> DayFunc {
    (part1, part2)
}

#[test]
fn test_part1() {
    let input: &str = include_str!("input/input_day10_sample.txt");
    let instructions = parse_input(input);
    let mut display = Display::new();
    execute_instructions(&mut display, &instructions);
    let answer = display.signal_strength;

    assert_eq!(answer, 13140);
}

#[test]
fn test_part2() {
    let input: &str = include_str!("input/input_day10_sample.txt");
    let instructions = parse_input(input);
    let mut display = Display::new();
    let mut buf = vec![];

    execute_instructions(&mut display, &instructions);
    get_pattern(display, &mut buf);

    let test_data: [&str; 6] = [
        "##..##..##..##..##..##..##..##..##..##..",
        "###...###...###...###...###...###...###.",
        "####....####....####....####....####....",
        "#####.....#####.....#####.....#####.....",
        "######......######......######......####",
        "#######.......#######.......#######.....",
    ];

    for (i, line) in buf.iter().enumerate() {
        assert_eq!(line, test_data[i]);
    }
}
