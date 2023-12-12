use aoc::general::open_file;
use nom::{
    bytes::streaming::take_till,
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Card {
    winning_numbers: Vec<u32>,
    game_numbers: Vec<u32>
}

type Scratchcards = BTreeMap<u32, Card>;

// Winning numbers on left, yours numbers on right.
// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11

// Thinking:
// Bit shift 0 to the left 0 << 1 to get teh doubling required in the res.



fn parse_numbers(input: &str) -> nom::IResult<&str, Vec<u32>> {
    let input = input.trim();
    separated_list1(complete::multispace1, complete::u32)(input)
}

fn split_card(input: &str) -> nom::IResult<&str, Card> {
    let (input, (winning_numbers, game_numbers)) = 
        separated_pair(parse_numbers, tag(" | "), parse_numbers)(input)?;


    Ok((input, Card {winning_numbers, game_numbers }))
}

fn parse_id(input: &str) -> nom::IResult<&str, u32> {
    let input = input.trim();
    preceded(take_till(|c: char| !c.is_whitespace() && !c.is_alphabetic()), complete::u32)(input)
}

fn split_line(input: &str) -> nom::IResult<&str, (u32, Card)> {
    separated_pair(parse_id, tag(": "), split_card)(input)
}

fn parse_line(input: &str) -> nom::IResult<&str, Vec<(u32, Card)>> {
    separated_list1(complete::line_ending, split_line)(input)
}

pub fn input_generator(input: &str) -> Scratchcards {
    let (_, games) = parse_line(input).unwrap();

    games.into_iter().collect()
}

fn solve_part1(input: String) -> u32 {
    let mut total: u32 = 0;
    let formatted_input = input_generator(&input);
    for (_, card) in formatted_input.iter() {
        let found: Vec<u32> = card.winning_numbers.clone().into_iter().filter(|winner| card.game_numbers.contains(winner)).collect();

        if found.len() > 0 {
            total += 1 << found.len()-1;
        }
    }

    println!("Part 1 Total: {total}");
    total
}

fn solve_part2(input: String) -> u32 {
    let mut total = 0;
    let originals = input_generator(&input);
    let mut cards: BTreeMap<u32, u32> = BTreeMap::new();


    for (id, _) in originals.iter() {
        cards.insert(*id, 1);
    }


    for (id, card) in originals.iter() {
        let matches: Vec<u32> = card.winning_numbers.clone().into_iter().filter(|winner| card.game_numbers.contains(winner)).collect();

        if matches.len() > 0 {
            // Clone next 2 Cards
            let matches = matches.len() as u32;
            let main_id = *id;
            let range_min = main_id+1;
            let range_max = main_id+1 + matches;

            for game_id in range_min..range_max {
                let c = *cards.get(&game_id).unwrap();
                cards.insert(game_id, c + (1 * cards[id]));
            }
        }
    }

    total = cards.values().sum();

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
        let data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(solve_part2(data.to_string()), 30)
    }
}