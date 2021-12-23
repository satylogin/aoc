use std::collections::{HashMap, VecDeque};

fn weight(c: char) -> usize {
    match c {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => unreachable!(),
    }
}

fn room(c: char) -> usize {
    (c as u8 - 'A' as u8 + 1) as usize * 2 + 1
}

fn room_for(r: usize) -> char {
    ((r / 2 - 1) as u8 + 'A' as u8) as char
}

fn can_reach(grid: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<Vec<bool>> {
    let mut vis = vec![vec![false; grid[0].len()]; grid.len()];
    let mut q = VecDeque::new();
    vis[i][j] = true;
    q.push_back((i, j));
    while let Some((x, y)) = q.pop_front() {
        for (a, b) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if !vis[a][b] && (grid[a][b] == '.' || grid[a][b] == grid[i][j]) {
                vis[a][b] = true;
                q.push_back((a, b));
            }
        }
    }
    vis
}

fn transitions(grid: Vec<Vec<char>>, cost: usize) -> Vec<(Vec<Vec<char>>, usize)> {
    let is_valid_room = |r: usize, c: char| (2..=5).all(|i| grid[i][r] == '.' || grid[i][r] == c);
    let mut transitions = vec![];
    // check all those we can come out of room.
    for j in [3, 5, 7, 9] {
        if is_valid_room(j, room_for(j)) {
            continue;
        }
        for i in 2..=5 {
            if grid[i][j] == '.' {
                continue;
            }
            let can_reach = can_reach(&grid, i, j);
            for k in 1..grid[0].len() - 1 {
                if can_reach[1][k] && ![3, 5, 7, 9].contains(&k) {
                    let c =
                        cost + (i - 1 + (j as i32 - k as i32).abs() as usize) * weight(grid[i][j]);
                    let mut t = grid.clone();
                    t[1][k] = grid[i][j];
                    t[i][j] = '.';
                    transitions.push((t, c));
                }
            }
            break;
        }
    }
    // check all those who can go into their room
    for j in 1..grid[0].len() - 1 {
        if !grid[1][j].is_alphabetic() {
            continue;
        }
        let r = room(grid[1][j]);
        if !is_valid_room(r, room_for(r)) {
            continue;
        }
        let can_reach = can_reach(&grid, 1, j);
        for k in (2..=5).rev() {
            if grid[k][r] == '.' && can_reach[k][r] {
                let c = cost + (k - 1 + (j as i32 - r as i32).abs() as usize) * weight(grid[1][j]);
                let mut t = grid.clone();
                t[k][r] = grid[1][j];
                t[1][j] = '.';
                transitions.push((t, c));
                break;
            }
        }
    }

    transitions
}

fn main() {
    let grid = std::fs::read_to_string("input.txt")
        .unwrap()
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let expected = [
        "#############",
        "#...........#",
        "###A#B#C#D###",
        "  #A#B#C#D#",
        "  #A#B#C#D#",
        "  #A#B#C#D#",
        "  #########",
    ]
    .into_iter()
    .map(|l| l.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();

    let mut costs = HashMap::new();
    let mut q = std::collections::BinaryHeap::new();
    costs.insert(grid.clone(), 0);
    q.push((0, grid));
    let mut cnt = 0;
    while let Some((cost, grid)) = q.pop() {
        cnt += 1;
        if cnt % 100000 == 0 {
            for r in &grid {
                println!("{:?}", r.iter().collect::<String>());
            }
            println!("cost: {}", cost);
        }
        let cost = (-cost) as usize;
        if cost != costs[&grid] {
            continue;
        }
        if grid == expected {
            break;
        }
        for (transition, t_cost) in transitions(grid, cost) {
            if costs.contains_key(&transition) && costs[&transition] <= t_cost {
                continue;
            }
            costs.insert(transition.clone(), t_cost);
            q.push((-(t_cost as i64), transition));
        }
    }
    dbg!(costs.get(&expected));
}
