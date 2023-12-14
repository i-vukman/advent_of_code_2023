use std::collections::HashMap;

pub fn calculate_calibration_value_part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first_digit = line
                .chars()
                .find(|c| c.is_numeric())
                .map(|c| c.to_digit(10).unwrap())
                .unwrap();

            let second_digit = line
                .chars()
                .rev()
                .find(|c| c.is_numeric())
                .map(|c| c.to_digit(10).unwrap())
                .unwrap();

            first_digit * 10 + second_digit
        })
        .sum()
}

pub fn calculate_calibration_value_part_2(input: &str) -> u32 {
    let numbers = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
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

    input
        .lines()
        .map(|line| {
            let mut forwards = line;
            let mut backwards = line;

            let first_digit = 'outer: loop {
                for (text, num) in numbers.iter() {
                    if forwards.starts_with(text) {
                        break 'outer num;
                    }
                }
                forwards = forwards.get(1..).unwrap();
            };

            let second_digit = 'outer: loop {
                for (text, num) in numbers.iter() {
                    if backwards.ends_with(text) {
                        break 'outer num;
                    }
                }
                backwards = backwards.get(..backwards.len() - 1).unwrap();
            };

            first_digit * 10 + second_digit
        })
        .sum()
}
