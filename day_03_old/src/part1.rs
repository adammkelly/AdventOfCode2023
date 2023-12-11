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
    neighbors: Vec<CharPos>
}

fn get_file() -> String {
    return fs::read_to_string("input.txt").expect("Unable to read file");
}

fn valid_symbol(c: char) -> bool {
    !char::is_numeric(c) && c != '.'
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

fn get_neighbors(line_tree: &BTreeMap<usize, BTreeMap<usize, CharPos>>, left: &usize, right: &usize, line_num: &usize, total_char_len: &usize) -> Vec<CharPos> {
    let total = line_tree.len()-1;
    let mut line_range = vec![];
    let mut neighbors = vec![];
    let mut l_range = *left;
    let mut r_range = *right+1;
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
    if r_range == usize::MAX {
        r_range = *total_char_len-1;
    }

    for l in line_range {
        for pos in l_range..r_range {
            // Gotcha here that range doesnt appear to iterate over the last number in the list.
            if (l != *line_num) || (l == *line_num && pos == l_range || pos == r_range-1) {
                neighbors.push(line_tree.get(&l).unwrap().get(&pos).unwrap().clone());
            }
        }
    }
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

fn main() {
    let mut total: i32 = 0;
    let mut line_tree = BTreeMap::new();
    let mut valid_nums = vec![];
    let file: String = get_file();
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
            if obj.val.is_ascii_digit() && *char_pos >= right {
                (right, num_val) = find_end_of_int(&chars, &char_pos, chars.len()-1);
                // locate neighbors
                let neighbors = get_neighbors(&line_tree, &obj.left, &right, line_num, &chars.len());
                // Add number
                valid_nums.push(ValidNumbers{val: num_val, neighbors})
            }
        }
    }

    for valid_num in valid_nums {
        let has_symbol = valid_num.neighbors.iter().any(|f| f.valid_symbol);
        if has_symbol {
            total += valid_num.val;
        }
    }
    println!("Res: {}", total);
}
