fn next_permutation<T>(arr: &mut [T]) -> bool
where
    T: std::cmp::Ord,
{
    use std::cmp::Ordering;

    let last_ascending = match arr.windows(2).rposition(|w| w[0] < w[1]) {
        Some(i) => i,
        None => {
            arr.reverse();
            return false;
        }
    };
    let swap_with = arr[last_ascending + 1..]
        .binary_search_by(|n| match arr[last_ascending].cmp(n) {
            Ordering::Equal => Ordering::Greater,
            ord => ord,
        })
        .unwrap_err();
    arr.swap(last_ascending, last_ascending + swap_with);
    arr[last_ascending + 1..].reverse();
    true
}

fn resolve(base: &Vec<char>, n: &str) -> i64 {
    let base: String = base.iter().collect();
    let mut patterns = vec![vec![0; 7]; 10];
    vec![
        vec![0, 1, 4, 5, 6, 3],
        vec![1, 4],
        vec![0, 1, 2, 6, 5],
        vec![0, 1, 2, 4, 5],
        vec![3, 2, 1, 4],
        vec![0, 3, 2, 4, 5],
        vec![0, 3, 6, 5, 4, 2],
        vec![0, 1, 4],
        vec![0, 1, 2, 3, 4, 5, 6],
        vec![0, 1, 2, 3, 4, 5],
    ]
    .into_iter()
    .enumerate()
    .for_each(|(i, n)| n.into_iter().for_each(|s| patterns[i][s] = 1));
    let mut slate = vec![0; 7];
    n.chars().for_each(|c| {
        slate[base.find(c).unwrap()] = 1;
    });
    for i in 0..patterns.len() {
        if patterns[i] == slate {
            return i as i64;
        }
    }
    -1
}

fn valid(base: &Vec<char>, nos: &Vec<&str>) -> bool {
    let mut got = vec![false; 10];
    for n in nos {
        let i = resolve(base, n);
        if i == -1 {
            return false;
        }
        got[i as usize] = true;
    }
    got.into_iter().all(|e| e)
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = data.split('\n').collect();
    let mut signals = vec!['d', 'a', 'f', 'e', 'b', 'c', 'g'];
    let s = lines
        .into_iter()
        .filter(|line| line.len() > 0)
        .map(|line| {
            let mut splitter = line.split('|');
            let nos = splitter
                .next()
                .unwrap()
                .split(' ')
                .filter(|c| c.len() > 0)
                .collect::<Vec<_>>();
            let query = splitter
                .next()
                .unwrap()
                .split(' ')
                .filter(|n| n.len() > 0)
                .collect::<Vec<_>>();
            loop {
                if valid(&signals, &nos) {
                    let mut num = 0;
                    query
                        .into_iter()
                        .for_each(|n| num = num * 10 + resolve(&signals, n));
                    dbg!(num);
                    break num;
                }
                next_permutation(&mut signals);
            }
        })
        .sum::<i64>();
    println!("{}", s);
}
