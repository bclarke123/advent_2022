use crate::util::*;
const INPUT: &str = include_str!("input/input_day7.txt");

#[derive(Debug)]
struct Fs {
    dirs: Vec<Fs>,
    files: Vec<(String, u32)>,
}

impl Fs {
    fn new(input: &mut Vec<&str>) -> Self {
        let dirs = vec![];
        let files = vec![];
        let mut ret = Self { dirs, files };

        ret.parse_input(input);

        ret
    }
    fn parse_input(&mut self, input: &mut Vec<&str>) {
        input.pop(); // "My" line

        loop {
            if input.is_empty() {
                return;
            }

            let next = input.last().unwrap();
            if next.starts_with("$ cd ..") {
                input.pop();
                return;
            }

            if next.starts_with("$ ls") || next.starts_with("dir ") {
                input.pop();
                continue;
            }

            if next.starts_with("$ cd") {
                self.add_subdir(input);
                continue;
            }

            self.add_file(next);
            input.pop();
        }
    }
    fn add_subdir(&mut self, input: &mut Vec<&str>) {
        let sub = Fs::new(input);
        self.dirs.push(sub);
    }
    fn add_file(&mut self, file: &str) {
        let space = file.chars().position(|x| x == ' ');
        let (size, name) = file.split_at(space.unwrap());
        let file = (name.trim().to_owned(), size.parse::<u32>().unwrap());
        self.files.push(file)
    }
    fn size(&self) -> u32 {
        let dir_sum: u32 = self.dirs.iter().map(|x| x.size()).sum();
        let file_sum: u32 = self.files.iter().map(|x| x.1).sum();
        dir_sum + file_sum
    }
    fn find_part1(&self) -> u32 {
        let mut total = 0;
        let size = self.size();
        if size < 100000 {
            total += size;
        }

        total + self.dirs.iter().map(|x| x.find_part1()).sum::<u32>()
    }
    fn find_part2(&self, min_size: u32, buf: &mut Vec<u32>) {
        let size = self.size();

        if size >= min_size {
            buf.push(size);
        }

        self.dirs.iter().for_each(|x| x.find_part2(min_size, buf));
    }
}

fn parse_input(input: &str) -> Fs {
    let mut lines = input.lines().rev().collect::<Vec<&str>>();
    Fs::new(&mut lines)
}

fn get_free_amount(root: &Fs, min_free: u32, fs_size: u32) -> u32 {
    let used = root.size();
    let free = fs_size - used;
    min_free - free
}

pub fn part1() {
    let root = parse_input(INPUT);
    let answer = root.find_part1();
    println!("The total directory size is {}", answer);
}

pub fn part2() {
    let root = parse_input(INPUT);
    let to_free = get_free_amount(&root, 30000000, 70000000);
    let mut buf: Vec<u32> = vec![];

    root.find_part2(to_free, &mut buf);
    buf.sort();

    let answer = buf.first().unwrap();
    println!("The smallest directory that could be deleted is {}", answer);
}

pub fn get_parts() -> DayFunc {
    (part1, part2)
}

#[test]
fn test_part1() {
    let input: &str = include_str!("input/input_day7_sample.txt");
    let root = parse_input(input);
    let answer = root.find_part1();
    assert_eq!(answer, 95437);
}

#[test]
fn test_part2() {
    let input: &str = include_str!("input/input_day7_sample.txt");
    let root = parse_input(input);
    let to_free = get_free_amount(&root, 30000000, 70000000);
    let mut buf: Vec<u32> = vec![];

    root.find_part2(to_free, &mut buf);
    buf.sort();

    let answer = *buf.first().unwrap();
    assert_eq!(to_free, 8381165);
    assert_eq!(answer, 24933642);
}
