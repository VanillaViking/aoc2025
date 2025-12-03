use std::{cmp, fs};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let banks: Vec<Vec<u32>> = input.lines()
        .map(|l| l.trim().chars().map(|ch| {
            ch.to_digit(10).unwrap()
        }).collect())
        .collect();
    
    part2(banks);
}

fn part1(banks: Vec<Vec<u32>>) -> u32 {
    let mut total = 0;
    for b in banks.iter() {
        let mut max = 0;
        let mut max_idx = b.len() -1;
        for i in 0..(b.len()-1) {
            if max < b[i] {
                max = b[i];
                max_idx = i;
            }
        }
        total += dbg!(max*10);
        
        max = 0;
        for i in (max_idx+1)..b.len() {
            max = cmp::max(max, b[i]);
        }

        total += dbg!(max);
    }

    dbg!(total)
}

fn part2(banks: Vec<Vec<u32>>) -> u64 {
    let mut total = 0;
    for b in banks.iter() {

        let mut max_idx = 0;
        for d in (1..=12).rev() {
            let mut max = 0;
            dbg!(&b[max_idx..(b.len()-d)]);
            dbg!(b.len() -d);
            for i in (max_idx.clone())..=(b.len()-d) {
                if max < b[i] {
                    max = b[i];
                    max_idx = i+1;
                }
            }
            total += dbg!(max as u64 * (10_u64.pow(d as u32-1)));
        }
    }
    dbg!(total)
}
