use std::fs;

fn main() {
    let mut calibration: u32 = 0;
    let input: String = fs::read_to_string("input.txt").expect("Unable to read file");
    let list_of_strings: std::str::Lines<'_> = input.lines();
    for str in list_of_strings.into_iter() {
        let only_numbers: Vec<char> = str.chars()
            .filter(|c| c.is_digit(10))
            .collect();
        let final_str: String = only_numbers[0].to_string() + &only_numbers[only_numbers.len() - 1].to_string();
        calibration += final_str.parse::<u32>().unwrap();
    }
    println!("Result: {calibration}");
}
