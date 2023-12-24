use std::{collections::BTreeMap};

use aoc::general::open_file;

fn parse_line(input: &str) -> (BTreeMap<&str, Node<'_>>, Vec<char>) {
    // let mut hands = vec![];
    let mut node_delim = "\n";
    let mut inst_delim = "\n\n";

    let mut tree: BTreeMap<&str, Node<'_>> = BTreeMap::new();
    let trim_pat: &[_] = &['(', ')'];
    // Difference in input data in terms of \n\n or \n\r\n
    let mut lines = input.split(inst_delim);
    for l in lines {
        println("{l}");
    }
    (tree, instructions)
}

pub fn input_generator(input: &str) -> (BTreeMap<&str, Node<'_>>, Vec<char>) {
    parse_line(input)
}

fn solve_part1(input: &str) -> u64 {
    let mut total: u64 = 0;
    let (input, moves) = input_generator(input);

    println!("Part 1 Total: {total}");
    total
}

fn solve_part2(input: &str) -> i64 {
    let total = 0;
    let (input, moves) = input_generator(input);

    println!("Part 2 Total: {total}");
    total
}

fn main() {
    let input = open_file("input.txt");
    solve_part1(&input);
    
    let input = open_file("input.txt");
    solve_part2(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part1(DATA), 2)
    }


    #[test]
    fn sample_02() {
        assert_eq!(solve_part2(DATA), 6)
    }
}