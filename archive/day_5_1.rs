fn to_usize(d: Option<&str>) -> usize {
    d.unwrap().parse::<usize>().unwrap()
}

fn main() {
    let points: Vec<[[usize; 2]; 2]> = std::fs::read_to_string("input.txt")
        .unwrap()
        .split('\n')
        .map(|l| {
            let mut iter = l.split(" -> ");
            let mut left = iter.next().unwrap().split(',');
            let mut right = iter.next().unwrap().split(',');
            [
                [to_usize(left.next()), to_usize(left.next())],
                [to_usize(right.next()), to_usize(right.next())],
            ]
        })
        .collect();

    let mut grid = vec![vec![0; 1000]; 1000];
    for point in &points {
        if point[0][0] == point[1][0] {
            let a = std::cmp::min(point[0][1], point[1][1]);
            let b = std::cmp::max(point[0][1], point[1][1]);
            for y in a..=b {
                grid[point[0][0]][y] += 1;
            }
        } else if point[0][1] == point[1][1] {
            let a = std::cmp::min(point[0][0], point[1][0]);
            let b = std::cmp::max(point[0][0], point[1][0]);
            for x in a..=b {
                grid[x][point[0][1]] += 1;
            }
        }
    }
    dbg!(grid
        .iter()
        .map(|r| r.iter().map(|c| (*c > 1) as usize).sum::<usize>())
        .sum::<usize>());
}
