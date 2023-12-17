#[test]
fn test_part_1_sample() {
    let input = include_str!("../input/sample.txt");
    let result = day_02::calculate_part_1(input);
    assert_eq!(8, result);
}

#[test]
fn test_part_1() {
    let input = include_str!("../input/puzzle_input.txt");
    let result = day_02::calculate_part_1(input);
    assert_eq!(2771, result);
}

#[test]
fn test_part_2_sample()
{
    let input = include_str!("../input/sample.txt");
    let result = day_02::calculate_part_2(input);
    assert_eq!(2286, result);
}

#[test]
fn test_part_2()
{
    let input = include_str!("../input/puzzle_input.txt");
    let result = day_02::calculate_part_2(input);
    assert_eq!(70924, result);
}
