use std::collections::BTreeMap;

fn freq_map() -> BTreeMap<String, usize> {
    let chars = (0..26).map(|i| ('A' as u8 + i) as char).collect::<Vec<_>>();
    chars
        .iter()
        .flat_map(|c1| chars.iter().map(move |c2| (format!("{}{}", c1, c2), 0)))
        .collect()
}

fn main() {
    let start_instant = std::time::Instant::now();
    let data = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = data.split('\n').filter(|l| !l.is_empty());

    let polymer = lines.next().unwrap().to_string();
    let conversions = lines
        .map(|l| {
            let parts = l.split("->").map(|w| w.trim()).collect::<Vec<_>>();
            (parts[0], parts[1])
        })
        .collect::<BTreeMap<_, _>>();
    let mut freq = freq_map();
    (0..polymer.len() - 1).for_each(|i| *freq.get_mut(&polymer[i..i + 2]).unwrap() += 1);
    for _ in 0..40 {
        let mut new_freq = freq_map();
        for (k, v) in freq.iter() {
            if let Some(c) = conversions.get(k.as_str()) {
                let k1 = format!("{}{}", k.chars().nth(0).unwrap(), c);
                let k2 = format!("{}{}", c, k.chars().nth(1).unwrap());
                *new_freq.get_mut(&k1).unwrap() += v;
                *new_freq.get_mut(&k2).unwrap() += v;
            } else {
                *new_freq.get_mut(k.as_str()).unwrap() += v;
            }
        }
        freq = new_freq;
    }

    let mut f = BTreeMap::new();
    for (k, v) in freq {
        *f.entry(k.chars().nth(0).unwrap()).or_insert(0) += v;
        *f.entry(k.chars().nth(1).unwrap()).or_insert(0) += v;
    }
    let mut vals = f
        .into_iter()
        .map(|(_, v)| if v & 1 == 1 { v / 2 + 1 } else { v / 2 })
        .filter(|v| *v != 0)
        .collect::<Vec<_>>();
    vals.sort();
    println!("{}", vals[vals.len() - 1] - vals[0]);
    println!("elapsed: {}", start_instant.elapsed().as_micros());
}
