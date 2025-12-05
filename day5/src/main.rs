use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let (ranges_str, ids) = input.split_once("\n\n").unwrap();

    let mut ranges: Vec<(u64,u64)> = ranges_str.lines().map(|r| {
        let (s, e) = r.split_once("-").unwrap();
        (s.parse::<u64>().unwrap(), e.parse::<u64>().unwrap())
    }).collect();
    
    let mut total: u64 = 0;
    for id in ids.lines().map(|i| i.parse::<u64>().unwrap()) {
        for r in ranges.iter() {
            if id <= r.1 && id >= r.0 {
                total += 1;
                break;
            }
        }
    }
    
    dbg!(total);
    
    ranges.sort_by(|a, b| (b.1 - b.0 + 1).cmp(&(a.1-a.0 + 1)));

    dbg!();

    let mut all_ids = 0;
    let mut seen_ranges: Vec<(u64, u64)> = Vec::new();
    for (i, r) in ranges.iter().enumerate() {
        let mut curr = r.clone();
        for seen_range in seen_ranges.iter() {
            if curr.0 <= seen_range.1 && curr.0 >= seen_range.0 {
                curr.0 = seen_range.1 + 1;
            }
            if curr.1 <= seen_range.1 && curr.1 >= seen_range.0 {
                curr.1 = seen_range.0 -1;
            }
        }
        if let Some(valids) = curr.1.checked_sub(curr.0) {
            dbg!(curr);
            seen_ranges.push((curr.0, curr.1));
            all_ids += valids+1;
        }
    }

    dbg!(all_ids);

}
