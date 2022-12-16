struct Scan {
    sensor: (i32, i32),
    beacon: (i32, i32),
}

fn segment_on(y: i32, s: &Scan) -> Option<(i32, i32)> {
    let mut d = (s.sensor.0 - s.beacon.0).abs() + (s.sensor.1 - s.beacon.1).abs();
    let d_from_seg = (s.sensor.1 - y).abs();
    if d < d_from_seg {
        return None;
    } else {
        d -= d_from_seg;
        Some((s.sensor.0 - d, s.sensor.0 + d))
    }
}

fn get_scans(data: &str) -> Vec<Scan> {
    let extract_num = |s: &str| {
        s.split_once('=')
            .unwrap()
            .1
            .trim_end_matches(',')
            .parse::<i32>()
            .unwrap()
    };
    let extract_pos = |s: &str| {
        let tokens = s.split(' ').collect::<Vec<_>>();
        let n = tokens.len();
        (extract_num(tokens[n - 2]), extract_num(tokens[n - 1]))
    };

    data.split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (sensor, beacon) = l.split_once(':').unwrap();
            Scan {
                sensor: extract_pos(sensor),
                beacon: extract_pos(beacon),
            }
        })
        .collect::<Vec<_>>()
}

fn solve(data: &str, idx: i32) {
    let scans = get_scans(data);
    for y in 0..=idx {
        let mut segments = scans
            .iter()
            .filter_map(|s| segment_on(y, s))
            .collect::<Vec<_>>();
        segments.sort();
        let mut r = (i32::MIN + 1, i32::MIN);
        for (s, e) in segments {
            if r.0 <= s && s <= r.1 {
                r.1 = r.1.max(e);
            } else {
                let x = r.1 + 1;
                if x >= 0 && x <= idx {
                    println!("{}", x as i64 * 4000000 + y as i64);
                    return;
                }
                r = (s, e);
            }
        }
        let x = r.1 + 1;
        if x >= 0 && x <= idx {
            println!("{}", x as i64 * 4000000 + y as i64);
        }
    }
}

fn main() {
    for (file, idx) in [("test.txt", 20), ("input.txt", 4000000)] {
        let now = std::time::Instant::now();
        solve(&std::fs::read_to_string(file).unwrap(), idx);
        println!("elapsed: {}ms", now.elapsed().as_millis());
    }
}
