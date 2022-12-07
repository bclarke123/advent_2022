use aoc_2022::{util::*, *};

fn main() {
    let days: Vec<DayFunc> = vec![
        day1::Day1::get_parts(),
        day2::Day2::get_parts(),
        day3::Day3::get_parts(),
        day4::Day4::get_parts(),
        day5::Day5::get_parts(),
        day6::Day6::get_parts(),
    ];

    for (i, day) in days.into_iter().enumerate() {
        println!();
        println!("~~ DAY {} ~~", i + 1);

        day.0();
        day.1();
    }
}
