#![feature(let_chains, coroutines, iter_from_coroutine, iter_next_chunk)]

use std::error::Error;
use std::fs;
use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod grid;

fn main() -> Result<(), Box<dyn Error>> {
    let days = [
        [day1::part1, day1::part2],
        [day2::part1, day2::part2],
        [day3::part1, day3::part2],
        [day4::part1, day4::part2],
        [day5::part1, day5::part2],
        [day6::part1, day6::part2],
    ];

    println!("Running all solutions");
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
        }
    }

    Ok(())
}
