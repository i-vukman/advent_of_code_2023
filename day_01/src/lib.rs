const NUMS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

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

pub fn calculate_calibration_value_part_2(input: &[u8]) -> usize {
    input
        .split(|b| b == &b'\n')
        .map(|line| {
            (0..line.len()).find_map(|i| num(line, i)).unwrap() * 10
                + (0..line.len()).rev().find_map(|i| num(line, i)).unwrap()
        })
        .sum::<usize>()
}

fn num(line: &[u8], index: usize) -> Option<usize> {
    line[index]
        .is_ascii_digit()
        .then_some((line[index] - b'0') as usize)
        .or(NUMS
            .iter()
            .enumerate()
            .find(|(_, num)| line[index..].starts_with(num))
            .map(|(i, _)| i + 1))
}
