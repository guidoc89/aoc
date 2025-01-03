use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::RangeBounds;

use crate::utils::file;
use crate::utils::solution::{Solution, SolutionPair};

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];


fn part1(file_string: String) -> usize {
    let mut guard_position: (i32, i32) = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let grid: Vec<Vec<char>> = file_string
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let ans: usize = 0;
    let n_rows = grid.len() as i32;
    let n_cols = grid[0].len() as i32;

    for i in (0..n_rows) {
        for j in (0..n_cols) {
            if grid[i as usize][j as usize] == '^' {
                guard_position = (i, j);
            }
        }
    }
    visited.insert(guard_position);

    let mut cycle: i32 = 0;
    loop {
        let (delta_r, delta_c) = DIRECTIONS[cycle as usize];
        let new_r = guard_position.0 + delta_r;
        let new_c = guard_position.1 + delta_c;

        if 0 <= new_r && new_r < n_rows && 0 <= new_c && new_c < n_cols && grid[new_r as usize][new_c as usize] != '#' {
            guard_position = (new_r, new_c);
            visited.insert(guard_position);
        }
        // We've encountered an obstacle, turn right 90 degrees
        else {
            cycle = (cycle + 1) % 4;
        }

        // We've exited the grid
        if 0 > new_r || new_r >= n_rows || 0 > new_c || new_c >= n_cols {
            break;
        }
    }

    visited.len()
}


fn part2(file_string: String) -> usize {
    let mut ans: usize = 0;
    let mut original_guard_position: (i32, i32) = (0, 0);
    let mut guard_position: (i32, i32) = (0, 0);
    let mut grid: Vec<Vec<char>> = file_string
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let n_rows = grid.len() as i32;
    let n_cols = grid[0].len() as i32;

    'outer: for i in (0..n_rows) {
        for j in (0..n_cols) {
            if grid[i as usize][j as usize] == '^' {
                original_guard_position = (i, j);
                break 'outer;
            }
        }
    }

    for i in (0..n_rows) {
        for j in (0..n_cols) {
            if grid[i as usize][j as usize] != '.' || (i, j) == original_guard_position {
                continue;
            }

            let mut cycle: i32 = 0;
            let mut visited: HashSet<((i32, i32), i32)> = HashSet::new();
            guard_position = original_guard_position;
            grid[i as usize][j as usize] = '#';

            loop {
                // NOTE: important! need to check not only whether we came back to the starting
                // point, but also whether we did it coming from the same direction -> same
                // starting position from the same direction == loop
                let mut loop_condition = (guard_position, cycle);

                if visited.contains(&loop_condition) {
                    ans += 1;
                    break;
                }

                visited.insert(loop_condition);
                let (delta_r, delta_c) = DIRECTIONS[cycle as usize];
                let new_r = guard_position.0 + delta_r;
                let new_c = guard_position.1 + delta_c;

                if 0 <= new_r && new_r < n_rows && 0 <= new_c && new_c < n_cols && grid[new_r as usize][new_c as usize] != '#' {
                    guard_position = (new_r, new_c);
                } else {
                    cycle = (cycle + 1) % 4;
                }

                // We've exited the grid
                if 0 > new_r || new_r >= n_rows || 0 > new_c || new_c >= n_cols {
                    break;
                }
            }
            // }

            // Backtrack
            grid[i as usize][j as usize] = '.';
        }
    }

    ans
}


pub fn solve(file_path: &str) -> SolutionPair {
    let file_string = file::read_file(file_path);

    let sol1 = part1(file_string.clone());
    let sol2 = part2(file_string.clone());

    (Solution::from(sol1), Solution::from(sol2))
}
