use day_01::calculate_calibration_value;

fn main() {
    let input = include_str!("../input/day_01_sample.txt");
    let result = calculate_calibration_value(input);
    println!("Result is {result}");
}
