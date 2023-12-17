use aoc::general::open_file;
use std::collections::BTreeMap;
use indicatif::ProgressIterator;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[derive(Ord, Eq)]
pub enum ParserState {
    Seeds = 1,
    SeedToSoil = 2,
    SoilToFertilizer = 3,
    FertilizerToWater = 4,
    WaterToLight = 5,
    LightToTemp = 6,
    TempToHumidity = 7,
    HumidityToLoc = 8
}

static PARSER_ORDER: [ParserState; 7] = [
    ParserState::SeedToSoil,
    ParserState::SoilToFertilizer,
    ParserState::FertilizerToWater,
    ParserState::WaterToLight,
    ParserState::LightToTemp,
    ParserState::TempToHumidity,
    ParserState::HumidityToLoc
];

#[derive(Debug, Clone)]
pub struct SoilItemMap {
    parsed_type: ParserState,
    source_range_start: u64,
    dest_range_start: u64,
    range_length: u64
}

impl SoilItemMap {
    fn check_bounds(&self, location: u64) -> bool {
        let start = self.source_range_start;
        let end = self.source_range_start + self.range_length;

        if location >= start && location <= end {
            return true;
        }

        false
    }

    fn get_dest(&self, location: u64) -> u64 {
        (location + self.dest_range_start)-self.source_range_start
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Seeds {
    parsed_type: ParserState,
    number: u64
}

type SoilMaps = BTreeMap<u64, String>;

// Winning numbers on left, yours numbers on right.
// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11

// Thinking:
// Bit shift 0 to the left 0 << 1 to get teh doubling required in the res.

pub fn input_parse_seeds(input: &str) -> Vec<Seeds> {
    let mut seed_list = vec![];
    for seed in input.split(" ") {
        seed_list.push(Seeds { parsed_type: ParserState::Seeds, number: seed.trim().parse::<u64>().unwrap() });
    }
    seed_list
}

pub fn get_parser_state(input: &str) -> ParserState {
    match input {
        "seed-to-soil" => ParserState::SeedToSoil,
        "soil-to-fertilizer" => ParserState::SoilToFertilizer,
        "fertilizer-to-water" => ParserState::FertilizerToWater,
        "water-to-light" => ParserState::WaterToLight,
        "light-to-temperature" => ParserState::LightToTemp,
        "temperature-to-humidity" => ParserState::TempToHumidity,
        "humidity-to-location" => ParserState::HumidityToLoc,
        _ => panic!("Never should her here.")
    }
}

pub fn parser_soil_item(parsed_type: ParserState, input: &str, rev: bool) -> SoilItemMap {
    let numbers = input.trim().split(" ").collect::<Vec<&str>>();
    let mut dest_range_start = numbers[0].parse().unwrap();
    let mut source_range_start = numbers[1].parse().unwrap();
    if rev {
        dest_range_start = numbers[1].parse().unwrap();
        source_range_start = numbers[2].parse().unwrap();
    }
    let range_length = numbers[2].parse().unwrap();
    SoilItemMap {
        parsed_type,
        source_range_start,
        dest_range_start,
        range_length
    }
}

pub fn generate_global_hashmap(map_items: &Vec<SoilItemMap>) -> BTreeMap<ParserState, Vec<SoilItemMap>> {
    let mut tree: BTreeMap<ParserState, Vec<SoilItemMap>> = BTreeMap::new();

    for map_i in map_items {

        let tree_key = &map_i.parsed_type;
        if !tree.contains_key(tree_key) {
            tree.insert(*tree_key, vec![]);
        }

        let mut sub_tree = tree.get(tree_key).unwrap().clone();
        sub_tree.push(map_i.clone());

        tree.insert(*tree_key, sub_tree);
    }

    tree
}

pub fn input_generator_format(input: &str) -> Vec<&str> {
    let parts = input.split("\n\n");
    let collection = parts.collect::<Vec<&str>>();
    let mut collection2 = vec![];
    for part in collection {
        let parts2 = part.split("\n");
        for part2 in parts2 {
            collection2.push(part2);
        }
    }
    collection2
}

pub fn input_generator(input: &str, rev: bool) -> (Vec<Seeds>, Vec<SoilItemMap>) {
    let mut map_items = vec![];
    let mut seed_num_list = vec![];
    let mut curr_state: ParserState = ParserState::Seeds;
    let input_fmt = input_generator_format(input);
    for substr in input_fmt {
        let is_seeds = substr.starts_with("seeds:");
        match is_seeds {
            true => {
                curr_state = ParserState::Seeds;
                let seed_numbers = substr.split("seeds: ").nth(1).unwrap();
                seed_num_list = input_parse_seeds(seed_numbers);
                continue;
            },
            false => {
                if substr.find("map:").is_some() {
                    // Just set the state here and continue;
                    let map_type = substr.split(" map:").nth(0).unwrap();
                    curr_state = get_parser_state(map_type);
                    continue;
                }
                if substr.trim().is_empty() {
                    continue;
                }
            },
        }
        // If we get here - We know its a map item of some kind.
        map_items.push(parser_soil_item(curr_state.clone(), substr, rev));
    }
    (seed_num_list, map_items)
}

fn get_next_value(global_map: &BTreeMap<ParserState, Vec<SoilItemMap>>, sub_map: ParserState, key: u64) -> u64 {
    let top_item = global_map.get(&sub_map);
    if top_item.is_some() {
        let tree_item = top_item;
        if tree_item.is_some() {
            for item in tree_item.unwrap() {
                if item.check_bounds(key) {
                    return item.get_dest(key)
                }
            }
        }
    }
    key
}

fn solve_part1(input: String) -> u64 {
    let mut total: u64 = u64::MAX;
    let (seeds, maps) = input_generator(&input, false);
    let global_hashmap: BTreeMap<ParserState, Vec<SoilItemMap>> = generate_global_hashmap(&maps);

    for s in seeds {
        let mut start = s.number;
        for tree_pos in PARSER_ORDER {
            // Get from map and pass on as key to next one.
            // Also return the number as passed in if not found.
            start = get_next_value(&global_hashmap, tree_pos, start);
        }

        if start < total {
            total = start;
        }
    }

    println!("Part 1 Total: {total}");
    total
}

fn solve_part2(input: String) -> u64 {
    let mut total: u64 = u64::MAX;
    // TODO: Revisit for a different approach, currently brute force
    // Use --release to speed up the process.
    let (seeds, maps) = input_generator(&input, false);
    let global_hashmap: BTreeMap<ParserState, Vec<SoilItemMap>> = generate_global_hashmap(&maps);
    let seeds_mesh= seeds.chunks(2).map(|c| (c[0],c[1]));

    for (s1, s2) in seeds_mesh.progress() {
        let _s_range = (s1.number..(s1.number+s2.number)).clone().into_iter().collect::<Vec<u64>>();
        for _s in _s_range.iter().progress() {
            let mut start = *_s;
            for tree_pos in PARSER_ORDER {
                // Get from map and pass on as key to next one.
                // Also return the number as passed in if not found.
                start = get_next_value(&global_hashmap, tree_pos, start);
            }

            if start < total {
                println!("Total: {start}");
                total = start;
            }
        }
    }

    // 1240035 is the answer but we are off by 1 (1240036)
    // Need to understand why
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

    const DATA: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part1(DATA.to_string()), 35)
    }

    #[test]
    fn sample_02() {
        assert_eq!(solve_part2(DATA.to_string()), 46)
    }
}