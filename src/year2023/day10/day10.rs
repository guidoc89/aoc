use std::{char, collections::{HashMap, HashSet, VecDeque}};

use crate::utils::{file, solution::{Solution, SolutionPair}};


// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

fn part1(file_string: String) -> usize {
    // solution: 1969958987
    let mut grid: Vec<Vec<char>> = file_string.split_terminator("\n").map(|line| line.chars().collect()).collect();
    // let n_rows = grid.len();
    // let n_cols = grid[0].len();
    // println!("Max distance: {}", max_distance);
    //
    // // Start flood fill at the start position
    let max_distance = 0;
    max_distance
}


fn part2(file_string: String) -> i32 {
    // solution: 1068
    let mut ans = 0;
    ans
}

pub fn solve(input_path: &str) -> SolutionPair {
    let file_string = file::read_file(input_path);
    let sol1 = part1(file_string.clone());
    let sol2 = part2(file_string.clone());

    (Solution::from(sol1), Solution::from(sol2))
}
