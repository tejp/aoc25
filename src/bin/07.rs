use std::collections::HashMap;

fn main() {
    let input: Vec<Vec<char>> = aoc::input(7).lines().map(|s| s.chars().collect()).collect();

    let mut beams = HashMap::from([(input[0].iter().position(|c| *c == 'S').unwrap(), 1i64)]);
    let mut part1 = 0i64;
    for r in input[2..].iter().step_by(2) {
        let mut new_beams = HashMap::new();
        for (b, c) in beams {
            if r[b] == '^' {
                part1 += 1;
                new_beams.entry(b - 1).and_modify(|n| *n += c).or_insert(c);
                new_beams.entry(b + 1).and_modify(|n| *n += c).or_insert(c);
            } else {
                new_beams.entry(b).and_modify(|n| *n += c).or_insert(c);
            }
        }
        beams = new_beams;
    }

    println!("{}", part1);

    let part2: i64 = beams.iter().map(|(_, c)| *c).sum();

    println!("{}", part2);
}
