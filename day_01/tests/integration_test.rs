#[test]
fn test_sample_file() {
    let input = include_str!("../input/day_01_sample.txt");
    let result = day_01::calculate_calibration_value(input);
    assert_eq!(result, 142);
}

#[test]
fn test_file() {
    let input = include_str!("../input/day_01.txt");
    let result = day_01::calculate_calibration_value(input);
    assert_eq!(result, 54239);
}
