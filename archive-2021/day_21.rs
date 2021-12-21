use std::collections::HashMap;

fn main() {
    let mut pos = HashMap::<[usize; 2], HashMap<[usize; 2], usize>>::new();
    pos.insert([6, 9], [([0, 0], 1)].into_iter().collect()); // start
    let mut turn = 0;
    let mut wins = [0, 0];
    while !pos.is_empty() {
        let mut new_pos = HashMap::new();
        for (p, states) in pos {
            for (s, f) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
                for (mut score, mut freq) in states.clone() {
                    let mut p = p.clone();
                    p[turn] = (p[turn] + s - 1) % 10 + 1;
                    score[turn] += p[turn];
                    freq *= f;
                    if score[turn] >= 21 {
                        wins[turn] += freq;
                    } else {
                        *new_pos
                            .entry(p)
                            .or_insert(HashMap::new())
                            .entry(score)
                            .or_insert(0) += freq;
                    }
                }
            }
        }
        pos = new_pos;
        turn ^= 1;
    }
    dbg!(wins);
}
