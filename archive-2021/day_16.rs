use std::collections::HashMap;

fn main() {
    let mapping = vec![
        ('0', "0000"),
        ('1', "0001"),
        ('2', "0010"),
        ('3', "0011"),
        ('4', "0100"),
        ('5', "0101"),
        ('6', "0110"),
        ('7', "0111"),
        ('8', "1000"),
        ('9', "1001"),
        ('A', "1010"),
        ('B', "1011"),
        ('C', "1100"),
        ('D', "1101"),
        ('E', "1110"),
        ('F', "1111"),
    ]
    .into_iter()
    .collect::<HashMap<char, &str>>();
    let message = std::fs::read_to_string("input.txt")
        .unwrap()
        .chars()
        .flat_map(|c| mapping[&c].chars())
        .collect::<String>();
    println!("{}", inspect(&message, 0).1);
}

fn inspect(encoded: &str, mut start: usize) -> (usize, i64) {
    if start + 6 > encoded.len() {
        return (encoded.len(), 0);
    }
    let _version = &encoded[start..start + 3];
    let type_id = &encoded[start + 3..start + 6];
    start += 6;

    let result = if type_id == "100" {
        // literal
        let mut num = String::new();
        loop {
            num.push_str(&encoded[start + 1..start + 5]);
            start += 5;
            if &encoded[start - 5..start - 4] == "0" {
                break;
            }
        }
        i64::from_str_radix(&num, 2).unwrap()
    } else {
        // operator
        let mut results = vec![];
        if &encoded[start..start + 1] == "1" {
            let num_sub_packets =
                usize::from_str_radix(&encoded[start + 1..start + 12], 2).unwrap();
            start += 12;
            for _i in 0..num_sub_packets {
                let (s, r) = inspect(encoded, start);
                start = s;
                results.push(r);
            }
        } else {
            let total_packets_len =
                usize::from_str_radix(&encoded[start + 1..start + 16], 2).unwrap();
            start += 16;
            let stop_at = start + total_packets_len;
            while start < stop_at {
                let (s, r) = inspect(encoded, start);
                start = s;
                results.push(r);
            }
        }
        match type_id {
            "000" => results.iter().sum(),
            "001" => results.iter().product(),
            "010" => *results.iter().min().unwrap(),
            "011" => *results.iter().max().unwrap(),
            "101" => (results[0] > results[1]) as i64,
            "110" => (results[0] < results[1]) as i64,
            "111" => (results[0] == results[1]) as i64,
            _ => unreachable!(),
        }
    };
    (start, result)
}
