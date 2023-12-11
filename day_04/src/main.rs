use aoc::general::open_file;
use aoc::point::*;
use std::collections::HashSet;


fn solve_part1(input: String) -> u32 {
    let mut total: u32 = 0;   

    println!("Part 1 Total: {total}");
    total
}

fn solve_part2(input: String) -> u32 {
    let mut total = 0;

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
        let data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(solve_part1(data.to_string()), 13)
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