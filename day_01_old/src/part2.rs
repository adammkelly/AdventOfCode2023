use std::fs;

fn translate_word(word: &str) -> u32 {
    match word {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => 0
    }
}

fn main() {
    let mut calibration: u32 = 0;
    let input: String = fs::read_to_string("input.txt").expect("Unable to read file");
    let list_of_strings: std::str::Lines<'_> = input.lines();
    let known: [&str; 18] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "1", "2", "3", "4", "5", "6", "7", "8", "9"
        ];
    for str in list_of_strings.into_iter() {
        let mut start: usize = 9999;
        let mut start_p = 0;
        let mut end: usize = 0;
        let mut end_p = 0;
        for dig_name in known {
            if let Some(pos) = str.find(dig_name) {
                if pos <= start {
                    start = pos;
                    start_p = translate_word(dig_name);
                }
            }
            if let Some(pos) = str.rfind(dig_name) {
                if pos >= end {
                    end = pos;
                    end_p = translate_word(dig_name);
                }
            }
        }
        
        let final_str = start_p.to_string() + &end_p.to_string();
        calibration += final_str.parse::<u32>().unwrap();

    }
    println!("Result: {calibration}");
}
