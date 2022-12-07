use aoc_2022::*;

fn main() {
    let days: Vec<util::DayFunc> = vec![
        day1::get_parts(),
        day2::get_parts(),
        day3::get_parts(),
        day4::get_parts(),
        day5::get_parts(),
        day6::get_parts(),
        day7::get_parts(),
    ];

    for (i, day) in days.into_iter().enumerate() {
        println!();
        println!("~~ DAY {} ~~", i + 1);

        day.0();
        day.1();
    }
}
