use std::fs;

fn get_file() -> String {
    return fs::read_to_string("input.txt").expect("Unable to read file");
}

fn solve_part1(input: String) -> u32 {
    let mut calibration: u32 = 0;
    for line in input.lines() {
        let mut first: String = String::new();
        let mut last: String = String::new();
        for c in line.chars() {
            if c.is_ascii_digit() {
                if first.is_empty() {
                    first = c.to_string();
                }
                last = c.to_string();
            }
        }
        if !first.is_empty() && !last.is_empty() {
            let final_str: String = first + &last;
            calibration += final_str.parse::<u32>().unwrap();
        }
    }
    println!("Part 1 Total: {calibration}");
    calibration
}

fn translate_word(word: &str) -> &str {
    match word {
        "one" | "1" => "1",
        "two" | "2" => "2",
        "three" | "3" => "3",
        "four" | "4" => "4",
        "five" | "5" => "5",
        "six" | "6" => "6",
        "seven" | "7" => "7",
        "eight" | "8" => "8",
        "nine" | "9" => "9",
        _ => "_"
    }
}

const KNOWN_OPTIONS: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    "1", "2", "3", "4", "5", "6", "7", "8", "9"
    ];

fn solve_part2(input: String) -> u32 {
    let mut calibration: u32 = 0;
    for line in input.lines() {
        let mut pos_first: usize = usize::MAX;
        let mut pos_last: usize = 0;
        let mut first: String = String::new();
        let mut last: String = String::new();

        for first_find in KNOWN_OPTIONS {
            if let Some(pos) = line.find(first_find) {
                if pos <= pos_first {
                    pos_first = pos;
                    first = first_find.to_string();
                }                
            }
        }

        for last_find in KNOWN_OPTIONS {
            if let Some(pos) = line.rfind(last_find) {
                if pos >= pos_last {
                    pos_last = pos;
                    last = last_find.to_string();
                }                
            }
        }

        if !first.is_empty() && !last.is_empty() {
            first = translate_word(&first).to_string();
            last = translate_word(&last).to_string();
            let final_str: String = first + &last;
            calibration += final_str.parse::<u32>().unwrap();
        }
    }
    println!("Part 2 Total: {calibration}");
    calibration
}

fn main() {
    let input = get_file();
    solve_part1(input);
    
    let input = get_file();
    solve_part2(input);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        assert_eq!(solve_part1(data.to_string()), 142)
    }

    #[test]
    fn sample_02() {
        let data = "
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        assert_eq!(solve_part2(data.to_string()), 281)
    }
}