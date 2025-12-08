use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input").unwrap();
     
    let coords: Vec<(i64, i64, i64)> = input.lines().map(|l| {
        let mut c_it = l.trim().split(",").map(|n| n.parse::<i64>().unwrap());
        (c_it.next().unwrap(), c_it.next().unwrap(), c_it.next().unwrap()) 
    }).collect();

    let mut disjoint_set = DisjointSet { ds: HashMap::new(), sizes: HashMap::new() };
    for c in coords.iter() {
        disjoint_set.ds.insert(c.clone(), *c);
    }
    
    let mut edges = Vec::new(); 
    for i in 1..coords.len() {
        for j in 0..i {
            edges.push(Edge { start: coords[i], end: coords[j] });
        }
    }

    edges.sort();
    
    let mut last_edge = None;
    for e in edges.into_iter() {
        if disjoint_set.get_rep(e.start) != disjoint_set.get_rep(e.end) {
            last_edge = Some(e.clone());
        }
        disjoint_set.union(e.start, e.end); 
    }

    let mut sizes: Vec<usize> = disjoint_set.sizes.into_values().collect();
    sizes.sort_by(|a,b| b.cmp(a));

    dbg!(&sizes);

    dbg!(sizes[0] * sizes[1] * sizes[2]);
    
    // p2
    dbg!(last_edge.clone().unwrap().start.0 * last_edge.unwrap().end.0);

    
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Edge {
    start: (i64, i64, i64),
    end: (i64, i64, i64)
}

impl Edge {
    fn dist(&self) -> i64 {
        ((self.end.0 - self.start.0).pow(2) + (self.end.1 -self.start.1).pow(2) + (self.end.2 -self.start.2).pow(2)).isqrt()
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist().cmp(&other.dist())
    }
}
impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct DisjointSet {
    ds: HashMap<(i64, i64, i64), (i64, i64, i64)>,
    sizes: HashMap<(i64,i64,i64), usize>
    
}


impl DisjointSet {
    fn get_rep(&mut self, val: (i64,i64,i64)) -> (i64,i64,i64) {
        if self.ds.get(&val).unwrap() == &val {
            return val
        } else {
            let new_rep = self.get_rep(self.ds.get(&val).unwrap().clone());
            self.ds.insert(val, new_rep);
            return new_rep
        }
    }

    fn union(&mut self, a: (i64,i64,i64), b: (i64,i64,i64)) {
        let a_rep = self.get_rep(a.clone());
        let b_rep = self.get_rep(b.clone());

        if a_rep == b_rep {
            return
        }
        
        let a_size = self.sizes.get(&a_rep).unwrap_or(&1).clone();
        let b_size = self.sizes.get(&b_rep).unwrap_or(&1).clone();
        let new_size = a_size + b_size;

        if a_size > b_size {
            self.ds.insert(b_rep, a_rep.clone());
            self.sizes.insert(a_rep, new_size);
        } else {
            self.ds.insert(a_rep, b_rep.clone());
            self.sizes.insert(b_rep, new_size);
        }
    }
}
