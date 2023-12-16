#[test]
fn test_sample_file_part_1() {
    let input = include_str!("../input/part_1_sample.txt");
    let result = day_01::calculate_calibration_value_part_1(input);
    assert_eq!(result, 142);
}

#[test]
fn test_file_part_1() {
    let input = include_str!("../input/part_1.txt");
    let result = day_01::calculate_calibration_value_part_1(input);
    assert_eq!(result, 54239);
}

#[test]
fn test_sample_file_part_2() {
    let input = include_bytes!("../input/part_2_sample.txt");
    let result = day_01::calculate_calibration_value_part_2(input);
    assert_eq!(result, 281);
}

#[test]
fn test_file_part_2() {
    let input = include_bytes!("../input/part_2.txt");
    let result = day_01::calculate_calibration_value_part_2(input);
    assert_eq!(result, 55343);
}
