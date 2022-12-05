const INPUT: &str = include_str!("input/input_day4.txt");

fn within(p: u32, a: &[u32]) -> bool {
    p >= a[0] && p <= a[1]
}

fn contained(a: &[u32], b: &[u32]) -> bool {
    within(a[0], b) && within(a[1], b) || within(b[0], a) && within(b[1], a)
}

fn overlap(a: &[u32], b: &[u32]) -> bool {
    within(a[0], b) || within(a[1], b) || within(b[0], a) || within(b[1], a)
}

fn test_areas<F>(input: &str, test: F) -> usize
where
    F: Fn(&[u32], &[u32]) -> bool,
{
    input
        .lines()
        .map(|x| {
            x.split(',')
                .map(|y| {
                    y.split('-')
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .filter(|x| test(&x[0], &x[1]))
        .count()
}

pub fn part1() {
    let areas = test_areas(INPUT, contained);
    println!("{} areas are completely covered by one elf", areas);
}

pub fn part2() {
    let areas = test_areas(INPUT, overlap);
    println!("{} areas are partially covered by one elf", areas);
}

#[test]
fn test_part1() {
    let test_input: &str = include_str!("input/input_day4_sample.txt");
    let areas = test_areas(test_input, contained);
    assert_eq!(areas, 2);
}

#[test]
fn test_part2() {
    let test_input: &str = include_str!("input/input_day4_sample.txt");
    let areas = test_areas(test_input, overlap);
    assert_eq!(areas, 4);
}
