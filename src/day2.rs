#[derive(Default, Debug)]
struct Bag {
    red: i32,
    green: i32,
    blue: i32,
}

struct BagIterator<'a>(&'a str);

impl<'a> Iterator for BagIterator<'a> {
    type Item = Bag;
    fn next(&mut self) -> Option<Self::Item> {
        let (bag, rest) = self.0.split_once("; ").unwrap_or((self.0, ""));
        self.0 = rest;

        bag.split(", ").try_fold(Bag::default(), |bag, item| {
            let (count, color) = item.split_once(' ')?;
            let count = count.parse::<i32>().ok()?;

            match color {
                "red" => Some(Bag {
                    red: bag.red + count,
                    ..bag
                }),
                "green" => Some(Bag {
                    green: bag.green + count,
                    ..bag
                }),
                "blue" => Some(Bag {
                    blue: bag.blue + count,
                    ..bag
                }),
                _ => None,
            }
        })
    }
}

pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|line| {
            let line = line.strip_prefix("Game ").unwrap();
            let (gid, line) = line.split_once(": ").unwrap();
            let gid = gid.parse::<i32>().unwrap();

            BagIterator(line)
                .all(|bag| bag.red <= 12 && bag.green <= 13 && bag.blue <= 14)
                .then_some(gid)
        })
        .sum()
}

pub fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let line = line
                .trim_start_matches("Game ")
                .trim_start_matches(|c: char| c.is_ascii_digit())
                .trim_start_matches(": ");

            let Bag { red, green, blue } =
                BagIterator(line).fold(Bag::default(), |bag, item| Bag {
                    red: bag.red.max(item.red),
                    green: bag.green.max(item.green),
                    blue: bag.blue.max(item.blue),
                });

            red * green * blue
        })
        .sum()
}

#[cfg(test)]
#[test]
fn p2t() {
    const INPUT: &str = include_str!("../inputs/day2.txt");
    let inp = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"
    .trim();
    assert_eq!(part1(INPUT), 2006);
    assert_eq!(part1(inp), 8);
    assert_eq!(part2(INPUT), 84911);
    assert_eq!(part2(inp), 2286);
}
