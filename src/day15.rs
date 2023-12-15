fn hash(s: &str) -> i64 {
    s.bytes().fold(0, |h, c| ((h + c as i64) * 17) % 256)
}

pub fn part1(input: &str) -> i64 {
    input.split(',').map(hash).sum()
}

pub fn part2(input: &str) -> i64 {
    let mut boxes: Vec<Vec<(&str, i64)>> = vec![vec![]; 256];

    for segment in input.split(',') {
        if let Some((key, value)) = segment.split_once('=') {
            let h = hash(key);
            let value = value.parse::<i64>().unwrap();
            let bucket = &mut boxes[h as usize];

            if let Some((_, lens)) = bucket.iter_mut().find(|(k, _)| *k == key) {
                *lens = value;
            } else {
                bucket.push((key, value));
            }
        } else if let Some(key) = segment.strip_suffix('-') {
            let h = hash(key);
            let bucket = &mut boxes[h as usize];

            if let Some(idx) = bucket.iter().position(|&(k, _)| k == key) {
                bucket.remove(idx);
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .flat_map(|(bucket_index, bucket)| {
            bucket
                .iter()
                .enumerate()
                .map(move |(slot, &(_, v))| (bucket_index + 1) as i64 * (slot + 1) as i64 * v)
        })
        .sum()
}

#[cfg(test)]
#[test]
fn p15t() {
    const INPUT: &str = include_str!("../inputs/day15.txt");

    assert_eq!(part1(INPUT.trim()), 502139);
    assert_eq!(part2(INPUT.trim()), 284132);
}
