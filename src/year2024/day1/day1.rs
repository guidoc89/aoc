use std::collections::HashMap;
use std::iter::zip;

use crate::utils::file;
use crate::utils::solution::{Solution, SolutionPair};

fn part1(file_string: String) -> i32 {
    let lines: Vec<_> = file_string.split_terminator("\n").map(|line| {
        let mut nums = line.split_ascii_whitespace().map(|c| c.parse::<i32>().unwrap());
        (nums.next().unwrap(), nums.next().unwrap())
    }).collect();

    let mut L: Vec<_> = lines.iter().map(|(l,_)| l).collect();
    let mut R: Vec<_> = lines.iter().map(|(_,r)| r).collect();
    L.sort_unstable();
    R.sort_unstable();

    L.into_iter().zip(R.into_iter()).map(|(l,r)| (r-l).abs()).sum()
}

fn part2(file_string: String) -> i32 {
    let lines: Vec<_> = file_string.split_terminator("\n").map(|line| {
        let mut nums = line.split_ascii_whitespace().map(|c| c.parse::<i32>().unwrap());
        (nums.next().unwrap(), nums.next().unwrap())
    }).collect();

    let mut L: Vec<_> = lines.iter().map(|(l,_)| *l).collect();
    let mut R: Vec<_> = lines.iter().map(|(_,r)| *r).collect();
    L.sort_unstable();
    R.sort_unstable();

    let mut hm = HashMap::new();
    for r in &R {
        // *hm.entry(*r).and_modify(|v| *v += 1).or_insert(1);
        *hm.entry(r).or_insert(0) += 1;
    }


    L.iter().map(|l| l * hm.get(l).unwrap_or(&0)).sum()
}

pub fn solve(file_path: &str) -> SolutionPair {
    let file_string = file::read_file(file_path);

    let sol1 = part1(file_string.clone());
    let sol2 = part2(file_string.clone());

    (Solution::from(sol1), Solution::from(sol2))
}
