
use std::{
    collections::HashMap, fs
};

fn part_1(file_path: &str) {
    let file_string = fs::read_to_string(file_path).unwrap();
    let maximums: HashMap<&str, i32> = HashMap::from([("blue", 14), ("red", 12), ("green", 13)]);

    let lines: Vec<&str> = file_string.split_terminator("\n").collect();
    let mut sum: i32 = 0;

    for line in lines {
        let both_parts: Vec<&str> = line.split(":").collect();
        let mut checks: Vec<bool> = Vec::new();
        let game_id: i32 = both_parts[0].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let right = both_parts[1];
        let _ = right
            .split(";")
            .map(|set| {
                let mut accumulator: HashMap<&str, i32> = HashMap::from([("blue", 0), ("red", 0), ("green", 0)]);
                set.split(",")
                    .map(|throw| {
                        if let Some((color, _)) = accumulator
                            .iter()
                            .find(|(&color, _)| throw.contains(color))
                        {
                            let to_add = throw.replace(color, "").trim().parse::<i32>().unwrap();
                            accumulator
                                .entry(color)
                                .and_modify(|current_count| *current_count += to_add)
                                .or_insert(0);
                        }
                    })
                    .collect::<Vec<_>>();

                // Need to check, by key, whether the accumulator values are greater than the initial, 
                checks.push(accumulator.iter().all(|(&color, current_value)| current_value <= maximums.get(color).unwrap()));
            })
            .collect::<Vec<_>>();

        if checks.iter().all(|check| *check) {
            sum += game_id;
        }

    }
    println!("{:?}", sum);
}

fn part_2(file_path: &str) {
    let file_string = fs::read_to_string(file_path).unwrap();

    let lines: Vec<&str> = file_string.split_terminator("\n").collect();
    let mut sum: i32 = 0;

    for line in lines {
        let mut game_minumums: HashMap<&str, i32> = HashMap::from([("blue", 0), ("red", 0), ("green", 0)]);
        let both_parts: Vec<&str> = line.split(":").collect();
        let right = both_parts[1];
        let _ = right
            .split(";")
            .map(|set| {
                set.split(",")
                    .map(|throw| {
                        if let Some((color, &value)) = game_minumums
                            .iter()
                            .find(|(&color, _)| throw.contains(color))
                        {
                            let to_add = throw.replace(color, "").trim().parse::<i32>().unwrap();
                            if to_add > value {
                                game_minumums.insert(color, to_add);
                            }
                        }
                    })
                    .collect::<Vec<_>>();

            })
            .collect::<Vec<_>>();
        
        sum += game_minumums.into_values().reduce(|acc,a| acc * a).unwrap();
    }
    println!("{:?}", sum);
}

pub fn solve(file_path: &str) {
    println!("Solution for 2022 day 2 - Input mode: {:?}", file_path);
}
// fn main() {
//     // PART 1
//     // let file_path: &str = "../test1.txt";
//     // let file_path: &str = "../prod1.txt";
//     // let file_path: &str = "../test2.txt";
//     let file_path: &str = "../prod2.txt";
//
//     // part_1(file_path);
//     part_2(file_path);
// }
