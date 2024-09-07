use std::collections::HashMap;
use crate::utils::file;
use crate::utils::solution::{SolutionPair, Solution};


fn part1(file_string: String) -> i32 {
   let final_lines: Vec<_> = file_string
        .split_terminator("\n")
        .map(|line| 
        line.chars().filter(|c| c.is_digit(10)).collect::<Vec<char>>())
        .collect();

    let digits: i32 = final_lines
        .iter()
        .map(|line| {
            format!(
                "{}{}",
                line.first().unwrap().to_string(),
                line.last().unwrap().to_string()
            )
            .parse::<i32>()
            .unwrap()
        }).sum::<i32>();

    digits

}

fn part2(file_string: String)-> u32 {
    let numbers: HashMap<&str, u8> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    // TODO: maybe try to do it concatenating all the maps in a one-liner?
    let final_lines: Vec<_> = file_string
        .split_terminator("\n").collect();

    let mut sum: u32 = 0;
    for line in final_lines.iter() {
        let mut line_numbers: Vec<String> = Vec::new();

      line_numbers.extend(line.chars().enumerate()
        .filter_map(|(i, c)| {
                // TODO: find a cleaner way
                if let Some((_, value)) =  numbers.iter().find(|(&key, _)| line[i..].starts_with(key)) {
                    // Some(*value as u32)
                    Some(value.to_string())
                } else if c.is_digit(10) {
                    Some(c.to_string())
                } else {
                    None
                }
            }
      ));
    let concat: String = format!("{}{}", line_numbers.first().unwrap(), line_numbers.last().unwrap());
    sum += concat.parse::<u32>().unwrap();

    }
    sum
}

pub fn solve(input_path: &str) -> SolutionPair {
    let file_string = file::read_file(input_path);
    let sol1 = part1(file_string.clone());
    let sol2 = part2(file_string.clone());

    (Solution::from(sol1), Solution::from(sol2))
}
