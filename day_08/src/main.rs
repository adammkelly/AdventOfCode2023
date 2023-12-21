use std::{collections::BTreeMap};

use aoc::general::open_file;

#[derive(Debug)]
pub struct Node<'a>{
    label: &'a str,
    left: &'a str,
    right: &'a str
}

fn locate_node<'a>(tree: &'a BTreeMap<&str, Node>, label: &str) -> Option<&'a Node<'a>> {
    tree.get(label)
}

fn move_to<'a>(n: &'a Node<'a>, step: &char) -> &'a str {
    // println!("{step}");
    match step {
        'L' => return n.left,
        'R' => return n.right,
        _ => panic!("Unknown move!")
    };
}

fn parse_line(input: &str) -> (BTreeMap<&str, Node<'_>>, Vec<char>) {
    // let mut hands = vec![];
    let mut node_delim = "\n";
    let mut inst_delim = "\n\n";

    let mut tree: BTreeMap<&str, Node<'_>> = BTreeMap::new();
    let trim_pat: &[_] = &['(', ')'];
    // Difference in input data in terms of \n\n or \n\r\n
    let mut lines = input.split(inst_delim);
    if lines.clone().collect::<Vec<&str>>().len() == 1 {
        inst_delim = "\n\r\n";
        node_delim = "\r\n";
        lines = input.split(inst_delim);
    }
    let str_collection = lines.clone().collect::<Vec<&str>>();
    // println!("{:?}", str_collection);
    let instructions = str_collection[0].trim().chars().collect::<Vec<char>>();
    let direction_nodes = str_collection[1];
    // println!("{:?}", instructions);
    for l in direction_nodes.split(node_delim) {
        let node_split = l.split(" = ").collect::<Vec<&str>>();
        let label = node_split[0].trim();
        let node_steps = node_split[1].trim_matches(trim_pat).split(", ").collect::<Vec<&str>>();
        tree.insert(label, Node { label, left: node_steps[0], right: node_steps[1] });
    }
    (tree, instructions)
}

pub fn input_generator(input: &str) -> (BTreeMap<&str, Node<'_>>, Vec<char>) {
    parse_line(input)
}

fn solve_part1(input: &str) -> u64 {
    let mut total: u64 = 0;
    let (input, moves) = input_generator(input);
    let mut start = "AAA";
    let mut total_moves: Vec<_> = moves.clone();
    for _ in 1..100 {
        total_moves.extend(moves.clone())
    }

    let end = "ZZZ";
    // println!("{:?}", input);

    for m in &total_moves {
        // println!("{start} = {:?} - {total}", total_moves.len());
        let node = locate_node(&input, start).unwrap();
        // println!("# {:?}", node);
        let _move_to = move_to(node, m);
        total += 1;
        if _move_to == end {
            break;
        }
        start = _move_to;
    }

    println!("Part 1 Total: {total}");
    total
}

fn solve_part2(input: &str) -> u64 {
    let mut total: u64 = 0;
    let (input, moves) = input_generator(input);
    let mut start_points = input.keys().filter(|&s| s.ends_with('A')).cloned().collect::<Vec<&str>>();
    println!("{:?}", start_points);
    let total_moves: Vec<_> = moves.clone();
    let mut pos: usize = 0;

    // println!("{:?}", input);
    
    'outer: while total != u64::MAX {
        let mut new_start_points = vec![];
        let mut count = 0;
        let m = total_moves.get(pos).unwrap();
        if (total % 10000) == 0 {
            println!("--- {total}");
        }
        for s in &start_points {

            //println!("{s} = {:?} - {total}", total_moves.len());
            // let node = locate_node(&input, s).unwrap();
            // println!("# {:?}", node);
            let node = &input[s];
            let _move_to = move_to(node, m);
            // println!("{total}");
            if _move_to.ends_with('Z') {
                count += 1;
                if count == start_points.len() {
                    total += 1;
                    break 'outer;
                }
            }
            new_start_points.push(_move_to);
        }
        start_points = new_start_points;
        total += 1;
        if pos == total_moves.len()-1 {
            pos = 0;
        } else {
            pos += 1;
        }
    }

    // Expecting 9606140307013 (this solution is too slow)
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

    const DATA: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part1(DATA), 2)
    }

    const DATA2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";


    #[test]
    fn sample_02() {
        assert_eq!(solve_part2(DATA2), 6)
    }
}