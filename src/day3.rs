const INPUT: &str = include_str!("input_day3.txt");

fn char_val(c: char) -> u32 {
    let c_val: u32 = c.into();
    if c_val > 64 && c_val < 91 {
        c_val - 38
    } else {
        c_val - 96
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

fn do_part1(input: &str) -> u32 {
    let lines = input
        .lines()
        .map(|x| x.split_at(x.len() / 2))
        .collect::<Vec<(&str, &str)>>();

    let mut result: u32 = 0;

    for line in lines {
        let arr = vec![line.0, line.1];
        result += shared_char(arr);
    }

    result
}

pub fn part1() {
    let result = do_part1(INPUT);
    println!("The sum of values is {}", result);
}

fn do_part2(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();
    let teams = lines[..].chunks(3);

    let mut result: u32 = 0;

    for team in teams {
        let arr = team.to_vec();
        result += shared_char(arr);
    }

    result
}

pub fn part2() {
    let result = do_part2(INPUT);
    println!("The team badge is {}", result);
}

#[test]
fn test_part1() {
    let test_input: &str = include_str!("input_day3_sample.txt");
    let result = do_part1(test_input);
    assert!(result == 157);
}

#[test]
fn test_part2() {
    let test_input: &str = include_str!("input_day3_sample.txt");
    let result = do_part2(test_input);
    assert!(result == 70);
}
