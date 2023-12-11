use std::fs;
use std::collections::HashMap;
use nom::{
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

// Day 2 was quite a step up.
// More understanding how this works exactly.
// Credit: https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_02.rs

fn get_file() -> String {
    return fs::read_to_string("input.txt").expect("Unable to read file");
}

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

fn parse_line(input: &str) -> nom::IResult<&str, (u32, Game)> {
    separated_pair(parse_id, tag(": "), parse_game)(input)
}

fn parse_input(input: &str) -> nom::IResult<&str, Vec<(u32, Game)>> {
    separated_list1(complete::line_ending, parse_line)(input)
}

fn input_generator(input: &str) -> Games {
    let (_, games) = parse_input(input).unwrap();

    games.into_iter().collect()
}

fn main() {
    let mut power_sum = 0;
    let input: String = get_file();
    for (id, game) in input_generator(&input) {
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

        power_sum += min_red * min_green * min_blue;
    }
    println!("Result: {power_sum}")
}
