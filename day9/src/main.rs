use std::{cmp, collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("sample").unwrap();

    let points: Vec<(i64, i64)> = input.lines().map(|l| {
        let (x, y) = l.trim().split_once(",").unwrap();
        (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
    }).collect();


    // p1 
    let mut max_area = 0;
    for i in 1..points.len() {
        for j in 0..i {
            max_area = cmp::max(max_area, ((points[i].0 - points[j].0).abs() +1) * ((points[i].1 - points[j].1).abs() + 1));
        }
    }

    dbg!(max_area);

    //p2

    let mut outline = HashMap::new();

    for i in 1..points.len() {
        let curr = points[i];
        let prev = points[i-1];

        if curr.0 == prev.0 {
            
        }
    }


}
