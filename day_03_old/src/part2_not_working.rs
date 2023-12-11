use std::fs;
use std::collections::BTreeMap;


#[derive(Debug, Clone)]
struct CharPos {
    val: char,
    line: usize,
    index: usize,
    left: usize,
    right: usize,
    valid_symbol: bool
}


#[derive(Debug, Clone)]
struct ValidNumbers {
    val: i32,
    neighbors: Vec<ValidNumbers>
}

fn get_file() -> String {
    return fs::read_to_string("input.txt").expect("Unable to read file");
}

fn valid_symbol(c: char) -> bool {
    c == '*'
}

fn current_left(pos: &usize) -> usize {
    let mut left = usize::MAX;
    if *pos != 0 {
        left = *pos - 1;
    }
    left
}

fn current_right(pos: &usize, total: &usize) -> usize {
    let mut right = usize::MAX;
    if *pos < *total {
        right = *pos + 1;
    }
    right
}

fn find_end_of_int(char_buff: &BTreeMap<usize, CharPos>, char_pos: &usize, total: usize) -> (usize, i32) {
    let mut end = *char_pos;
    let mut cpos = char_buff.get(&end).unwrap();
    let mut num_str = String::new();
    while cpos.val.is_ascii_digit() {
        num_str = num_str + &cpos.val.to_string();
        if end >= total {
            break;
        }
        end += 1;
        cpos = char_buff.get(&end).unwrap();
    }
    (end, num_str.parse().unwrap())
}

fn get_neighbors(line_tree: &BTreeMap<usize, BTreeMap<usize, CharPos>>, left: &usize, right: &usize, line_num: &usize, total_char_len: &usize) -> Vec<ValidNumbers> {
    let total = line_tree.len()-1;
    let mut line_range = vec![];
    let mut neighbors: Vec<ValidNumbers> = vec![];
    let mut l_range = *left;
    let mut r_range = *right;
    if *line_num > 0 {
        line_range.push(*line_num-1);
    }
    line_range.push(*line_num);
    if *line_num < total {
        line_range.push(*line_num+1);
    }
    if l_range == usize::MAX {
        l_range = 0;
    }
    if r_range == usize::MAX || r_range > total_char_len-1 {
        r_range = *total_char_len-1;
    }

    for l in line_range {
        println!("----------");
        let mut start = 0;
        let mut partial_str = String::new();
        for pos in l_range..r_range {
            if (l != *line_num) || (l == *line_num && pos == l_range || pos == r_range-1) {
                println!("{l}: {pos} {:?}", line_tree.get(&l).unwrap());
                let char_pos = line_tree.get(&l).unwrap().get(&pos).unwrap().clone();
                if char_pos.val.is_ascii_digit() {
                    start = pos;
                    let mut temp_char = line_tree.get(&l).unwrap().get(&pos).unwrap().clone();
                    while temp_char.val.is_ascii_digit() {
                        println!("GOT start: {start} {:?}", temp_char);
                        if temp_char.left == usize::MAX {
                            println!("EXIT1 start: {start} {:?}", temp_char);
                            break;
                        }
                        temp_char = line_tree.get(&l).unwrap().get(&temp_char.left).unwrap().clone();
                        if temp_char.val.is_ascii_digit() {
                            if temp_char.left == usize::MAX {
                                println!("EXIT2 start: {start} {:?}", temp_char);
                                break;
                            }
                            start = temp_char.left;
                            println!("GOT: {start} {:?}", temp_char);
                        }
                    }
                    println!("GOTEX: {start} {:?}", temp_char);
                    temp_char = line_tree.get(&l).unwrap().get(&start).unwrap().clone();
                    while temp_char.val.is_ascii_digit() {
                        if  temp_char.right == usize::MAX {
                            break;
                        }
                        partial_str = partial_str + &temp_char.val.to_string();
                        temp_char = line_tree.get(&l).unwrap().get(&temp_char.right).unwrap().clone();
                        if temp_char.val.is_ascii_digit() {
                            if temp_char.right == usize::MAX {
                                break;
                            }
                        }
                    }
                    if !partial_str.is_empty() {
                        let current_val = partial_str.parse().unwrap();
                        let exists = neighbors.iter().find(|&n| n.val == current_val).is_some();
                        if !exists {
                            neighbors.push(ValidNumbers {val: current_val, neighbors: vec![]});
                        }
                        partial_str.clear();
                    }
                }
            }
        }
    }
    println!("EOF ----------");
    neighbors
}

fn get_line_chars(line: &str, line_num: &usize) -> BTreeMap<usize, CharPos> {
    let mut tree = BTreeMap::new();
    let curr_line = String::from(line);
    let chars_in_line = curr_line.char_indices();
    let total_chars: usize = chars_in_line.clone().count() - 1;
    for (pos, char) in chars_in_line {
        let left = current_left(&pos);
        let right = current_right(&pos, &total_chars);
        let valid_symbol = valid_symbol(char);
        tree.insert(pos, CharPos{val: char, index: pos, left, right, valid_symbol, line: *line_num});
    }
    tree
}
fn solve(file: String) -> i32 {
    let mut total: i32 = 0;
    let mut line_tree = BTreeMap::new();
    let mut valid_nums = vec![];
    let mut line_num = 0;

    // Sort into tree.
    for line in file.lines() {
        let chars = get_line_chars(line, &line_num);
        line_tree.insert(line_num, chars);
        line_num += 1;
    }
    
    for (line_num, chars) in &line_tree {

        let mut right = 0;
        let mut num_val:i32 = 0;
        for (char_pos, obj) in chars {
            if obj.valid_symbol {
                // locate neighbors
                let neighbors = get_neighbors(&line_tree, &obj.left, &obj.right, line_num, &chars.len());
                // Add number
                valid_nums.push(ValidNumbers{val: num_val, neighbors})
            }
        }
    }

    for valid_num in valid_nums {
        // let has_symbol = valid_num.neighbors.iter().any(|f| f.valid_symbol);
        // if has_symbol {
        //     total += valid_num.val;
        // }
        println!("{:?}", valid_num);
        if valid_num.neighbors.len() == 2 {
            total += valid_num.neighbors[0].val * valid_num.neighbors[1].val;
        }
    }
    println!("Res: {}", total);
    total
}

fn main() {
    let file: String = get_file();
    solve(file);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";

        assert_eq!(solve(data.to_string()), 4361)
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

        assert_eq!(solve(data.to_string()), 457835)
    }
}