use typed_arena::Arena;

pub fn part1(input: &str) -> i64 {
    fn inner_recursive(arena: &Arena<i64>, nums: &[i64]) -> i64 {
        if nums.iter().all(|&v| v == 0) {
            0
        } else {
            let diffs = arena.alloc_extend(nums.windows(2).map(|v| v[1] - v[0]));
            *nums.last().unwrap() + inner_recursive(arena, diffs)
        }
    }

    let arena = Arena::new();
    input
        .lines()
        .map(|line| {
            let nums = arena.alloc_extend(
                line.split_ascii_whitespace()
                    .map(|v| v.parse::<i64>().unwrap()),
            );
            inner_recursive(&arena, nums)
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    fn inner_recursive(arena: &Arena<i64>, nums: &[i64]) -> i64 {
        if nums.iter().all(|&v| v == 0) {
            0
        } else {
            let diffs = arena.alloc_extend(nums.windows(2).map(|v| v[1] - v[0]));
            *nums.first().unwrap() - inner_recursive(arena, diffs)
        }
    }

    let arena = Arena::new();
    input
        .lines()
        .map(|line| {
            let nums = arena.alloc_extend(
                line.split_ascii_whitespace()
                    .map(|v| v.parse::<i64>().unwrap()),
            );
            inner_recursive(&arena, nums)
        })
        .sum()
}

#[cfg(test)]
#[test]
fn p9t() {
    const INPUT: &str = include_str!("../inputs/day9.txt");
    assert_eq!(part1(INPUT.trim()), 2175229206);
    assert_eq!(part2(INPUT.trim()), 942);
}
