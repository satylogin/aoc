use std::collections::VecDeque;

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    let grid = data
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c as u8 - '0' as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let n = grid.len();
    let m = grid[0].len();
    let mut mark = vec![vec![false; m]; n];
    let mut lens = vec![];
    for i in 0..n {
        for j in 0..m {
            if !mark[i][j] && grid[i][j] != 9 {
                let mut q = VecDeque::new();
                q.push_back((i as i32, j as i32));
                mark[i][j] = true;
                let mut len = 1;
                while let Some((i, j)) = q.pop_front() {
                    for (x, y) in vec![(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                        if x >= 0
                            && x < n as i32
                            && y >= 0
                            && y < m as i32
                            && !mark[x as usize][y as usize]
                            && grid[x as usize][y as usize] != 9
                        {
                            mark[x as usize][y as usize] = true;
                            len += 1;
                            q.push_back((x, y));
                        }
                    }
                }
                lens.push(len);
            }
        }
    }
    lens.sort_by(|x, y| y.cmp(&x));
    println!("{}", lens[0] * lens[1] * lens[2]);
}
