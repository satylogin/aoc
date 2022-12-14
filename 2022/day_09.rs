fn main() {
    for file in ["test.txt", "test2.txt", "input.txt"] {
        let moves = std::fs::read_to_string(file)
            .unwrap()
            .split('\n')
            .filter(|l| !l.is_empty())
            .map(|l| {
                let (dir, steps) = l.split_once(' ').unwrap();
                (dir.chars().next().unwrap(), steps.parse::<usize>().unwrap())
            })
            .collect::<Vec<_>>();
        dbg!(visited(&moves, 10));
    }
}

fn visited(moves: &Vec<(char, usize)>, knots: usize) -> usize {
    let mut visited = std::collections::HashSet::new();
    visited.insert((0, 0));
    let mut loc = vec![(0, 0); knots];
    for &(dir, steps) in moves {
        for _ in 0..steps {
            match dir {
                'R' => loc[0].0 += 1,
                'L' => loc[0].0 -= 1,
                'U' => loc[0].1 += 1,
                'D' => loc[0].1 -= 1,
                _ => unreachable!(),
            }
            for i in 1..knots {
                if is_far(loc[i - 1], loc[i]) {
                    loc[i] = towards(loc[i - 1], loc[i]);
                }
            }
            visited.insert(loc[knots - 1]);
        }
    }
    visited.len()
}

fn is_far(ch: (i32, i32), ct: (i32, i32)) -> bool {
    (ch.0 - ct.0).abs() > 1 || (ch.1 - ct.1).abs() > 1
}

fn towards(ch: (i32, i32), ct: (i32, i32)) -> (i32, i32) {
    if ch.0 == ct.0 || ch.1 == ct.1 {
        return (ct.0 + (ch.0 - ct.0) / 2, ct.1 + (ch.1 - ct.1) / 2);
    } else {
        for (a, b) in [
            (ct.0 - 1, ct.1 - 1),
            (ct.0 + 1, ct.1 - 1),
            (ct.0 - 1, ct.1 + 1),
            (ct.0 + 1, ct.1 + 1),
        ] {
            if !is_far((a, b), ch) {
                return (a, b);
            }
        }
    }
    unreachable!()
}
