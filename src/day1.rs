const INPUT: &str = include_str!("input_day1.txt");

fn get_carry_amounts(buf: &mut Vec<usize>) {
    let lines = INPUT.lines().collect::<Vec<&str>>();

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

fn tally(n_elves: usize) -> usize {
    let mut buf: Vec<usize> = vec![];

    get_carry_amounts(&mut buf);
    total_carry(&buf, n_elves)
}

pub fn part1() {
    let max = tally(1);
    println!("The top elf is carrying {}", max);
}

pub fn part2() {
    let max = tally(3);
    println!("The top three elves are carrying {}", max);
}
