use crate::utils::{file, solution::{SolutionPair, Solution}};

fn get_possibilities(time: &i64, record: &i64) -> i64 {
    let mut ans: i64 = 0;
    for i in 0..*time {
        if (time - i) * i > *record {
            ans += 1
        }
    }
    return ans;
}

pub fn part2(file_string: String) -> i64 {
    // let file_lines: Vec<Vec<_>> = file_string  // for part1
    let file_lines: Vec<_> = file_string // for part2
        .split_terminator("\n")
        .map(|line| {
            line.split(":").collect::<Vec<&str>>()[1]
                .trim()
                .replace(" ", "")
                .parse::<i64>()
                .unwrap()
            // .split_whitespace()
            // .map(|n| n.parse::<i64>().unwrap())
            // .map(|n| n.parse::<i64>().unwrap())
            // .collect::<Vec<i64>>()
            // .collect::<Vec<_>>()
        })
        // .collect::<Vec<Vec<i64>>>();
        // .collect::<Vec<Vec<_>>>();
        .collect::<Vec<_>>();

    // let pairs = file_lines[0]
    //     .iter()
    //     .zip(file_lines[1].iter())
    //     .collect::<Vec<(&i64, &i64)>>();
    
    let mut ans: Vec<i64> = vec![];
    // for (time, record) in pairs {
    // ans.push(get_possibilities(time, record));
    ans.push(get_possibilities(&file_lines[0], &file_lines[1]));
    // }
    
    ans.iter().fold(1, |acc, n| acc * n)
}

pub fn part1(file_string: String) -> &'static str {
    // TODO: get the first part from the other function commencts, is correct
    "Inside the part1 function, need to uncomment some lines"

}
pub fn solve(input_path: &str) -> SolutionPair {
    let file_string = file::read_file(input_path);
    let sol1 = part1(file_string.clone());
    let sol2 = part2(file_string.clone());

    (Solution::from(sol1), Solution::from(sol2))
}
