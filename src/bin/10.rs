fn main() {
    let mut settings: Vec<Vec<bool>> = vec![];
    let mut buttons: Vec<Vec<Vec<usize>>> = vec![];
    let mut joltages: Vec<Vec<usize>> = vec![];
    aoc::input(10)
        //     "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        // [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        // [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
        .lines()
        .for_each(|line| {
            let vect: Vec<&str> = line.split(" ").collect();
            let setting = vect[0]
                .chars()
                .filter_map(|c| {
                    if c == '#' {
                        Some(true)
                    } else if c == '.' {
                        Some(false)
                    } else {
                        None
                    }
                })
                .collect();
            settings.push(setting);

            let ns: Vec<Vec<usize>> = vect[1..vect.len()]
                .iter()
                .map(|s| {
                    s.split(|c: char| !c.is_numeric())
                        .filter_map(|s| s.parse().ok())
                        .collect()
                })
                .collect();

            buttons.push(ns[..ns.len() - 1].to_vec());

            joltages.push(ns[ns.len() - 1].clone());
        });

    //println!("{:?}", joltages);

    let mut part1 = 0;
    for i in 0..buttons.len() {
        let state = vec![false; settings[i].len()];
        let comb = buttons[i].iter().fold(
            vec![(state.clone(), 0)],
            |state: Vec<(Vec<bool>, i64)>, toggles: &Vec<usize>| {
                state
                    .into_iter()
                    .flat_map(|state| {
                        let (state, c) = state;
                        let mut new_state = state.clone();
                        for &t in toggles {
                            new_state[t] = !new_state[t];
                        }
                        [(state, c), (new_state, c + 1)]
                    })
                    .collect()
            },
        );
        let (_, pushes) = comb
            .iter()
            .filter(|(s, _)| *s == settings[i])
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap();
        part1 += pushes;
    }

    println!("{:?}", part1);

    /*let mut part2 = 0;
    for i in 0..buttons.len() {
        let init_state = vec![(vec![0; joltages[i].len()], 0)];
        let comb = buttons[i].clone()
            .into_iter()
            .fold(init_state.into_iter(), |state: std::vec::IntoIter<(Vec<usize>, usize)>, toggles: Vec<usize>| {
                let mut new_states = vec![];
                state.for_each(|(state, c)| {
                    let mut state = state.clone();
                    new_states.push((state.clone(), c));
                    for j in 1.. {
                        for t in toggles.iter() {
                            state[*t] += 1;
                        }
                        if state.iter().zip(joltages[i].iter()).all(|(a, b)| a <= b) {
                            //println!("{:?}\n{:?}\n", state, joltages[i]);
                            new_states.push((state.clone(), c + j));
                        } else {
                            break;
                        }
                    }
                });
                state.chain(new_states.into_iter())
            });
        let (_, count) = comb
            .filter(|(s, _)| *s == joltages[i])
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap();
        part2 += count;
    }

    println!("{:?}", part2);*/
}
