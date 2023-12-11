use aoc::general::open_file;
use nom::{
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};
use std::collections::HashMap;

// I am using this one to learn using nom.
// Full credit for this goes to:
// https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_02.rs

#[derive(Debug, Copy, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

impl std::str::FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Cube {
    color: Color,
    count: u32,
}

type Round = Vec<Cube>;
type Game = Vec<Round>;
type Games = HashMap<u32, Game>;

fn parse_id(input: &str) -> nom::IResult<&str, u32> {
    preceded(tag("Game "), complete::u32)(input)
}
fn parse_cube(input: &str) -> nom::IResult<&str, Cube> {
    let input = input.trim();
    let (input, (count, color)) =
        separated_pair(complete::u32, complete::space1, complete::alpha1)(input)?;

    let color = color.parse::<Color>().unwrap();

    Ok((input, Cube { color, count }))
}

fn parse_round(input: &str) -> nom::IResult<&str, Round> {
    separated_list1(tag(", "), parse_cube)(input)
}

fn parse_game(input: &str) -> nom::IResult<&str, Game> {
    separated_list1(tag("; "), parse_round)(input)
}

fn split_line(input: &str) -> nom::IResult<&str, (u32, Game)> {
    separated_pair(parse_id, tag(": "), parse_game)(input)
}

fn parse_line(input: &str) -> nom::IResult<&str, Vec<(u32, Game)>> {
    separated_list1(complete::line_ending, split_line)(input)
}

pub fn input_generator(input: &str) -> Games {
    let (_, games) = parse_line(input).unwrap();

    games.into_iter().collect()
}

fn solve_part1(input: String) -> u32 {
    let mut total: u32 = 0;
    let max_blue = 14;
    let max_green = 13;
    let max_red = 12;

    let x = input_generator(&input);
    for (id, game) in x {
        let is_impossible = game.iter().any(|round| {
            round.iter().any(|cube| match (cube.color, cube.count) {
                (Color::Red, count) => count > max_red,
                (Color::Green, count) => count > max_green,
                (Color::Blue, count) => count > max_blue,
            })
        });

        if !is_impossible {
            total += id;
        }
    }
    println!("Part 1 Total: {total}");
    total
}

fn solve_part2(input: String) -> u32 {
    let mut total = 0;
    let x = input_generator(&input);

    for game in x.values() {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for cube in game.iter().flatten() {
            match cube.color {
                Color::Red => min_red = cube.count.max(min_red),
                Color::Green => min_green = cube.count.max(min_green),
                Color::Blue => min_blue = cube.count.max(min_blue),
            }
        }

        total += min_red * min_green * min_blue;
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
        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(solve_part1(data.to_string()), 8)
    }

    #[test]
    fn sample_02() {
        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(solve_part2(data.to_string()), 2286)
    }
}