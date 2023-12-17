use std::collections::hash_map::Entry;
use std::collections::VecDeque;

use owo_colors::OwoColorize;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;

use crate::grid::ByteGridView;
use crate::grid::Direction;
use crate::grid::Position;

#[derive(Copy, Clone, Debug)]
struct BfsState {
    pos: Position,
    dir: Direction,
    steps_taken: u32,
    heat_loss: i64,
    parent_path: Option<usize>,
}

#[allow(dead_code, reason = "debugging")]
fn visualize(grid: ByteGridView<'_>, path: &[Position]) {
    let points = FxHashSet::from_iter(path.iter().copied());
    for y in 0..grid.rows() {
        for x in 0..grid.columns() {
            if points.contains(&Position { y, x }) {
                print!("{}", "x".red().bold());
            } else {
                print!("{}", grid[y][x] as char);
            }
        }
        println!();
    }
}

#[allow(dead_code, reason = "debugging")]
fn reconstruct_path(last: usize, paths: &[(Option<usize>, Position)]) -> Vec<Position> {
    let mut real_path = Vec::new();
    let mut index = Some(last);

    while let Some(i) = index {
        real_path.push(paths[i].1);
        index = paths[i].0;
    }

    real_path
}

pub fn part1(input: &str) -> i64 {
    let grid = ByteGridView::from(input);

    let mut queue = VecDeque::new();
    queue.push_back(BfsState {
        pos: Position::default(),
        dir: Direction::Right,
        steps_taken: 0,
        heat_loss: 0,
        parent_path: None,
    });

    // (pos, dir, steps_remaining)
    let mut visited = FxHashMap::<_, i64>::default();
    let mut path = Vec::new();

    let mut min = i64::MAX;

    while let Some(s) = queue.pop_front() {
        match visited.entry((s.pos, s.dir, s.steps_taken)) {
            Entry::Occupied(mut entry) => {
                if s.heat_loss >= *entry.get() {
                    // If the current heat loss is greater (or equal), no point in revisiting this.
                    continue;
                } else {
                    entry.insert(s.heat_loss);
                }
            }
            Entry::Vacant(entry) => drop(entry.insert(s.heat_loss)),
        }

        let path_id = path.len();
        path.push((s.parent_path, s.pos));

        if s.pos.x == grid.columns() - 1 && s.pos.y == grid.rows() - 1 {
            min = s.heat_loss.min(min);
            continue;
        }

        for (_, y, x) in grid.non_diagonal_neighbors(s.pos.y, s.pos.x) {
            let direction = if y < s.pos.y {
                Direction::Up
            } else if y > s.pos.y {
                Direction::Down
            } else if x < s.pos.x {
                Direction::Left
            } else if x > s.pos.x {
                Direction::Right
            } else {
                unreachable!()
            };

            if direction == s.dir.reverse() {
                continue;
            }

            if direction == s.dir && s.pos != (Position { x: 0, y: 0 }) {
                if s.steps_taken < 3 {
                    queue.push_back(BfsState {
                        pos: Position { y, x },
                        dir: direction,
                        steps_taken: s.steps_taken + 1,
                        heat_loss: s.heat_loss + (grid[y][x] - b'0') as i64,
                        parent_path: Some(path_id),
                    });
                }
            } else {
                queue.push_back(BfsState {
                    pos: Position { y, x },
                    dir: direction,
                    steps_taken: 1,
                    heat_loss: s.heat_loss + (grid[y][x] - b'0') as i64,
                    parent_path: Some(path_id),
                });
            }
        }
    }

    min
}

pub fn part2(input: &str) -> i64 {
    let grid = ByteGridView::from(input);

    let mut queue = VecDeque::<BfsState>::new();
    queue.push_back(BfsState {
        pos: Position { x: 0, y: 0 },
        dir: Direction::Right,
        steps_taken: 0,
        heat_loss: 0,
        parent_path: None,
    });

    // (pos, dir, steps_remaining)
    let mut visited = FxHashMap::<_, i64>::default();
    let mut path = Vec::new();
    let mut min = i64::MAX;

    while let Some(s) = queue.pop_front() {
        match visited.entry((s.pos, s.dir, s.steps_taken)) {
            Entry::Occupied(mut entry) => {
                if s.heat_loss >= *entry.get() {
                    // If the current heat loss is greater (or equal), no point in revisiting this.
                    continue;
                } else {
                    entry.insert(s.heat_loss);
                }
            }
            Entry::Vacant(entry) => drop(entry.insert(s.heat_loss)),
        }

        let path_id = path.len();
        path.push((s.parent_path, s.pos));

        if s.pos.x == grid.columns() - 1
            && s.pos.y == grid.rows() - 1
            && s.steps_taken >= 4
            && s.steps_taken <= 10
        {
            min = s.heat_loss.min(min);
            continue;
        }

        for (_, y, x) in grid.non_diagonal_neighbors(s.pos.y, s.pos.x) {
            let direction = if y < s.pos.y {
                Direction::Up
            } else if y > s.pos.y {
                Direction::Down
            } else if x < s.pos.x {
                Direction::Left
            } else if x > s.pos.x {
                Direction::Right
            } else {
                unreachable!()
            };

            if direction == s.dir.reverse() {
                continue;
            }

            if direction != s.dir && s.pos != (Position { x: 0, y: 0 }) {
                if s.steps_taken >= 4 {
                    queue.push_back(BfsState {
                        pos: Position { y, x },
                        dir: direction,
                        steps_taken: 1,
                        heat_loss: s.heat_loss + (grid[y][x] - b'0') as i64,
                        parent_path: Some(path_id),
                    });
                }
            } else if s.steps_taken < 10 {
                queue.push_back(BfsState {
                    pos: Position { y, x },
                    dir: direction,
                    steps_taken: s.steps_taken + 1,
                    heat_loss: s.heat_loss + (grid[y][x] - b'0') as i64,
                    parent_path: Some(path_id),
                });
            }
        }
    }

    min
}

#[cfg(test)]
#[test]
fn p17t() {
    const INPUT: &str = include_str!("../inputs/day17.txt");
    assert_eq!(part1(INPUT.trim()), 1155);
    assert_eq!(part2(INPUT.trim()), 1283);
}
