use std::cmp;

pub fn calculate_part_1(input: &str) -> usize {
    input
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            line.splitn(2, ':')
                .nth(1)
                .unwrap()
                .split(|l| l == ',' || l == ';')
                .all(|item| {
                    let mut split = item.trim().splitn(2, ' ');
                    let num = split.nth(0).unwrap().parse::<usize>().unwrap();
                    let color = split.nth(0).unwrap();

                    match color {
                        "red" => num < 13,
                        "green" => num < 14,
                        "blue" => num < 15,
                        _ => false,
                    }
                })
                .then_some(i + 1)
        })
        .sum()
}

pub fn calculate_part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (r, g, b) = line
                .splitn(2, ':')
                .nth(1)
                .unwrap()
                .split(|l| l == ',' || l == ';')
                .map(|game_move| {
                    let mut split = game_move.trim().splitn(2, ' ');
                    let num = split.nth(0).unwrap().parse::<usize>().unwrap();
                    let color = split.nth(0).unwrap();

                    match color {
                        "red" => (num, 0, 0),
                        "green" => (0, num, 0),
                        "blue" => (0, 0, num),
                        _ => (0, 0, 0),
                    }
                })
                .fold((0, 0, 0), |acc, e| {
                    (
                        cmp::max(acc.0, e.0),
                        cmp::max(acc.1, e.1),
                        cmp::max(acc.2, e.2),
                    )
                });
            r * g * b
        })
        .sum::<usize>()
}
