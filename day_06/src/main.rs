use aoc::general::open_file;
use nom::{
    bytes::streaming::take_till,
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    sequence::{preceded, separated_pair}, FindSubstring, combinator::complete,
};

#[derive(Debug)]
pub struct Race {
    time: u64,
    distance: u64
}

fn parse_line(input: &str) -> Vec<Race> {
    let mut races = vec![];
    let (times, distances) = input.split_once('\n').unwrap();
    let time_vals = times.split(":").nth(1).unwrap().split_whitespace().map(|s| s.trim().parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let dist_vals = distances.split(":").nth(1).unwrap().split_whitespace().map(|s| s.trim().parse::<u64>().unwrap()).collect::<Vec<u64>>();

    for i in 0..time_vals.len() {
        races.push(Race {time: time_vals[i], distance: dist_vals[i]});
    }
    races
}

pub fn input_generator(input: &str) -> Vec<Race> {
    parse_line(input)
}

fn distance_per_second(throttle_held_for: u64) -> u64 {
    1 * throttle_held_for
}

fn travel_time(throttle_held_for: u64, total_time: u64) -> u64 {
    total_time - throttle_held_for
}

fn combinations(race: &Race) -> Vec<u64> {
    let mut comb = vec![];
    let total_time = race.time;
    let distance_to_beat = race.distance;

    for i in 1..total_time {
        let d = distance_per_second(i) * travel_time(i, total_time);
        if d > distance_to_beat {
            comb.push(d);
        }
    }
    comb
}

fn solve_part1(input: &str) -> u64 {
    let mut total: u64 = 0;
    let mut combos = vec![];
    let formatted_input = input_generator(input);

    for race in &formatted_input {
        let c = combinations(race).len() as u64;
        combos.push(c);
    }

    for c in &combos {
        if total == 0 {
            total = combos[0];
        } else {
            total *= c;
        }
    }

    println!("Part 1 Total: {total}");
    total
}

fn solve_part2(input: &str) -> u64 {
    let mut total: u64 = 0;
    let mut combos = vec![];
    let formatted_input = input_generator(input);
    let mut time_str = String::new();
    let mut dist_str = String::new();

    for race in &formatted_input {
        time_str += &race.time.to_string();
        dist_str += &race.distance.to_string();
    }

    let race = Race { time: time_str.parse().unwrap(), distance: dist_str.parse().unwrap() };
    total = combinations(&race).len() as u64;

    for c in &combos {
        if total == 0 {
            total = combos[0];
        } else {
            total *= c;
        }
    }
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

    const DATA: &str = "Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part1(DATA), 288)
    }

    #[test]
    fn sample_02() {
        assert_eq!(solve_part2(DATA), 71503)
    }
}