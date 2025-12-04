use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    
    let mut total = 0;
    let mut removed = part1(&mut grid);
    while removed != 0 {
        total += removed;
        removed = part1(&mut grid);
    }

    dbg!(total);
}

fn part1(grid: &mut Vec<Vec<char>>) -> u32 {
    let mut total = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != '@' {continue;}
            let mut roll_count = 0;
            if x > 0 {
                if grid[y][x-1] == '@' {
                    roll_count += 1;
                }
            }
            if y > 0 {
                if grid[y-1][x] == '@' {
                    roll_count += 1;
                }
            }
            if x > 0 && y > 0 {
                if grid[y-1][x-1] == '@' {
                    roll_count += 1;
                }
            }
            if x < grid[0].len() -1 {
                if grid[y][x+1] == '@' {
                    roll_count += 1;
                }
            }
            if y < grid.len() -1 {
                if grid[y+1][x] == '@' {
                    roll_count += 1;
                }
            }
            if x < grid[0].len() -1 && y < grid[0].len() -1 {
                if grid[y+1][x+1] == '@' {
                    roll_count += 1;
                }
            }
            if x < grid[0].len() -1 && y > 0 {
                if grid[y-1][x+1] == '@' {
                    roll_count += 1;
                }
            }

            if y < grid.len() -1 && x > 0 {
                if grid[y+1][x-1] == '@' {
                    roll_count += 1;
                }
            }

            if roll_count < 4 {
                total += 1;
                    grid[y][x] = '.';
            }

        }
    }
    dbg!(total)
}
