use crate::util::*;
const INPUT: &str = include_str!("input/input_day6.txt");

fn uniq(arr: &[char]) -> bool {
    for (i, c1) in arr.iter().enumerate() {
        for (j, c2) in arr.iter().enumerate() {
            if i == j {
                continue;
            }

            if c1 == c2 {
                return false;
            }
        }
    }

    true
}

fn find_start_sequence(input: &str, window_size: usize) -> usize {
    if let Some(pos) = input
        .chars()
        .collect::<Vec<char>>()
        .as_slice()
        .windows(window_size)
        .position(uniq)
    {
        pos + window_size
    } else {
        0
    }
}

pub fn part1() {
    let answer = find_start_sequence(INPUT, 4);
    println!("The marker appears after reading {} chars", answer);
}

pub fn part2() {
    let answer = find_start_sequence(INPUT, 14);
    println!("The message appears after reading {} chars", answer);
}

pub struct Day6;
impl Day for Day6 {
    fn get_parts() -> DayFunc {
        (part1, part2)
    }
}

#[test]
fn test_part1() {
    let input: Vec<(&str, usize)> = vec![
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];

    for (code, answer) in input {
        assert_eq!(
            find_start_sequence(code, 4),
            answer,
            "Testing we get the answer {} to code {}",
            answer,
            code
        );
    }
}

#[test]
fn test_part2() {
    let input: Vec<(&str, usize)> = vec![
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
    ];

    for (code, answer) in input {
        assert_eq!(
            find_start_sequence(code, 14),
            answer,
            "Testing we get the answer {} to code {}",
            answer,
            code
        );
    }
}
