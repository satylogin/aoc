use std::collections::HashMap;

const WEIGHT: [i64; 4] = [1, 10, 100, 1000];
const BUF_WEIGHT: [i64; 7] = [0, 1, 3, 5, 7, 9, 10];

fn weight(c: char) -> i64 {
    WEIGHT[(c as u8 - 'A' as u8) as usize]
}

fn buf_traversal_cost(i: usize, j: usize, c: char) -> i64 {
    (BUF_WEIGHT[i] - BUF_WEIGHT[j]).abs() * WEIGHT[(c as u8 - 'A' as u8) as usize]
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    r: [Vec<char>; 4],
    b: [char; 7],
}

impl State {
    fn is_valid_room(&self, i: usize) -> bool {
        self.r[i]
            .iter()
            .all(|&c| i == (c as u8 - 'A' as u8) as usize)
    }

    fn entry_cost(&self, i: usize) -> i64 {
        (4 - self.r[i].len()) as i64 * WEIGHT[i]
    }

    fn exit_cost(&self, i: usize, c: char) -> i64 {
        (4 - self.r[i].len()) as i64 * weight(c)
    }

    fn transition_room_to_buffer(&self) -> Vec<(State, i64)> {
        let mut res = vec![];
        for i in 0..4 {
            if self.is_valid_room(i) {
                continue;
            }
            let mut next = self.clone();
            let c = next.r[i].pop().unwrap();
            for j in (0..=i + 1).rev() {
                let cost = buf_traversal_cost(j, i + 1, c) + weight(c) + next.exit_cost(i, c);
                if next.b[j] == '.' {
                    next.b[j] = c;
                    res.push((next.clone(), cost));
                    next.b[j] = '.';
                } else {
                    break;
                }
            }
            for j in i + 2..7 {
                let cost = buf_traversal_cost(i + 2, j, c) + weight(c) + next.exit_cost(i, c);
                if next.b[j] == '.' {
                    next.b[j] = c;
                    res.push((next.clone(), cost));
                    next.b[j] = '.';
                } else {
                    break;
                }
            }
        }
        res
    }

    fn transition_buffer_to_room(&self) -> Vec<(State, i64)> {
        let mut res = vec![];
        for i in 0..7 {
            if self.b[i] == '.' {
                continue;
            }
            let r = (self.b[i] as u8 - 'A' as u8) as usize;
            if !self.is_valid_room(r) {
                continue;
            }
            if i <= r + 1 {
                if (i + 1..=r + 1).all(|i| self.b[i] == '.') {
                    let mut next = self.clone();
                    let c = buf_traversal_cost(i, r + 1, next.b[i])
                        + weight(next.b[i])
                        + self.entry_cost(r);
                    next.r[r].push(next.b[i]);
                    next.b[i] = '.';
                    res.push((next, c));
                }
            } else {
                if (r + 2..i).all(|i| self.b[i] == '.') {
                    let mut next = self.clone();
                    let c = buf_traversal_cost(r + 2, i, next.b[i])
                        + weight(next.b[i])
                        + self.entry_cost(r);
                    next.r[r].push(next.b[i]);
                    next.b[i] = '.';
                    res.push((next, c));
                }
            }
        }
        res
    }

    fn transitions(&self) -> Vec<(State, i64)> {
        let mut res = self.transition_room_to_buffer();
        res.append(&mut self.transition_buffer_to_room());
        res
    }
}

fn main() {
    let input = State {
        r: [
            vec!['C', 'D', 'D', 'D'],
            vec!['A', 'B', 'C', 'D'],
            vec!['B', 'A', 'B', 'B'],
            vec!['C', 'C', 'A', 'A'],
        ],
        b: ['.'; 7],
    };
    let expected = State {
        r: [
            vec!['A', 'A', 'A', 'A'],
            vec!['B', 'B', 'B', 'B'],
            vec!['C', 'C', 'C', 'C'],
            vec!['D', 'D', 'D', 'D'],
        ],
        b: ['.'; 7],
    };
    let mut costs = HashMap::new();
    let mut q = std::collections::BinaryHeap::new();
    costs.insert(input.clone(), 0);
    q.push((0, input));
    while let Some((cost, grid)) = q.pop() {
        let cost = -cost;
        if cost != costs[&grid] {
            continue;
        }
        if grid == expected {
            break;
        }
        for (transition, t_cost) in grid.transitions() {
            if let Some(&c) = costs.get(&transition) {
                if c <= t_cost + cost {
                    continue;
                }
            }
            costs.insert(transition.clone(), t_cost + cost);
            q.push((-(t_cost + cost), transition));
        }
    }
    dbg!(costs.get(&expected));
}
