fn main() {
    let input: Vec<i64> = aoc24::input(1)
        .lines()
        .map(|line| {
            let dir = line.chars().next().unwrap();
            let neg = if dir == 'R' { 1 } else { -1 };
            neg * line[1..].parse::<i64>().unwrap()
        })
        .collect();

    let (_, part1) = input.iter().fold((50, 0), |(pos, zeros), n| {
        let pos = (pos + n).rem_euclid(100);
        (pos, zeros + if pos == 0 { 1 } else { 0 })
    });

    println!("{}", part1);

    let (_, part2) = input.iter().fold((50, 0), |(pos, zeros), n| {
        let mut zeros = zeros + (pos + n).abs().div_euclid(100);
        if pos + n <= 0 && pos != 0 {
            zeros += 1;
        }
        ((pos + n).rem_euclid(100), zeros)
    });

    println!("{}", part2);
}
