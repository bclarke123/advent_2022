const INPUT: &str = include_str!("input_day3.txt");

fn char_val(c: char) -> u32 {
    let c_val: u32 = c.into();
    match c_val {
        x if x > 64 && x < 91 => x - 38,
        x => x - 96,
    }
}

fn shared_char(lines: Vec<&str>) -> u32 {
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

pub fn part1() {
    let lines = INPUT
        .split('\n')
        .map(|x| x.trim().split_at(x.len() / 2))
        .collect::<Vec<(&str, &str)>>();

    let mut result: u32 = 0;

    for line in lines {
        let arr = vec![line.0, line.1];
        result += shared_char(arr);
    }

    println!("The sum of values is {}", result);
}

pub fn part2() {
    let lines = INPUT.split('\n').map(|x| x.trim()).collect::<Vec<&str>>();
    let teams = lines[..].chunks(3);

    let mut result: u32 = 0;

    for team in teams {
        let arr = team.to_vec();
        result += shared_char(arr);
    }

    println!("The team badge is {}", result);
}
