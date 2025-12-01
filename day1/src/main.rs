use std::fs;


fn main() {
    let input = fs::read_to_string("sample").unwrap();
    
    let mut curr = 50;

    let lines: Vec<&str> = input.lines().collect();

    let mut pass = 0;
    for line in lines.iter() {
        let direction = line.chars().nth(0).unwrap();
        let amount = line[1..].parse::<i32>().unwrap();

        match direction {
            'L' => {
                for _ in 0..amount {
                    curr -= 1;
                    if curr == 0 {
                        pass+=1;
                    } else if curr < 0 {
                        curr = 99;
                    }
                }
            }
            'R' => {
                for _ in 0..amount {
                    curr += 1;
                    if curr == 100 {
                        pass+=1;
                        curr = 0;
                    } 
                }
            }
            _ => panic!()
        }
    }

    dbg!(pass);
    
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

