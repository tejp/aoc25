fn impossible((w, h, v): &(usize, usize, Vec<usize>), shapes: &Vec<Vec<char>>) -> bool {
    let n_fill: Vec<usize> = shapes
        .iter()
        .map(|v| v.iter().filter(|&c| *c == '#').count())
        .collect();
    let s: usize = v.iter().zip(n_fill.iter()).map(|(n1, n2)| n1 * n2).sum();

    s > w * h
}

fn main() {
    let input: Vec<&str> = aoc::input(12).trim().split("\n\n").collect();
    let shapes: Vec<Vec<char>> = input[..input.len() - 1]
        .iter()
        .map(|s| s.chars().filter(|&c| c == '#' || c == '.').collect())
        .collect();
    let areas: Vec<(usize, usize, Vec<usize>)> = input[input.len() - 1]
        .split("\n")
        .map(|s| {
            let mut ns = s
                .split(|c: char| !c.is_numeric())
                .filter_map(|n| n.parse().ok());
            let (w, h) = (ns.next().unwrap(), ns.next().unwrap());
            (w, h, ns.collect())
        })
        .collect();

    let my_shapes = vec![
        "###\n###\n###\n###",                     // 3,3
        "###\n###\n###\n###\n###\n###",           // 3,4,5
        "###\n###\n###\n###\n###\n###\n###\n###", // 4,4,5,5
    ];

    let areas: Vec<(usize, usize, Vec<usize>)> = areas
        .into_iter()
        .filter(|a| !impossible(a, &shapes))
        .collect();

    println!("{:?} {:?}", areas.len(), shapes);
}
