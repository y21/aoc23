#![feature(let_chains)]

use std::error::Error;
use std::fs;
use std::time::Instant;

mod day1;
fn main() -> Result<(), Box<dyn Error>> {
    let days = [[day1::part1, day1::part2]];

    println!("Running all solutions");
    for (day_i, day) in days.iter().enumerate() {
        let input = fs::read_to_string(format!("inputs/day{}.txt", day_i + 1))?;
        for (part_i, part) in day.iter().enumerate() {
            let time = Instant::now();
            part(&input);
            let elapsed = time.elapsed();
            println!("Day {day_i} Part {part_i}: {elapsed:?}");
        }
    }

    Ok(())
}
