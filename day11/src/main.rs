use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut adj_list = HashMap::new();

    for line in input.lines() {
        let (k, v_str) = line.split_once(":").unwrap();
        let v: Vec<&str> = v_str.trim().split(" ").collect();
        adj_list.insert(k, v);
    }

    dbg!(part1(&adj_list, "you"));

    let mut cache = HashMap::new();
    dbg!(part2(&adj_list, "svr", false, false, &mut cache));
    

}

fn part1(adj_list: &HashMap<&str, Vec<&str>>, curr: &str) -> u64 {

    if curr == "out" {
        return 1
    }
    
    let mut total = 0;
    if let Some(edges) = adj_list.get(curr) {
        for e in edges.iter() {
            total += part1(adj_list, *e); 
        }
    }
    total
}

fn part2<'a>(adj_list: &HashMap<&'a str, Vec<&'a str>>, curr: &'a str, dac_seen: bool, fft_seen: bool, cache: &mut HashMap<(&'a str,bool,bool), u64>) -> u64 {

    if let Some(v) = cache.get(&(curr, dac_seen, fft_seen)) {
        return *v
    }

    if curr == "out" && dac_seen && fft_seen {
        return 1
    }

    let mut this_dac = dac_seen;
    let mut this_fft = fft_seen;

    if curr == "fft" {
        this_fft = true;
    }
    if curr == "dac" {
        this_dac = true;
    }
    
    let mut total = 0;
    if let Some(edges) = adj_list.get(curr) {
        for e in edges.iter() {
            total += part2(adj_list, *e, this_dac, this_fft, cache); 
        }
    }
    cache.insert((curr, this_dac, this_fft), total);
    dbg!(total)
}


