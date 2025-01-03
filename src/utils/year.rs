use crate::{year2022, year2023, year2024};
use crate::utils::{solution::SolutionPair, cli::FileInput};

pub fn match_year_and_day(year: &str, day: &str, file_input: FileInput) -> SolutionPair {
    let input_name = file_input.file_name();
    let input_path = format!("src/year{}/{}/{}", year, day, input_name);
    match year {
        "2022" => match_day_in_year_2022(day, input_path),
        "2023" => match_day_in_year_2023(day, input_path),
        "2024" => match_day_in_year_2024(day, input_path),
        _ => panic!("Year not defined")
    }
}

fn match_day_in_year_2022(day: &str, input_path: String) -> SolutionPair {
    match day {
        // "day1" => year2022::day1::day1::solve(&input_path),
        // "day2" => year2022::day2::day2::solve(&input_path),
        // "day3" => year2022::day3::day3::solve(&input_path),
        // "day4" => year2022::day4::day4::solve(&input_path),
        // "day5" => year2022::day5::day5::solve(&input_path),
        // "day6" => year2022::day6::day6::solve(&input_path),
        // "day7" => year2022::day7::day7::solve(&input_path),
        _ => panic!("Day not defined")
    }
}

fn match_day_in_year_2023(day: &str, input_path: String) -> SolutionPair {
    match day {
        // "day1" => year2023::day1::day1::solve(&input_path),
        // "day2" => year2023::day2::day2::solve(&input_path),
        // "day3" => year2023::day3::day3::solve(&input_path),
        // "day4" => year2023::day4::day4::solve(&input_path),
        // "day5" => year2023::day5::day5::solve(&input_path),
        // "day6" => year2023::day6::day6::solve(&input_path),
        // "day7" => year2023::day7::day7::solve(&input_path),
        // "day8" => year2023::day8::day8::solve(&input_path),
        // "day9" => year2023::day9::day9::solve(&input_path),
        "day10" => year2023::day10::day10::solve(&input_path),
        _ => panic!("Day not defined")
    }
}

fn match_day_in_year_2024(day: &str, input_path: String) -> SolutionPair {
    match day {
        "day1" => year2024::day1::day1::solve(&input_path),
        "day2" => year2024::day2::day2::solve(&input_path),
        "day3" => year2024::day3::day3::solve(&input_path),
        "day4" => year2024::day4::day4::solve(&input_path),
        "day5" => year2024::day5::day5::solve(&input_path),
        "day6" => year2024::day6::day6::solve(&input_path),
        _ => panic!("Day not defined")
    }
}
