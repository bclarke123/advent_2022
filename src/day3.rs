const INPUT: &str = include_str!("input_day3.txt");

fn char_val(c: char) -> u32 {
    let c_val: u32 = c.into();
    match c_val {
        x if x > 64 && x < 91 => x - 38,
        x => x - 96,
    }
}

pub fn part1() {
    let lines = INPUT
        .split('\n')
        .map(|x| x.trim().split_at(x.len() / 2))
        .collect::<Vec<(&str, &str)>>();

    let mut result: u32 = 0;

    for line in lines {
        let ch1 = line.0.chars();

        for c in ch1 {
            if line.1.contains(c) {
                let value: u32 = char_val(c);
                result += value;
                break;
            }
        }
    }

    println!("The sum of values is {}", result);
}

pub fn part2() {
    let lines = INPUT.split('\n').map(|x| x.trim()).collect::<Vec<&str>>();
    let teams = lines[..].chunks(3);

    let mut result: u32 = 0;

    for team in teams {
        let ch1 = team[0].chars();
        for c in ch1 {
            if team[1].contains(c) && team[2].contains(c) {
                let value: u32 = char_val(c);
                result += value;
                break;
            }
        }
    }

    println!("The team badge is {}", result);
}
