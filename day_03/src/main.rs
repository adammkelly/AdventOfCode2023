use aoc::general::open_file;
use aoc::point::*;
use std::collections::HashSet;

// I am using this one to learn using nom.
// Full credit for this goes to:
// https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_02.rs

pub struct Input {
    gears: HashSet<Point>,
    symbols: HashSet<Point>,
    numbers: Vec<(u32, Vec<Point>)>,
}


fn gen_input(input: String) -> Input {
    let mut points = vec![];
    let mut numbers = vec![];
    let mut symbols: HashSet<Point> = HashSet::new();
    let mut gears: HashSet<Point> = HashSet::new();
    let mut curr_num = String::new();

    for (y, line) in input.lines().enumerate() {

        for (x, c) in line.chars().enumerate() {
            let point = Point::new(x as i32, y as i32);

            if c.is_ascii_digit() {
                curr_num = curr_num + &c.to_string();
                points.push(point);
                continue;
            }

            match c {
                '.' => (),
                '*' => {
                    symbols.insert(point);
                    gears.insert(point);
                }
                _ => {
                    symbols.insert(point);
                }
            }

            if !curr_num.is_empty() {
                let num: u32 = curr_num.parse().unwrap();
                numbers.push((num, points.clone()));
                curr_num.clear();
                points.clear();
            }
        }

    }

    Input {
        symbols,
        numbers,
        gears,
    }
}


fn solve_part1(input: String) -> u32 {
    let mut total: u32 = 0;
    let input_info: Input = gen_input(input);
    for (number, points) in input_info.numbers {
        'points_loop: for num_point in points {
            for diagonal in DIAGONALS.iter().map(|diag| num_point + *diag) {
                if input_info.symbols.contains(&diagonal) {
                    total += number;
                    break 'points_loop;
                }
            }
        }
    }    

    println!("Part 1 Total: {total}");
    total
}

fn solve_part2(input: String) -> u32 {
    let mut total = 0;
    let input_info: Input = gen_input(input);
    let mut seen_points = vec![];

    for gear in input_info.gears {
        let mut numbers: Vec<u32> = vec![];
        let possibles = DIAGONALS.iter().map(|diag| gear + *diag);
        for gear_point_neighbor in possibles {
            for (number, points) in &input_info.numbers {
                if seen_points.contains(&gear_point_neighbor) {
                    continue;
                }
                if points.contains(&gear_point_neighbor) {
                    numbers.push(number.clone());
                    seen_points = points.clone();
                }
            }
        }

        if numbers.len() == 2 {
            total += numbers[0] * numbers[1];
        }
    }

    println!("Part 2 Total: {total}");
    total
}

fn main() {
    let input = open_file("input.txt");
    solve_part1(input);
    
    let input = open_file("input.txt");
    solve_part2(input);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(solve_part1(data.to_string()), 4361)
    }

    #[test]
    fn sample_02() {
        let data = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(solve_part2(data.to_string()), 467835)
    }
}