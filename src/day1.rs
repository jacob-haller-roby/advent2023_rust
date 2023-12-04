use std::fs;
use fancy_regex::Regex;
use std::collections::HashMap;

fn parse_row(row: &str) -> Vec<&str> {
    Regex::new(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine|zero))").unwrap()
        .captures_iter(row)
        .map(|c| c.unwrap().get(1).unwrap().as_str())
        .collect()
}

fn parse_digit_str(digit_str: &str) -> u32 {
    let map: HashMap<&str, u32> =
        [
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
            ("zero", 0),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("0", 0),
        ].iter().cloned().collect();
    map.get(digit_str).unwrap().clone()
}

pub fn main() -> u32 {
    let file_path = String::from("./files/day1.txt");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    contents.split("\n")
        .map(parse_row)
        .map(|digits| {
            [*digits.get(0).unwrap(), *digits.get(digits.len() - 1).unwrap()]
        })
        .map(|digits| digits.map(parse_digit_str))
        .map(|digits| digits.get(0).unwrap() * 10 + digits.get(1).unwrap())
        .sum::<u32>()
}