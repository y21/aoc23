use std::cmp::Ordering;

use aoc::MoreItertools;

fn race(time: i64, time_to_hold: i64) -> i64 {
    time_to_hold * (time - time_to_hold)
}

pub fn part1(input: &str) -> i64 {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|v| v.parse::<i64>().unwrap());

    let distance = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|v| v.parse::<i64>().unwrap());

    times
        .into_iter()
        .zip(distance)
        .map(|(time, record)| {
            i64::try_from(
                (0..=time)
                    .filter(|&time_to_hold| race(time, time_to_hold) > record)
                    .count(),
            )
            .unwrap()
        })
        .product()
}

pub fn part2(input: &str) -> i64 {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .bytes()
        .skip("Time:".len())
        .filter(|b| !b.is_ascii_whitespace())
        .parse_int();

    let record = lines
        .next()
        .unwrap()
        .bytes()
        .skip("Distance:".len())
        .filter(|b| !b.is_ascii_whitespace())
        .parse_int();

    // Find the point at which our distance becomes >= record
    let smallest_time_pos = (0..)
        .position(|time_to_hold| race(time, time_to_hold) > record)
        .unwrap() as i64;

    // Everything after the point at which the distance goes down again is "mirrored".
    // The point at which the distance goes down is always the middle.
    // So, the count is `(smallest point where distance >= record..=middle) * 2`
    match race(time, time / 2).cmp(&race(time, time / 2 + 1)) {
        Ordering::Greater => ((time / 2) - smallest_time_pos + 1) * 2 - 1,
        Ordering::Equal => ((time / 2) - smallest_time_pos + 1) * 2,
        Ordering::Less => unreachable!(),
    }
}

#[cfg(test)]
#[test]
fn p6t() {
    const INPUT: &str = include_str!("../inputs/day6.txt");
    let example = "Time:      7  15   30
Distance:  9  40  200"
        .trim();

    assert_eq!(part1(example), 288);
    assert_eq!(part1(INPUT), 128700);
    assert_eq!(part2(example), 71503);
    assert_eq!(part2(INPUT), 39594072);
}
