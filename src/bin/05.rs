fn main() {
    let (ranges, ids) = aoc::input(5).split_once("\n\n").unwrap();

    let mut ranges: Vec<(i64, i64)> = ranges
        .lines()
        .map(|r| {
            let (s, e) = r.split_once("-").unwrap();
            (s.parse().unwrap(), e.parse().unwrap())
        })
        .collect();

    let part1 = ids
        .lines()
        .map(|id| id.parse().unwrap())
        .filter(|id: &i64| ranges.iter().any(|(s, e)| id >= s && id <= e))
        .count();

    println!("{}", part1);

    ranges.sort_unstable();

    let (part2, _) = ranges.iter().fold((0, -1), |(count, i), &(s, e)| {
        (count + 0.max(e - s.max(i + 1) + 1), e.max(i))
    });

    println!("{}", part2);
}
