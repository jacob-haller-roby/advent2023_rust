use std::fs;
use std::collections::HashMap;

fn parse_row(row: &str) -> Vec<&str> {
    let mut matches: Vec<&str> = vec![];
    for start in 0..row.len() {
        let substring = &row[start..];
        if substring.starts_with("1") {
            matches.push("1");
            continue;
        }

        if substring.starts_with("2") {
            matches.push("2");
            continue;
        }

        if substring.starts_with("3") {
            matches.push("3");
            continue;
        }

        if substring.starts_with("4") {
            matches.push("4");
            continue;
        }

        if substring.starts_with("5") {
            matches.push("5");
            continue;
        }

        if substring.starts_with("6") {
            matches.push("6");
            continue;
        }

        if substring.starts_with("7") {
            matches.push("7");
            continue;
        }

        if substring.starts_with("8") {
            matches.push("8");
            continue;
        }

        if substring.starts_with("9") {
            matches.push("9");
            continue;
        }

        if substring.starts_with("0") {
            matches.push("0");
            continue;
        }

        if substring.starts_with("one") {
            matches.push("one");
            continue;
        }

        if substring.starts_with("two") {
            matches.push("two");
            continue;
        }

        if substring.starts_with("three") {
            matches.push("three");
            continue;
        }

        if substring.starts_with("four") {
            matches.push("four");
            continue;
        }

        if substring.starts_with("five") {
            matches.push("five");
            continue;
        }

        if substring.starts_with("six") {
            matches.push("six");
            continue;
        }

        if substring.starts_with("seven") {
            matches.push("seven");
            continue;
        }

        if substring.starts_with("eight") {
            matches.push("eight");
            continue;
        }

        if substring.starts_with("nine") {
            matches.push("nine");
            continue;
        }

        if substring.starts_with("zero") {
            matches.push("zero");
            continue;
        }
    }

    println!("row: {} digits: {:?}", row, matches);
    return matches;
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