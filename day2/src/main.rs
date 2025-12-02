use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let ranges: Vec<(&str, &str)> = input.trim().split(",").map(|r| {
        let (start, end) = r.split_once("-").unwrap();
        (start, end)
    }).collect();

    // part1(ranges);
    part2(ranges);

}

fn part1(ranges: Vec<(&str, &str)>) -> u64 {
    let mut total = 0;

    for r in ranges.iter() {
        if r.0.len() % 2 != 0 && r.1.len() % 2 != 0 {
            continue;
        }

        let half_len = r.1.len() / 2;
        
        for i in 0..(10_usize.pow(half_len as u32)) {
            let mut number = i.to_string();
            number.push_str(&i.to_string());
            let num = number.parse::<u64>().unwrap();
            if num >= r.0.parse::<u64>().unwrap() && num <= r.1.parse::<u64>().unwrap() {
                total += dbg!(number.parse::<u64>().unwrap());
            }
        }

    }

    dbg!(total)
}

fn part2(ranges: Vec<(&str, &str)>) -> u64 {
    let mut total = 0;
    for r in ranges.iter() {
        for n in r.0.parse::<u64>().unwrap()..=r.1.parse::<u64>().unwrap() {
            let mut n_str = n.to_string();
            let mut viable_patterns = vec![None; n_str.len()];
            for pat_len in 1..=(n_str.len()/2) {
                if n_str.len() % pat_len != 0 {continue;}
                viable_patterns[pat_len] = Some(&n_str[..pat_len]);
            }

            let is_viable = viable_patterns.iter().filter_map(|p| *p).any(|pat| {
                let mut my_str = n_str.clone();
                while !my_str.is_empty() {
                    if my_str.starts_with(pat) {
                        my_str = my_str[pat.len()..].to_string();
                    } else {
                        return false
                    }
                }
                return true
            });

            if is_viable {
                total += dbg!(n);
            }
        }
    }
    dbg!(total)
}
