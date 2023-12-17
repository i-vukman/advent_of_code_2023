#[test]
fn test_part_1_sample() {
    let input = include_str!("../input/part_1_sample.txt");
    let result = day_02::calculate(input);
    assert_eq!(8, result);
}

#[test]
fn test_part_1() {
    let input = include_str!("../input/part_1.txt");
    let result = day_02::calculate(input);
    assert_eq!(2771, result);
}
