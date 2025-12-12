use std::{cmp::{self, Reverse}, collections::{BinaryHeap, HashMap, HashSet, VecDeque}, fs};

#[derive(Debug)]
struct FactoryMachine {
    lights_diagram: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl FactoryMachine {
    fn parse(input: &str) -> Self {
        let (indicators_str, rest_str) = input.split_once("]").unwrap();
        let lights_diagram: Vec<bool> = indicators_str[1..].chars().map(|c| if c == '#' { true } else { false }).collect();
        
        let (button_str, joltage_str) = rest_str.split_once("|{").unwrap();
        let buttons: Vec<Vec<usize>> = button_str.split("|").map(|b| b.split(",").map(|idx| idx.parse::<usize>().unwrap()).collect()).collect();
        let joltage = joltage_str[..(joltage_str.len()-1)].split(",").map(|n| n.parse::<usize>().unwrap()).collect();

        FactoryMachine { lights_diagram, buttons, joltage }

    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let machines: Vec<FactoryMachine> = input.lines().map(|l| FactoryMachine::parse(l)).collect();
    // dbg!(part1(machines));
    dbg!(part2_old(machines));

}

// starting ....
// actions
//  button 1
//      min steps #.#.
//  button 2
//  button 3
//  button 4
//
//  can't cause they arent SUB problems
//

fn part1(machines: Vec<FactoryMachine>) -> u64 {
    let mut total = 0;
    for (i,m) in machines.iter().enumerate() {
        dbg!(i);
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        queue.push_back((0,vec![false; m.lights_diagram.len()]));
        while let Some(curr) = queue.pop_front() {
            if curr.1 == m.lights_diagram {
                total += curr.0;
                break;
            }
            if seen.contains(&curr.1) {
                continue;
            }
            seen.insert(curr.1.to_vec());
            for b in m.buttons.iter() {
                let mut next_state = curr.1.to_vec();
                for i in b.iter() {
                    next_state[*i] = !next_state[*i];
                }
                queue.push_back((curr.0+1, next_state));
            }
        }
    }
    total
}


fn part2(machines: Vec<FactoryMachine>) -> u64 {
    let mut total = 0;
    for (i,m) in machines.iter().enumerate() {
        let mut cache = HashMap::new();
        total += dbg!(min_presses(vec![0; m.joltage.len()], &m.joltage, &m.buttons, &mut cache));
    }
    total
}

fn min_presses(curr: Vec<usize>, dest: &Vec<usize>, buttons: &Vec<Vec<usize>>, cache: &mut HashMap<Vec<usize>, u64>) -> u64 {
    if let Some(v) = cache.get(&curr) {
        return *v
    }
    if curr == *dest {
        return 0
    }
    for i in 0..curr.len() {
        if curr[i] > dest[i] {
            return u64::MAX
        }
    }
    
    let mut min_p = u64::MAX;
    for b in buttons {
        let mut new_state = curr.to_vec();
        for i in b.iter() {
            new_state[*i] += 1;
        }
        let next = min_presses(new_state, dest, buttons, cache);
        if next == u64::MAX {
            continue;
        }
        min_p = cmp::min(min_p, next+1);
    }
    cache.insert(curr, min_p);
    min_p
}


fn part2_old(machines: Vec<FactoryMachine>) -> u64 {
    let mut total = 0;
    'machines: for m in machines.iter() {
        let mut dists = HashMap::new();
        let mut min_heap = BinaryHeap::new();
        dists.insert(vec![0; m.joltage.len()], 0);
        min_heap.push(Reverse((*m.joltage.iter().max().unwrap(), vec![0; m.joltage.len()], 0)));

        'heap: while let Some(Reverse(curr)) = min_heap.pop() {
            if curr.1 == m.joltage {
                total += dbg!(curr.2 as u64);
                continue 'machines;
            }

            for b in m.buttons.iter() {
                let mut next_j = curr.1.to_vec();
                for i in b.iter() {
                    next_j[*i] += 1;
                }
                let mut h = 0;
                for i in 0..m.joltage.len() {
                    if let Some(diff) = m.joltage[i].checked_sub(next_j[i]) {
                        h  = cmp::max(h, diff);
                    } else {
                        continue 'heap;
                    }
                }
                let next_state = (curr.2+1+h, next_j, curr.2+1);

                if *dists.get(&next_state.1).unwrap_or(&usize::MAX) > next_state.0 {
                    min_heap.push(Reverse(next_state.clone()));
                    dists.insert(next_state.1, next_state.0);
                }

            }

        }

    }
    total
}
