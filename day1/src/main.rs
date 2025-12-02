use std::fs;


fn main() {
    let input = fs::read_to_string("input").unwrap();
    
    let mut curr = 50;
    let mut lock = 50;

    let lines: Vec<&str> = input.lines().collect();

    let mut pass = 0;
    let mut pass2 = 0;
    for line in lines.iter() {
        let direction = line.chars().nth(0).unwrap();
        let diff = line[1..].parse::<i32>().unwrap();

        match direction {
            'L' => {
                println!("{}", format!("{} - {}", curr, diff));
                for _ in 0..diff {
                    curr -= 1;
                    if curr == 0 {
                        pass+=1;
                    } else if curr < 0 {
                        curr = 99;
                    }
                }

                println!("{}", format!("{} - {}", lock, diff));

                lock -= diff;
                if lock <= 0 {
                    if lock + diff == 0 { // lock + diff will amount to the previous value
                        pass2 += lock.abs() / 100;
                    } else {
                        pass2 += lock.abs() / 100 +1;
                    }
                    lock = 100 - (lock.abs() % 100);
                    if lock == 100 { lock = 0; }
                }
                println!("pass: {}", pass);
                println!("pass2: {}", pass2);
            }
            'R' => {
                println!("{}", format!("{} + {}", curr, diff));
                for _ in 0..diff {
                    curr += 1;
                    if curr == 100 {
                        pass+=1;
                        curr = 0;
                    } 
                }

                println!("{}", format!("{} + {}", lock, diff));

                lock += diff;
                if lock >= 100 {
                    pass2 += lock.abs() / 100;
                    // if curr2 - amount != 100 {
                    //     pass2 += curr2.abs() / 100;
                    // } else {
                    //     pass2 += (curr2.abs() / 100) - 1;
                    // }
                    lock = lock % 100;
                }
                println!("pass: {}", pass);
                println!("pass2: {}", pass2);
            }
            _ => panic!()
        }
    }

    dbg!(pass);
    dbg!(pass2);
    
}


// fn main() {
//     let input = fs::read_to_string("input").unwrap();
    
//     let mut curr = 50;

//     let lines: Vec<&str> = input.lines().collect();

//     let mut pass = 0;
//     for line in lines.iter() {
//         let direction = line.chars().nth(0).unwrap();
//         let amount = line[1..].parse::<i32>().unwrap();

//         match direction {
//             'L' => {
//                 println!("{}", format!("{} - {}", curr, amount));
//                 curr -= amount;
//                 if curr <= 0 {
//                     if curr + amount != 0 {
//                         pass += (curr.abs() / 100) +1;
//                     }
//                     curr = 100 - (curr.abs() % 100);
//                     if curr == 100 { curr = 0; }
//                 }
//                 dbg!(pass);
//             }
//             'R' => {
//                 println!("{}", format!("{} + {}", curr, amount));
//                 curr += amount;
//                 if curr >= 100 {
//                     if curr - amount != 100 {
//                         pass += curr.abs() / 100;
//                     }
//                     curr = curr % 100;
//                 }
//                 dbg!(pass);
//             }
//             _ => panic!()
//         }
//     }

//     dbg!(pass);
    
// }

