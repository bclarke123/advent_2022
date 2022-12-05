const INPUT: &str = include_str!("input/input_day2.txt");

fn play_for_str(s: &str) -> i32 {
    match s.chars().next().unwrap() {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        _ => 3,
    }
}

fn guess_for_play(your_hand: i32, outcome: &str) -> i32 {
    match (your_hand, outcome.chars().next().unwrap()) {
        (x, 'Y') => x,
        (1, 'X') => 3,
        (1, 'Z') => 2,
        (2, 'X') => 1,
        (2, 'Z') => 3,
        (3, 'X') => 2,
        (3, 'Z') => 1,
        _ => panic!(),
    }
}

fn bonus(your_hand: i32, my_hand: i32) -> i32 {
    match (your_hand, my_hand) {
        (x, y) if x == y => 3,
        (1, 2) => 6,
        (1, 3) => 0,
        (2, 3) => 6,
        (2, 1) => 0,
        (3, 1) => 6,
        (3, 2) => 0,
        _ => panic!(),
    }
}

fn play_p1(your_guess: &str, my_guess: &str) -> i32 {
    let your_hand = play_for_str(your_guess);
    let my_hand = play_for_str(my_guess);
    let bonus = bonus(your_hand, my_hand);
    my_hand + bonus
}

fn play_p2(your_guess: &str, my_guess: &str) -> i32 {
    let your_hand = play_for_str(your_guess);
    let my_hand = guess_for_play(your_hand, my_guess);
    let bonus = bonus(your_hand, my_hand);
    my_hand + bonus
}

fn tally<F>(input: &str, cb: F) -> i32
where
    F: Fn(&str, &str) -> i32,
{
    input
        .lines()
        .map(|l| {
            let g = l.split(' ').collect::<Vec<_>>();
            cb(g[0], g[1])
        })
        .sum()
}

pub fn part1() {
    let score = tally(INPUT, play_p1);
    println!("Total P1 Rock Paper Scissors score: {}", score);
}

pub fn part2() {
    let score = tally(INPUT, play_p2);
    println!("Total P2 Rock Paper Scissors score: {}", score);
}

#[test]
fn test_part1() {
    let test_input: &str = include_str!("input/input_day2_sample.txt");
    let score = tally(test_input, play_p1);
    assert_eq!(score, 15);
}

#[test]
fn test_part2() {
    let test_input: &str = include_str!("input/input_day2_sample.txt");
    let score = tally(test_input, play_p2);
    assert_eq!(score, 12);
}
