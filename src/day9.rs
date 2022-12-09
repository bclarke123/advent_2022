use crate::util::*;
const INPUT: &str = include_str!("input/input_day9.txt");

fn parse_input(input: &str) -> Vec<(char, i32)> {
    input
        .lines()
        .map(|l| {
            let mut words = l.split(' ');
            (
                words.next().unwrap().chars().next().unwrap(),
                words.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

fn walk_steps(input: &str, rope_len: usize) -> usize {
    let steps = parse_input(input);
    let mut rope = vec![(0i32, 0i32); rope_len];
    let mut buf: Vec<(i32, i32)> = vec![];

    for (dir, amt) in steps {
        for _ in 0..amt {
            match dir {
                'U' => rope[0].1 += 1,
                'D' => rope[0].1 -= 1,
                'L' => rope[0].0 -= 1,
                'R' => rope[0].0 += 1,
                _ => panic!("Bad input"),
            }

            for i in 1..rope_len {
                let prev = rope[i - 1];
                let curr = rope[i];

                let dx = prev.0 - curr.0;
                let adx = dx.abs();

                let dy = prev.1 - curr.1;
                let ady = dy.abs();

                if dx.abs() > 1 || dy.abs() > 1 {
                    let xdir = match dx {
                        x if x > 0 => 1,
                        _ => -1,
                    };

                    let ydir = match dy {
                        y if y > 0 => 1,
                        _ => -1,
                    };

                    if adx != 0 {
                        rope[i].0 += xdir;
                    }

                    if ady != 0 {
                        rope[i].1 += ydir;
                    }
                }
            }

            let tail = rope[rope_len - 1];

            if !buf.contains(&tail) {
                buf.push(tail.clone());
            }
        }
    }

    buf.len()
}

fn part1() {
    let answer = walk_steps(INPUT, 2);
    println!("The first rope tail visited {} unique spots", answer);
}

fn part2() {
    let answer = walk_steps(INPUT, 10);
    println!("The second rope tail visited {} unique spots", answer);
}

pub fn get_parts() -> DayFunc {
    (part1, part2)
}

#[test]
fn test_part1() {
    let input: &str = include_str!("input/input_day9_sample.txt");
    let answer = walk_steps(input, 2);

    assert_eq!(answer, 13);
}

#[test]
fn test_part2() {
    let input: &str = include_str!("input/input_day9_sample2.txt");
    let answer = walk_steps(input, 10);

    assert_eq!(answer, 36);
}
