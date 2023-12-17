pub fn calculate(input: &str) -> usize {
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
