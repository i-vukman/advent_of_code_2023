pub fn calculate_calibration_value(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first_digit = line.chars().find(|c| c.is_numeric()).map(|c| c.to_digit(10).unwrap()).unwrap();

            let second_digit = line.chars().rev().find(|c| c.is_numeric()).map(|c| c.to_digit(10).unwrap()).unwrap();

            first_digit * 10 + second_digit
        })
        .sum()
}
