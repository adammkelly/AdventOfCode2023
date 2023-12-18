use std::{cmp::Ordering, collections::BTreeMap};

use aoc::general::open_file;


#[derive(Debug, PartialEq)]
#[derive(Eq)]
pub struct Hand {
    cards: Vec<char>,
    bid: u64,
    overall_rank: u64,
    individual_rank: Vec<u64>
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {

        if self.overall_rank < other.overall_rank {
            return Some(Ordering::Less)
        } else if self.overall_rank > other.overall_rank {
            return Some(Ordering::Greater)
        }

        // println!("{:?} = {:?}", self.individual_rank, other.individual_rank);
        // Overall rank matches. Lets compare cards.
        for (pos, card) in self.individual_rank.iter().enumerate() {
            let other_card = other.individual_rank[pos];
            if card == &other_card {
                continue;
            } if card < &other_card {
                return Some(Ordering::Less)
            } else if card > &other_card {
                return Some(Ordering::Greater)
            }
        }
        Some(Ordering::Equal)
    }
}


fn card_rank(card: &char, replace_j: bool) -> u64 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if replace_j {
                return 1
            }
            return 11
        },
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        '1' => 1,
        _ => panic!("Unknown high card! {card}")
    }
}

fn card_type(vec_list: &Vec<(char, u64)>, replace_j: bool) -> u64 {
    let mut pairs = 0;
    let mut high_card = vec![];
    for (ind, (char, count)) in vec_list.iter().enumerate() {
        match count {
            5 => return 7,
            4 => return 6,
            3 => {
                let expected: u64 = 2;
                if vec_list[ind+1].1 == expected {
                    return 5
                }
                return 4
            },
            2 => {
                pairs += 1;
                continue;
            },
            1 => {
                // Potentially high card.
                high_card.push(card_rank(&char, replace_j));
                continue;
            }
            _ => panic!("Unknown high card!")
        };
    }
    if pairs == 2 {
        return 3
    } else if pairs == 1 {
        return 2
    } else if high_card.len() == 5 {
        let mut is_high_card = true;
        let mut seen_before = vec![];
        for (pos, val) in high_card.iter().enumerate() {
            if seen_before.contains(val) {
                is_high_card = false;
                break;
            }
            seen_before.push(*val);
        }

        if is_high_card {
            return 1
        }
    }
    return 1
}

fn get_char_count(cards_vec: &Vec<char>) -> Vec<(char, u64)> {
    let mut hmap: BTreeMap<char, u64> = BTreeMap::new();

    for c in cards_vec {
        if hmap.contains_key(&c) {
            let val = hmap.get(&c).unwrap();
            hmap.insert(*c, val + 1);
        } else {
            hmap.insert(*c, 1);
        }
    }
    let mut all_items = Vec::from_iter(hmap.clone());
    all_items.sort_by(|a, b| a.1.cmp(&b.1));
    all_items.reverse();
    return all_items
}

fn get_individual_ranks(cards_vec: Vec<char>, replace_j: bool) -> Vec<u64> {
    let mut individual_rank = vec![];

    for (index, c) in cards_vec.iter().enumerate() {
        let rank = card_rank(c, replace_j);
        individual_rank.insert(index, rank);
    }
    return individual_rank
}

fn parse_line(input: &str, replace_j: bool) -> Vec<Hand> {
    let mut hands = vec![];
    let lines = input.split('\n');
    for l in lines {
        let mut overall_rank = 0;
        let (cards, bid) = l.split_once(' ').unwrap();
        let mut new_cards = cards.to_string().clone();
        if replace_j {
            let cards_vec = cards.chars().collect::<Vec<char>>();
            let char_count = get_char_count(&cards_vec.clone());
            let mut curr_count = 0;
            let mut current_char: char = 'J';
            for (c, count) in char_count {
                let rank = card_rank(&c, true);
                if count > curr_count && rank > card_rank(&current_char, replace_j) {
                    curr_count = count;
                    current_char = c;
                }
            }
            let val = current_char.to_string().clone();
            // Replace J in cards
            new_cards = new_cards.replacen('J', &val, 5);
            let cards_vec = new_cards.chars().collect::<Vec<char>>();
            let char_count = get_char_count(&cards_vec.clone());
            overall_rank = card_type(&char_count, replace_j);
        }
        let cards_vec = cards.chars().collect::<Vec<char>>();
        let bid_64 = bid.trim().parse::<u64>().unwrap();
        let individual_rank = get_individual_ranks(cards_vec.clone(), replace_j);
        let char_count = get_char_count(&cards_vec.clone());
        if !replace_j {
            overall_rank = card_type(&char_count, replace_j);
        }
        hands.push(Hand { cards: cards_vec, bid: bid_64, overall_rank, individual_rank });
    }
    hands.sort_by(|a, b| a.cmp(&b));

    hands
}

pub fn input_generator(input: &str, replace_j: bool) -> Vec<Hand> {
    parse_line(input, replace_j)
}

fn solve_part1(input: &str) -> u64 {
    let mut total: u64 = 0;
    let input = input_generator(input, false);

    for (ind, card) in input.iter().enumerate() {
        let index = ind as u64 +1;
        total += card.bid * index;
    }

    // Expecting 251806792
    println!("Part 1 Total: {total}");
    total
}

fn solve_part2(input: &str) -> u64 {
    let mut total: u64 = 0;
    let input = input_generator(input, true);

    for (ind, card) in input.iter().enumerate() {
        let index = ind as u64 +1;
        total += card.bid * index;
    }

    // Expecting 252113488
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

    const DATA: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part1(DATA), 6440)
    }


    #[test]
    fn sample_02() {
        assert_eq!(solve_part2(DATA), 5905)
    }

    const DATA_JOKER: &str = "JAAKK 1
JJJAK 2";


    #[test]
    fn sample_03() {
        assert_eq!(solve_part2(DATA_JOKER), 5)
    }
}