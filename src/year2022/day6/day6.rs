use std::fs;

fn get_possibilities(time: &i64, record: &i64) -> i64 {
    let mut ans: i64 = 0;
    for i in 0..*time {
        if (time - i) * i > *record {
            ans += 1
        }
    }
    return ans;
}

pub fn part1(file_path: &str) {
    let file_string: String = fs::read_to_string(file_path).unwrap();

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
    
    println!("{:?}", ans.iter().fold(1, |acc, n| acc * n));
}

pub fn solve(file_path: &str) {
    println!("Solution for 2022 day 6 - Input mode: {:?}", file_path);
}
