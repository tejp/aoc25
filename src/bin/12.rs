fn main() {
    let input: Vec<&str> = aoc::input(12).trim().split("\n\n").collect();
    let to_binary = |c| match c {
        '#' => Some(1),
        '.' => Some(0),
        _ => None,
    };
    let shapes: Vec<Vec<usize>> = input[..input.len() - 1]
        .iter()
        .map(|s| s.chars().filter_map(|c| to_binary(c)).collect())
        .collect();
    let areas: Vec<(usize, usize, Vec<usize>)> = input[input.len() - 1]
        .lines()
        .map(|s| {
            let mut ns = s
                .split(|c: char| !c.is_numeric())
                .filter_map(|n| n.parse().ok());
            let (w, h) = (ns.next().unwrap(), ns.next().unwrap());
            (w, h, ns.collect())
        })
        .collect();

    let mut part1 = 0;

    let areas: Vec<(usize, usize, Vec<usize>)> = areas
        .into_iter()
        .filter(|(w, h, ns)| {
            let is_possible = w * h >= ns.iter().sum::<usize>() * 9;
            part1 += if is_possible { 1 } else { 0 };
            let n_optimal: usize = shapes
                .iter()
                .zip(ns.iter())
                .map(|(s, n)| s.iter().sum::<usize>() * n)
                .sum();
            !is_possible || n_optimal <= w * h
        })
        .collect();

    if areas.len() != 0 {
        println!("{}", part1);
    } else {
        panic!("Crap! It's all you now...")
    }
}
