use aho_corasick::AhoCorasick;

pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut it = line
                .bytes()
                .filter(|c| c.is_ascii_digit())
                .map(|c| i32::from(c - b'0'));
            let first = it.next().unwrap();
            let last = it.last().unwrap_or(first);

            first * 10 + last
        })
        .sum()
}

pub fn part2(input: &str) -> i32 {
    let ac = AhoCorasick::new([
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1",
        "2", "3", "4", "5", "6", "7", "8", "9",
    ])
    .unwrap();

    input
        .lines()
        .map(|line| {
            let mut index = 0;

            let mut first = None;
            let mut last = None;

            while let Some(seg) = line.get(index..)
                && let Some(m) = ac.find(seg)
            {
                let pattern = m.pattern().as_i32();

                let digit = if (0..=9).contains(&pattern) {
                    // Digit as a word, need to subtract 1 because words can overlap:
                    // oneight
                    // ^^^ we need to check the 'e' again
                    index += m.end() - 1;
                    pattern
                } else {
                    // Digit as a number
                    index += m.end();
                    pattern - 10
                };

                if first.is_none() {
                    first = Some(digit);
                }
                last = Some(digit);
            }

            let first = first.unwrap();
            first * 10 + last.unwrap_or(first)
        })
        .sum()
}

#[cfg(test)]
#[test]
fn p1t() {
    const INPUT: &str = include_str!("../inputs/day1.txt");
    assert_eq!(part1(INPUT), 54927);

    let example: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
        .trim();

    assert_eq!(part2(example), 281);
    assert_eq!(part2(INPUT), 54581);
}
