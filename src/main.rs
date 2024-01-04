#![feature(
    let_chains,
    coroutines,
    iter_from_coroutine,
    iter_next_chunk,
    lint_reasons
)]

use std::error::Error;
use std::fs;
use std::time::Duration;
use std::time::Instant;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod grid;

fn main() -> Result<(), Box<dyn Error>> {
    let days = [
        [day1::part1, day1::part2],
        [day2::part1, day2::part2],
        [day3::part1, day3::part2],
        [day4::part1, day4::part2],
        [day5::part1, day5::part2],
        [day6::part1, day6::part2],
        [day7::part1, day7::part2],
        [day8::part1, day8::part2],
        [day9::part1, day9::part2],
        [day10::part1, day10::part2],
        [day11::part1, day11::part2],
        [day12::part1, |_| 0 /* todo */],
        [day13::part1, day13::part2],
        [day14::part1, |_| 0 /* todo */],
        [day15::part1, day15::part2],
        [day16::part1, day16::part2],
        [day17::part1, day17::part2],
        [day18::part1, |_| 0 /* todo */],
        [day19::part1, day19::part2],
    ];

    println!("Running all solutions");
    let mut total = Duration::ZERO;
    for (day_n, day) in days.iter().enumerate() {
        let day_n = day_n + 1;

        let input = fs::read_to_string(format!("inputs/day{day_n}.txt"))?;
        let input = input.trim_end();
        for (part_n, part) in day.iter().enumerate() {
            let part_n = part_n + 1;
            let time = Instant::now();
            part(input);
            let elapsed = time.elapsed();
            println!("Day {day_n} Part {part_n}: {elapsed:?}");
            total += elapsed;
        }
    }
    println!("Total: {total:?}");

    Ok(())
}
