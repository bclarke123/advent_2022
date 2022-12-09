use crate::util::*;
use std::cmp;
const INPUT: &str = include_str!("input/input_day8.txt");

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn is_visible(x: usize, y: usize, forest: &Vec<Vec<u32>>) -> bool {
    if x == 0 || y == 0 {
        return true;
    }

    let y_len = forest.len();
    let x_len = forest[y].len();

    if x == x_len - 1 || y == y_len - 1 {
        return true;
    }

    let height = forest[y][x];

    let mut visible = true;
    for i in 0..x {
        if forest[y][i] >= height {
            visible = false;
        }
    }

    if visible {
        return true;
    }

    visible = true;
    for i in x + 1..x_len {
        if forest[y][i] >= height {
            visible = false;
        }
    }

    if visible {
        return true;
    }

    visible = true;
    for row in forest.iter().take(y) {
        if row[x] >= height {
            visible = false;
        }
    }

    if visible {
        return true;
    }

    visible = true;
    for row in forest.iter().take(y_len).skip(y + 1) {
        if row[x] >= height {
            visible = false;
        }
    }

    visible
}

fn walk_grid(forest: &Vec<Vec<u32>>) -> u32 {
    let mut ret = 0;

    for y in 0..forest.len() {
        let row = &forest[y];
        for x in 0..row.len() {
            if is_visible(x, y, forest) {
                ret += 1;
            }
        }
    }

    ret
}

fn view_score(x: usize, y: usize, forest: &Vec<Vec<u32>>) -> u32 {
    let height = forest[y][x];
    let y_len = forest.len();
    let x_len = forest[y].len();

    let mut lh_score = 0;
    for i in 1..=x {
        lh_score += 1;

        if forest[y][x - i] >= height {
            break;
        }
    }

    let mut rh_score = 0;
    for i in x + 1..x_len {
        rh_score += 1;

        if forest[y][i] >= height {
            break;
        }
    }

    let mut up_score = 0;
    for i in 1..=y {
        up_score += 1;

        if forest[y - i][x] >= height {
            break;
        }
    }

    let mut dn_score = 0;
    for row in forest.iter().take(y_len).skip(y + 1) {
        dn_score += 1;

        if row[x] >= height {
            break;
        }
    }

    lh_score * rh_score * up_score * dn_score
}

fn walk_scores(forest: &Vec<Vec<u32>>) -> u32 {
    let mut ret = 0;

    for y in 0..forest.len() {
        let row = &forest[y];
        for x in 0..row.len() {
            ret = cmp::max(ret, view_score(x, y, forest));
        }
    }

    ret
}

fn part1() {
    let trees = parse_input(INPUT);
    let answer = walk_grid(&trees);
    println!("The total number of visible trees is {}", answer);
}

fn part2() {
    let trees = parse_input(INPUT);
    let answer = walk_scores(&trees);
    println!("The best visibility score is {}", answer);
}

pub fn get_parts() -> DayFunc {
    (part1, part2)
}

#[test]
fn test_part1() {
    let input: &str = include_str!("input/input_day8_sample.txt");
    let forest = parse_input(input);
    let answer = walk_grid(&forest);
    assert_eq!(answer, 21);
}

#[test]
fn test_part2() {
    let input: &str = include_str!("input/input_day8_sample.txt");
    let forest = parse_input(input);
    let answer = walk_scores(&forest);
    assert_eq!(answer, 8);
}
