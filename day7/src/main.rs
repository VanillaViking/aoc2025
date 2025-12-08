use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut cache: Vec<Vec<Option<u64>>> = vec![vec![None; grid[0].len()]; grid.len()];

    let mut start = (0,0);
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 'S' {
                start = (x,y);
            }
        }
    }

    // dbg!(part1(&grid, start, &mut cache));
    
    let mut dp = vec![1 as u64; grid[0].len() +1];

    for y in (0..grid.len()).rev() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '^' {
                dp[x] = dp[x-1] + dp[x+1];
            }
        }
    }
    dbg!(dp[start.0]);
}
/*

y: 0..=grid.len()
    y+1 before y
    big before small
    (0..=grid.len()).rev()
x: 0..grid[0].len()


 *
 * */
fn part1(grid: &Vec<Vec<char>>, start: (usize, usize),cache: &mut Vec<Vec<Option<u64>>>) -> u64 {
    if start.1 >= grid.len() {
        return 1
    }
    if let Some(a) = cache[start.1][start.0] {
        return a
    }

    let mut ans = 0;
    if grid[start.1][start.0] == '^' {
        ans = part1(grid, (start.0.checked_sub(1).unwrap(), start.1), cache) + part1(grid, (start.0+1, start.1), cache);
    } else {
        ans = part1(grid, (start.0, start.1+1), cache);
    }
    cache[start.1][start.0] = Some(ans);
    dbg!(start);
    dbg!(ans)
}
