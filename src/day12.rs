use itertools::Itertools;
use std::fmt::Debug;
use std::fmt::Display;
#[derive(PartialEq, Eq, Copy, Clone)]
enum Spring {
    Broken,
    Unknown,
    Operational,
}

impl Debug for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Broken => write!(f, "#"),
            Self::Unknown => write!(f, "?"),
            Self::Operational => write!(f, "."),
        }
    }
}

impl Display for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Spring {
    pub fn maybe_broken(self) -> bool {
        matches!(self, Self::Unknown | Self::Broken)
    }
}

pub fn part1(input: &str) -> i64 {
    let lines = input.lines().map(|line| {
        let (springs, damaged) = line.split_once(' ').unwrap();
        let springs = springs
            .bytes()
            .map(|b| match b {
                b'#' => Spring::Broken,
                b'.' => Spring::Operational,
                b'?' => Spring::Unknown,
                _ => unreachable!(),
            })
            .collect_vec();
        let damaged = damaged
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect_vec();
        (springs, damaged)
    });

    let mut arrangements = 0;
    for (springs, damaged) in lines {
        let mut queue: Vec<(&[Spring], &[i64])> = vec![(&springs, &damaged)];

        while let Some((springs, damaged)) = queue.pop() {
            for (start, window) in springs.windows(damaged[0] as usize).enumerate() {
                let end = start + damaged[0] as usize - 1;
                let surrounded_by_damaged = start
                    .checked_sub(1)
                    .is_some_and(|i| springs[i] == Spring::Broken)
                    || springs.get(end + 1).is_some_and(|&s| s == Spring::Broken);

                if !surrounded_by_damaged && window.iter().all(|&s| s.maybe_broken()) {
                    if damaged.len() == 1 {
                        // Path can't be used if there are still broken ones, we were too eager
                        if !springs[end + 1..].iter().any(|&v| v == Spring::Broken) {
                            // Last one found.
                            arrangements += 1;
                        }
                    } else if let Some(post) = springs.get(end + 2..) {
                        queue.push((post, &damaged[1..]));
                    }
                }

                if window[0] == Spring::Broken {
                    break;
                }
            }
        }
    }

    arrangements
}

#[allow(clippy::redundant_locals)]
pub fn _part2(_input: &str) -> i64 {
    // TODO: make it run in a reasonable amount of time
    todo!("commit once cleaned up")
}

#[cfg(test)]
#[test]
fn p12t() {
    const INPUT: &str = include_str!("../inputs/day12.txt");
    assert_eq!(part1(INPUT), 7195);
    // assert_eq!(part2(INPUT), 33992866292225);
}
