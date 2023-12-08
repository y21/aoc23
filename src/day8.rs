use std::collections::HashMap;

use num::Integer;
use rustc_hash::FxHashMap;

pub fn part1(input: &str) -> i64 {
    let mut iter = input.split("\n\n");
    let steps = iter.next().unwrap().bytes().cycle();
    let nodes = iter
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let (name, instrs) = line.split_once(" = ").unwrap();
            let instrs = instrs
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split_once(", ")
                .unwrap();
            (name, instrs)
        })
        .collect::<FxHashMap<_, _>>();

    let mut count = 0;

    let mut cur_node = "AAA";
    for step in steps {
        if cur_node == "ZZZ" {
            break;
        }

        let node = &nodes[&cur_node];
        match step {
            b'L' => cur_node = node.0,
            b'R' => cur_node = node.1,
            _ => unreachable!(),
        }
        count += 1;
    }

    count
}

pub fn part2(input: &str) -> i64 {
    let mut iter = input.split("\n\n");
    let steps = iter.next().unwrap().bytes();
    let nodes = iter
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let (name, instrs) = line.split_once(" = ").unwrap();

            let instrs = instrs
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split_once(", ")
                .unwrap();
            (name, instrs)
        })
        .collect::<HashMap<_, _>>();

    let paths = nodes.keys().copied().filter(|v| v.ends_with('A'));

    let mut cycle_counts = Vec::new();
    let mut seen = FxHashMap::default();
    for path in paths {
        let mut node = path;

        for (node_cycle_count, (index, step)) in steps.clone().enumerate().cycle().enumerate() {
            if let Some(index) = seen.get(&(index, step, node)) {
                cycle_counts.push((node_cycle_count - index) as i64);
                break;
            } else {
                seen.insert((index, step, node), node_cycle_count);
            }

            match step {
                b'L' => node = nodes[&node].0,
                b'R' => node = nodes[&node].1,
                _ => unreachable!(),
            }
        }

        seen.clear();
    }

    cycle_counts.into_iter().reduce(|x, y| x.lcm(&y)).unwrap()
}

#[cfg(test)]
#[test]
fn p8t() {
    const INPUT: &str = include_str!("../inputs/day8.txt");
    assert_eq!(part1(INPUT), 21251);
    assert_eq!(part2(INPUT), 11678319315857);
}
