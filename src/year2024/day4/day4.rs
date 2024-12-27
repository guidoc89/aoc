use std::ops::RangeBounds;

use crate::utils::file;
use crate::utils::solution::{Solution, SolutionPair};


fn part1(file_string: String) -> i32 {
    let grid: Vec<Vec<char>> = file_string.split_terminator("\n").map(|line| line.chars().collect::<Vec<_>>()).collect();
    let n_rows = grid.len();
    let n_cols = grid[0].len();
    let mut ans: i32 = 0;

    for i  in 0..n_rows {
        for j in 0..n_cols {
            
            // Horizontal
            if (j + 3) < n_cols && grid[i][j] == 'X' && grid[i][j+1] == 'M' && grid[i][j+2] == 'A' && grid[i][j+3] == 'S'  {
                ans += 1
            }
            if (j + 3) < n_cols && grid[i][j] == 'S' && grid[i][j+1] == 'A' && grid[i][j+2] == 'M' && grid[i][j+3] == 'X' {
                ans += 1
            }
            
            // Vertical
            if (i + 3) < n_rows && grid[i][j] == 'X' && grid[i+1][j] == 'M' && grid[i+2][j] == 'A' && grid[i+3][j] == 'S' {
                ans += 1
            }
            if (i + 3) < n_rows && grid[i][j] == 'S' && grid[i+1][j] == 'A' && grid[i+2][j] == 'M' && grid[i+3][j] == 'X' {
                ans += 1
            }
            
            // Top left ->  bottom right
            if  (i + 3) < n_rows && (j + 3) < n_cols && grid[i][j] == 'X' && grid[i+1][j+1] == 'M' && grid[i+2][j+2] == 'A' && grid[i+3][j+3] == 'S' {
                ans += 1
            }
            if  (i + 3) < n_rows && (j + 3) < n_cols && grid[i][j] == 'S' && grid[i+1][j+1] == 'A' && grid[i+2][j+2] == 'M' && grid[i+3][j+3] == 'X' {
                ans += 1
            }

            if i >= 3 && (j + 3) < n_cols && grid[i][j] == 'S' && grid[i-1][j+1] == 'A' && grid[i-2][j+2] == 'M' && grid[i-3][j+3] == 'X' {
                ans += 1
            }

            if  i >= 3 && (j + 3) < n_cols && grid[i][j] == 'X' && grid[i-1][j+1] == 'M' && grid[i-2][j+2] == 'A' && grid[i-3][j+3] == 'S' {
                ans += 1
            }

        }
    }
    
    ans
}

fn part2(file_string: String) -> &'static str {
    "Not implemented day4 part 2"
}

pub fn solve(file_path: &str) -> SolutionPair {
    let file_string = file::read_file(file_path);

    let sol1 = part1(file_string.clone());
    let sol2 = part2(file_string.clone());

    (Solution::from(sol1), Solution::from(sol2))
}
