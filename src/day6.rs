use std::cmp::Ordering;

use aoc::MoreItertools;

pub fn part1(input: &str) -> i32 {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|v| v.parse::<i32>().unwrap());

    let distance = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|v| v.parse::<i32>().unwrap());

    times
        .into_iter()
        .zip(distance)
        .map(|(time, record)| {
            i32::try_from(
                (0..=time)
                    .filter(|&time_to_hold| time_to_hold * (time - time_to_hold) > record)
                    .count(),
            )
            .unwrap()
        })
        .product()
}

pub fn part2(input: &str) -> i32 {
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
    let smallest_time = (0..)
        .find(|time_to_hold| time_to_hold * (time - time_to_hold) > record)
        .unwrap();

    // Find how many elements there are until we go down
    let mut last = i64::MIN;
    let mut count = 0;
    for time_to_hold in smallest_time.. {
        let distance = time_to_hold * (time - time_to_hold);
        match distance.cmp(&last) {
            Ordering::Equal => return count * 2,
            Ordering::Less => return count * 2 - 1,
            Ordering::Greater => {
                last = distance;
                count += 1;
            }
        }
    }
    unreachable!()
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
