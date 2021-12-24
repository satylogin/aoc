use std::collections::HashMap;

static mut NUMS: [[i64; 3]; 14] = [[0; 3]; 14];

struct DP {
    mem: HashMap<(usize, i64), i64>,
}

impl DP {
    fn solve(&mut self, step: usize, z: i64) -> i64 {
        if step == 14 {
            return if z == 0 { 0 } else { -1 };
        }
        if let Some(&n) = self.mem.get(&(step, z)) {
            return n;
        }
        let [a, b, c] = unsafe { NUMS[step] };
        let mut num = -1;
        for ip in 1..=9 {
            let x = ((z % 26 + b) != ip) as i64;
            let nz = (z / a) * (25 * x + 1) + (ip + c) * x;
            let n = self.solve(step + 1, nz);
            if n >= 0 {
                num = n * 10 + ip;
                break;
            }
        }
        self.mem.insert((step, z), num);
        num
    }
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    let ins = data
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();
    for i in (0..ins.len()).step_by(18) {
        unsafe {
            NUMS[i / 18] = [i + 4, i + 5, i + 15]
                .map(|n| ins[n].split(' ').nth(2).unwrap().parse::<i64>().unwrap());
        }
    }
    let num = format!(
        "{}",
        DP {
            mem: HashMap::new(),
        }
        .solve(0, 0)
    )
    .chars()
    .rev()
    .collect::<String>();
    println!("{}", num);
}
