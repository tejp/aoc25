fn main() {
    let input: Vec<Vec<char>> = aoc24::input(3)
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    let mut part1: i64 = 0;

    for l in &input {
        let mut first: char = '0';
        let mut second: char = '0';
        for &c in l {
            if second > first {
                first = second;
                second = c;
            } else if c > second {
                second = c;
            }
        }
        part1 += format!("{}{}", first, second).parse::<i64>().unwrap();
    }

    println!("{}", part1);

    let mut part2: i64 = 0;

    for l in &input {
        let mut on: [char; 12] = ['0'; 12];
        on.copy_from_slice(&l[0..12]);
        for &c in &l[12..] {
            for i in 0..11 {
                if on[i + 1] > on[i] {
                    on[i..].rotate_left(1);
                    on[11] = c;
                    break;
                }
            }
            if c > on[11] {
                on[11] = c;
            }
        }
        part2 += on.iter().collect::<String>().parse::<i64>().unwrap();
    }
    println!("{}", part2);
}
