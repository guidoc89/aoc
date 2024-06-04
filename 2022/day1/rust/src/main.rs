use std::{fmt::format, fs};


fn part_1(file_path: &str ) {
    let mut file_string = fs::read_to_string(file_path).unwrap();

    let final_lines: Vec<_> = file_string
        .split_terminator("\n")
        .map(|line| 
        line.chars().filter(|c| c.is_digit(10)).collect::<Vec<char>>())
        .collect();

    // let digits: Vec<i32> = final_lines
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

    println!("{:?}", digits);
}


fn part_2(file_path: &str ) {
    let mut file_string = fs::read_to_string(file_path).unwrap();

    let final_lines: Vec<_> = file_string
        .split_terminator("\n")
        .map(|line| 
        line.chars().filter(|c| c.is_digit(10)).collect::<Vec<char>>())
        .collect();

    // let digits: Vec<i32> = final_lines
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

    println!("{:?}", digits);

}

fn main() {
    // PART 1
    // let file_path: &str = "../test.txt";
    let file_path: &str = "../prod.txt";
    part_2(file_path);
}
