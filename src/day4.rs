use itertools::Itertools;
use typed_arena::Arena;

fn count_winning_parts(winning_numbers: &[i32], my_numbers: &[i32]) -> usize {
    my_numbers
        .iter()
        .filter(|&num| winning_numbers.iter().any(|w| w == num))
        .count()
}

fn parse_nums<'a>(arena: &'a Arena<i32>, s: &str) -> &'a [i32] {
    arena.alloc_extend(
        s.split_ascii_whitespace()
            .map(|v| v.parse::<i32>().unwrap()),
    )
}

pub fn part1(input: &str) -> i32 {
    let arena = Arena::new();
    input
        .lines()
        .map(|line| {
            let (_, wnums, mnums) = line
                .split(|c| c == '|' || c == ':')
                .collect_tuple()
                .unwrap();

            (parse_nums(&arena, wnums), parse_nums(&arena, mnums))
        })
        .map(|(wnums, mnums)| {
            count_winning_parts(wnums, mnums)
                .checked_sub(1)
                .map(|v| 1 << v)
                .unwrap_or_default()
        })
        .sum()
}

pub fn part2(input: &str) -> i32 {
    let arena = Arena::new();

    let cards = input
        .lines()
        .map(|line| {
            let (_, wnums, mnums) = line
                .split(|c| c == '|' || c == ':')
                .collect_tuple()
                .unwrap();

            (parse_nums(&arena, wnums), parse_nums(&arena, mnums))
        })
        .collect_vec();

    let mut counts = vec![1; cards.len()];
    for (index, &(wnums, mnums)) in cards.iter().enumerate() {
        let matching = count_winning_parts(wnums, mnums);
        for n in index + 1..index + matching + 1 {
            counts[n] += counts[index];
        }
    }

    counts.iter().sum()
}

#[cfg(test)]
#[test]
fn p4t() {
    const INPUT: &str = include_str!("../inputs/day4.txt");
    let inp = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"
    .trim();

    assert_eq!(part1(inp), 13);
    assert_eq!(part1(INPUT), 27454);
    assert_eq!(part2(inp), 30);
    assert_eq!(part2(INPUT), 6857330);
}
