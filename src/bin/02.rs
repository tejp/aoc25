fn main() {
    let input: Vec<(i64, i64)> = aoc::input(2)
        .trim()
        .split(',')
        .map(|n| {
            let (l, r) = n.split_once('-').unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        })
        .collect();

    let mut part1 = 0;

    input.iter().for_each(|&(l, r)| {
        for n in l..r {
            let s = n.to_string();
            let (l, r) = s.split_at(s.len() / 2);
            if l == r {
                part1 += n;
            }
        }
    });

    println!("{}", part1);

    let mut part2 = 0;

    input.iter().for_each(|&(l, r)| {
        for n in l..r {
            let s = n.to_string();
            for i in 1..s.len() / 2 + 1 {
                let v: Vec<&[u8]> = s.as_bytes().chunks(i).collect();
                if v.iter().all(|&n| n == v[0]) {
                    part2 += n;
                    break;
                }
            }
        }
    });

    println!("{}", part2);
}
