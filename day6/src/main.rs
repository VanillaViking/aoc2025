use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    dbg!(input.split("\n\n").map(|problem| {
        let mut operation = '!';
        let mut total: u64 = 0;
        for l in problem.lines() {
            let mut curr = 0;
            if l.as_bytes()[l.len()-1] as char == '+' || l.as_bytes()[l.len()-1] as char == '*' {
                operation = l.as_bytes()[l.len()-1] as char;
                curr = l[..(l.len()-1)].parse::<u64>().unwrap();
            } else {
                curr = l.parse::<u64>().unwrap();
            }
            match operation {
                '*' => {
                    if total != 0 {
                        total *= curr
                    } else {
                        total = curr
                    }
                },
                '+' => total += curr,
                _ => panic!()
            };

        }
        total
    }).sum::<u64>());



    // let part1: u64 = input.lines().map(|l| {
    //     let nums: Vec<&str> = l.trim().split(" ").collect();
    //     let operation = nums[nums.len() -1];

    //     let parsed_nums = nums[..(nums.len()-1)].iter().map(|n| {
    //         dbg!(n);
    //         n.parse::<u64>().unwrap()
    //     });

    //     match operation {
    //         "*" => parsed_nums.reduce(|acc, e| acc * e).unwrap(),
    //         "+" => parsed_nums.reduce(|acc, e| acc + e).unwrap(),
    //         _ => panic!()
    //     }
    // }).sum::<u64>();

    // dbg!(part1);
}
