fn main() {
    let mut grid = std::fs::read_to_string("input.txt")
        .unwrap()
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let n = grid.len();
    let m = grid[0].len();
    for step in 1.. {
        let mut moved = false;
        for (f, (x, y)) in [('>', (0, 1)), ('v', (1, 0))] {
            let mut next = grid.clone();
            for i in 0..n {
                for j in 0..m {
                    if grid[i][j] == f && grid[(i + x) % n][(j + y) % m] == '.' {
                        next[(i + x) % n][(j + y) % m] = f;
                        next[i][j] = '.';
                        moved = true;
                    }
                }
            }
            grid = next;
        }
        if !moved {
            println!("{}", step);
            break;
        }
    }
}
