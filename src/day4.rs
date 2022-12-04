const INPUT: &str = include_str!("input_day4.txt");

fn contained(a: &[u32], b: &[u32]) -> bool {
    a[0] <= b[0] && a[1] >= b[1] || b[0] <= a[0] && b[1] >= a[1]
}

fn overlap(a: &[u32], b: &[u32]) -> bool {
    let within =
        |a: &[u32], b: &[u32]| a[0] >= b[0] && a[0] <= b[1] || a[1] >= b[0] && a[1] <= b[1];
    within(a, b) || within(b, a)
}

fn test_areas<F>(test: F) -> usize
where
    F: Fn(&[u32], &[u32]) -> bool,
{
    INPUT
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
    let areas = test_areas(contained);
    println!("{} areas are completely covered by one elf", areas);
}

pub fn part2() {
    let areas = test_areas(overlap);
    println!("{} areas are partially covered by one elf", areas);
}
