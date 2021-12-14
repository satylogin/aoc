use std::collections::BTreeMap;

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = data.split('\n').filter(|l| !l.is_empty());

    let polymer = lines.next().unwrap().to_string();
    let conversions = lines
        .map(|l| {
            let mut splitter = l.split("->");
            let base = splitter.next().unwrap().trim();
            let to = splitter.next().unwrap().trim();
            (base, to)
        })
        .collect::<BTreeMap<_, _>>();
    let mut freq: BTreeMap<String, usize> = BTreeMap::new();
    for i in 0..26 {
        for j in 0..26 {
            freq.insert(
                format!("{}{}", ('A' as u8 + i) as char, ('A' as u8 + j) as char),
                0,
            );
        }
    }
    for i in 0..polymer.len() - 1 {
        *freq.get_mut(&polymer[i..=i + 1]).unwrap() += 1;
    }
    for _ in 0..40 {
        let mut new_freq = BTreeMap::new();
        for i in 0..26 {
            for j in 0..26 {
                new_freq.insert(
                    format!("{}{}", ('A' as u8 + i) as char, ('A' as u8 + j) as char),
                    0,
                );
            }
        }
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
        let k = k.chars().collect::<Vec<_>>();
        *f.entry(k[0]).or_insert(0) += v;
        *f.entry(k[1]).or_insert(0) += v;
    }
    for (_, v) in &mut f {
        if (*v) & 1 == 1 {
            *v = *v / 2 + 1;
        } else {
            *v = *v / 2;
        }
    }
    let mut vals = f
        .into_iter()
        .map(|(_, v)| v)
        .filter(|v| *v != 0)
        .collect::<Vec<_>>();
    vals.sort();
    dbg!(&vals);
    println!("{}", vals[vals.len() - 1] - vals[0]);
}
