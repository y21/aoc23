use std::ops::RangeInclusive;

use itertools::Itertools;

#[derive(Debug)]
struct ConversionSection {
    source_range: RangeInclusive<i64>,
    dest_range: RangeInclusive<i64>,
}

fn parse_section(s: &str) -> Vec<ConversionSection> {
    s.lines()
        .skip(1)
        .map(|v| {
            let (dest_range_start, source_range_start, range_len) = v
                .split_ascii_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap();

            ConversionSection {
                dest_range: dest_range_start..=dest_range_start + range_len - 1,
                source_range: source_range_start..=source_range_start + range_len - 1,
            }
        })
        .collect_vec()
}

pub fn part1(input: &str) -> i64 {
    fn find_dest(num: i64, group: &[ConversionSection]) -> i64 {
        group
            .iter()
            .find(|s| s.source_range.contains(&num))
            .map(|s| num + (s.dest_range.start() - s.source_range.start()))
            .unwrap_or(num)
    }

    let mut groups_iter = input.split("\n\n");

    let seeds = groups_iter
        .next()
        .unwrap()
        .split(|c| c == ':' || c == ' ')
        .skip(2)
        .map(|v| v.parse::<i64>().unwrap())
        .collect_vec();

    let maps = groups_iter.map(parse_section).next_chunk::<7>().unwrap();

    seeds
        .into_iter()
        .map(|seed| maps.iter().fold(seed, |input, map| find_dest(input, map)))
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> i64 {
    fn get_outputs(
        input: RangeInclusive<i64>,
        map: &[ConversionSection],
    ) -> Vec<RangeInclusive<i64>> {
        let mut mapped = vec![];
        let mut unmapped = vec![];

        for ConversionSection {
            source_range,
            dest_range,
        } in map
        {
            let start = (*source_range.start()).max(*input.start());
            let end = (*source_range.end()).min(*input.end());
            let shift = dest_range.start() - source_range.start();

            if start < end {
                mapped.push((start..=end, shift));
            }
        }
        mapped.sort_by(|(r1, _), (r2, _)| r1.start().cmp(r2.start()));

        if let Some((first, _)) = mapped.first() {
            if first.start() != input.start() {
                unmapped.push((*input.start())..=(*first.start()));
            }

            let mut cur = *(first.start());
            for (m, _) in mapped.iter() {
                if cur != *m.start() {
                    unmapped.push(cur..=*m.end());
                }
                cur = *m.end() + 1;
            }

            if cur < *input.end() {
                unmapped.push(cur..=*input.end());
            }
        } else {
            unmapped.push(*input.start()..=*input.end());
        }

        unmapped
            .into_iter()
            .map(|v| (v, 0))
            .chain(mapped)
            .map(|(v, shift)| v.start() + shift..=v.end() + shift)
            .filter(|v| v.start() < v.end())
            .sorted_by(|a, b| a.start().cmp(b.start()))
            .dedup()
            .collect_vec()
    }

    let mut groups_iter = input.split("\n\n");

    let seeds = groups_iter
        .next()
        .unwrap()
        .split(|c| c == ':' || c == ' ')
        .skip(2)
        .map(|v| v.parse::<i64>().unwrap())
        .chunks(2)
        .into_iter()
        .map(|chunks| {
            let (s, len) = chunks.into_iter().collect_tuple().unwrap();
            s..=s + len - 1
        })
        .collect_vec();

    let maps = groups_iter.map(parse_section).next_chunk::<7>().unwrap();

    maps.iter()
        .fold(seeds, |v, s| {
            v.into_iter().fold(Vec::new(), |mut v, input| {
                v.append(&mut get_outputs(input, s));
                v
            })
        })
        .into_iter()
        .map(|v| *v.start())
        .min()
        .unwrap()
}

#[cfg(test)]
#[test]
fn p5t() {
    const INPUT: &str = include_str!("../inputs/day5.txt");
    let example = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
        .trim();

    assert_eq!(part1(example), 35);
    assert_eq!(part1(INPUT), 111627841);
    assert_eq!(part2(example), 46);
    assert_eq!(part2(INPUT), 69323688);
}
