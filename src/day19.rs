use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::fmt::Debug;

#[derive(Debug, Copy, Clone)]
enum Op {
    Lt,
    Gt,
}

#[derive(Debug, Copy, Clone)]
enum Workflow<'a> {
    Path(&'a str),
    Branch {
        lhs: &'a str,
        op: Op,
        rhs: i32,
        target: &'a str,
    },
}

#[derive(Copy, Clone)]
struct IRange<I> {
    min: I,
    max: I,
}

impl IRange<i32> {
    pub fn values_contained(self) -> i32 {
        (self.max + 1) - self.min
    }
}

impl<I: Debug> Debug for IRange<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}..={:?}", self.min, self.max)
    }
}

#[derive(Debug, Copy, Clone)]
struct Costs {
    x: IRange<i32>,
    m: IRange<i32>,
    a: IRange<i32>,
    s: IRange<i32>,
}

impl Costs {
    pub fn range_mut(&mut self, s: &str) -> &mut IRange<i32> {
        match s {
            "x" => &mut self.x,
            "m" => &mut self.m,
            "a" => &mut self.a,
            "s" => &mut self.s,
            _ => unreachable!(),
        }
    }
}

pub fn part1(input: &str) -> i64 {
    let mut workflow_map = FxHashMap::default();

    let (workflows, parts) = input.split_once("\n\n").unwrap();
    for workflow in workflows.lines() {
        let (name, path_input) = workflow[..workflow.len() - 1].split_once('{').unwrap();
        let mut paths = Vec::new();
        for path in path_input.split(',') {
            if let Some((cond, target)) = path.split_once(':') {
                let mut chars = cond[1..].chars();
                let op = match chars.next() {
                    Some('<') => Op::Lt,
                    Some('>') => Op::Gt,
                    _ => unreachable!(),
                };
                paths.push(Workflow::Branch {
                    lhs: &cond[0..1],
                    rhs: chars.as_str().parse::<i32>().unwrap(),
                    target,
                    op,
                });
            } else {
                paths.push(Workflow::Path(path));
            }
        }
        workflow_map.insert(name, paths);
    }

    let mut sum = 0;
    for part in parts.lines() {
        let (x, m, a, s) = part[1..part.len() - 1]
            .split(',')
            .map(|v| v[2..].parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();

        let mut key = "in";
        while key != "R" && key != "A" {
            let paths = &workflow_map[&key];
            let target = paths
                .iter()
                .copied()
                .find_map(|w| match w {
                    Workflow::Path(p) => Some(p),
                    Workflow::Branch {
                        lhs,
                        op,
                        rhs,
                        target,
                    } => match (lhs, op) {
                        ("x", Op::Lt) if x < rhs => Some(target),
                        ("x", Op::Gt) if x > rhs => Some(target),
                        ("m", Op::Lt) if m < rhs => Some(target),
                        ("m", Op::Gt) if m > rhs => Some(target),
                        ("a", Op::Lt) if a < rhs => Some(target),
                        ("a", Op::Gt) if a > rhs => Some(target),
                        ("s", Op::Lt) if s < rhs => Some(target),
                        ("s", Op::Gt) if s > rhs => Some(target),
                        _ => None,
                    },
                })
                .unwrap();
            key = target;
        }

        if key == "A" {
            sum += x + m + a + s;
        }
    }
    sum.into()
}

pub fn part2(input: &str) -> i64 {
    let mut workflow_map = FxHashMap::default();

    let (workflows, _) = input.split_once("\n\n").unwrap();
    for workflow in workflows.lines() {
        let (name, path_input) = workflow[..workflow.len() - 1].split_once('{').unwrap();
        let mut paths = Vec::new();
        for path in path_input.split(',') {
            if let Some((cond, target)) = path.split_once(':') {
                let mut chars = cond[1..].chars();
                let op = match chars.next() {
                    Some('<') => Op::Lt,
                    Some('>') => Op::Gt,
                    _ => unreachable!(),
                };
                paths.push(Workflow::Branch {
                    lhs: &cond[0..1],
                    rhs: chars.as_str().parse::<i32>().unwrap(),
                    target,
                    op,
                });
            } else {
                paths.push(Workflow::Path(path));
            }
        }
        workflow_map.insert(name, paths);
    }

    fn find_accepted_costs<'wf>(
        wfmap: &FxHashMap<&'wf str, Vec<Workflow<'wf>>>,
        wf: &'wf str,
        mut costs: Costs,
        final_costs: &mut Vec<Costs>,
        parent: Option<&dyn Debug>,
    ) {
        if wf == "A" {
            final_costs.push(costs);
            return;
        } else if wf == "R" {
            return;
        }
        let paths = &wfmap[&wf];
        for path in paths {
            match *path {
                Workflow::Path(p) => {
                    find_accepted_costs(wfmap, p, costs, final_costs, Some(&(wf, parent)));
                }
                Workflow::Branch {
                    lhs,
                    op,
                    rhs,
                    target,
                } => {
                    let mut rec_costs = costs;
                    match op {
                        Op::Lt => {
                            // x < 5 -> ZZZ
                            // checking ZZZ, x: ..=4
                            // checking others, x: 5..
                            let range = costs.range_mut(lhs);
                            range.min = rhs;

                            let rec_range = rec_costs.range_mut(lhs);
                            rec_range.max = rhs - 1;
                        }
                        Op::Gt => {
                            // x > 5 -> ZZZ
                            // checking ZZZ, x: 6..
                            // checking others, x: ..=5
                            let range = costs.range_mut(lhs);
                            range.max = rhs;

                            let rec_range = rec_costs.range_mut(lhs);
                            rec_range.min = rhs + 1;
                        }
                    }
                    find_accepted_costs(wfmap, target, rec_costs, final_costs, Some(&(wf, parent)));
                }
            }
        }
    }

    let mut final_costs = Vec::new();
    find_accepted_costs(
        &workflow_map,
        "in",
        Costs {
            x: IRange { min: 1, max: 4000 },
            m: IRange { min: 1, max: 4000 },
            a: IRange { min: 1, max: 4000 },
            s: IRange { min: 1, max: 4000 },
        },
        &mut final_costs,
        None,
    );

    final_costs.iter().fold(0, |p, c| {
        p + (c.x.values_contained() as i64
            * c.m.values_contained() as i64
            * c.a.values_contained() as i64
            * c.s.values_contained() as i64)
    })
}

#[cfg(test)]
#[test]
fn p19t() {
    const SAMPLE: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    const INPUT: &str = include_str!("../inputs/day19.txt");
    assert_eq!(part1(SAMPLE.trim()), 19114);
    assert_eq!(part1(INPUT.trim()), 487623);
    assert_eq!(part2(SAMPLE.trim()), 167409079868000);
    assert_eq!(part2(INPUT.trim()), 113550238315130);
}
