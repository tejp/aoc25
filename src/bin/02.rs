fn main() {
    let input: Vec<(i64, i64)> = aoc24::input(2)
        .trim()
        .split(',')
        .map(|n| {
            let (l, r) = n.split_once('-').unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        })
        .collect();

    let mut inv_sum = 0;

    input.iter().for_each(|(l, r)| {
        for n in *l..*r {
            let s = n.to_string();
            let (l, r) = s.split_at(s.len() / 2);
            if l == r {
                inv_sum += n;
            }
        }
    });

    println!("{}", inv_sum);

    inv_sum = 0;

    input.iter().for_each(|(l, r)| {
        for n in *l..*r {
            let s = n.to_string();
            for i in 1..s.len() / 2 + 1 {
                let v: Vec<&[u8]> = s.as_bytes().chunks(i).collect();
                if v.iter().all(|&n| n == v[0]) {
                    inv_sum += n;
                    break;
                }
            }
        }
    });

    println!("{}", inv_sum);
}
