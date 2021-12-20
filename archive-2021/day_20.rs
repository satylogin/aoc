fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = data.split('\n').filter(|l| !l.is_empty());

    let resolver = lines.next().unwrap().chars().collect::<Vec<_>>();

    let mut grid = lines
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let n = grid.len();
    for i in 0..n {
        for _ in 0..55 {
            grid[i].insert(0, '.');
            grid[i].push('.');
        }
    }
    let r = vec!['.'; grid[0].len()];
    for _ in 0..55 {
        grid.insert(0, r.clone());
        grid.push(r.clone());
    }
    let n = grid.len();
    let m = grid[0].len();
    for i in 0..50 {
        let mut resolved = vec![vec!['.'; m]; n];
        for i in 0..n {
            for j in 0..m {
                let mut num = String::new();
                for (x, y) in vec![
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 0),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ] {
                    let (x, y) = (i as i32 + x, j as i32 + y);
                    let d = if x >= 0 && x < n as i32 && y >= 0 && y < m as i32 {
                        if grid[x as usize][y as usize] == '.' {
                            '0'
                        } else {
                            '1'
                        }
                    } else {
                        '0'
                    };
                    num.push(d);
                }
                let num = usize::from_str_radix(&num, 2).unwrap();
                resolved[i][j] = resolver[num];
            }
        }
        grid = resolved;
        if i & 1 == 1 {
            grid[0][0] = '.';
            grid[0][m - 1] = '.';
            grid[n - 1][0] = '.';
        }
    }
    let cnt = grid
        .iter()
        .map(|row| row.iter().map(|c| (*c == '#') as usize).sum::<usize>())
        .sum::<usize>();
    println!("{}", cnt);
}
