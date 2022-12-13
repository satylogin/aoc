fn main() {
    for file in ["test.txt", "input.txt"] {
        let grid = std::fs::read_to_string(file)
            .unwrap()
            .split('\n')
            .filter(|l| !l.is_empty())
            .map(|l| {
                l.chars()
                    .map(|c| (c as u8 - '0' as u8) as i32)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        dbg!(scenic_score(&grid));
    }
}

fn scenic_score(grid: &Vec<Vec<i32>>) -> usize {
    let (n, m) = (grid.len(), grid[0].len());
    let mut score = vec![vec![1; m]; n];
    for i in 0..n {
        let mut stack = vec![];
        for j in 0..m {
            score[i][j] *= j - block_index(&mut stack, j, grid[i][j], 0);
        }
        let mut stack = vec![];
        for j in (0..m).rev() {
            score[i][j] *= block_index(&mut stack, j, grid[i][j], m - 1) - j;
        }
    }
    for j in 0..m {
        let mut stack = vec![];
        for i in 0..n {
            score[i][j] *= i - block_index(&mut stack, i, grid[i][j], 0);
        }
        let mut stack = vec![];
        for i in (0..n).rev() {
            score[i][j] *= block_index(&mut stack, i, grid[i][j], n - 1) - i;
        }
    }
    return *score
        .iter()
        .map(|sr| sr.iter().max().unwrap())
        .max()
        .unwrap();
}

fn block_index(stack: &mut Vec<(usize, i32)>, idx: usize, v: i32, default: usize) -> usize {
    let mut index = default;
    while let Some(&(t_idx, top)) = stack.last() {
        if top < v {
            stack.pop();
        } else {
            index = t_idx;
            break;
        }
    }
    stack.push((idx, v));
    index
}
