use std::{cmp::max, cmp::min};

fn main() {
    let lines = include_str!("../input").lines().collect::<Vec<_>>();
    println!("Total part1: {}", day5_part1(lines));
}

#[derive(Debug)]
struct Seed<'a> {
    value: isize,
    seed_type: &'a str,
}

#[derive(Debug)]
struct SeedRange<'a> {
    start: isize,
    end: isize,
    seed_type: &'a str,
}

#[derive(Debug)]
struct Converter {
    destination: isize,
    source: isize,
    length: isize,
}

#[derive(Debug)]
struct Map {
    converters: Vec<Converter>,
    from: String,
    to: String,
}

fn get_maps(input: Vec<&str>) -> Vec<Map> {
    let mut maps: Vec<Map> = Vec::new();
    for (_, &line) in input.iter().skip(2).enumerate() {
        if line == "" {
            continue;
        }
        if !&line[0..1].chars().next().unwrap().is_digit(10) {
            let split_dash_line = line
                .split(" ")
                .next()
                .unwrap()
                .split("-")
                .collect::<Vec<_>>();
            maps.push(Map {
                converters: Vec::new(),
                from: split_dash_line[0].to_string(),
                to: split_dash_line[2].to_string(),
            });
            continue;
        } else {
            let split_line = line.split(" ").collect::<Vec<_>>();
            let converter = Converter {
                destination: split_line[0].parse::<isize>().unwrap(),
                source: split_line[1].parse::<isize>().unwrap(),
                length: split_line[2].parse::<isize>().unwrap(),
            };
            let map = maps.last_mut().unwrap();
            map.converters.push(converter);
        }
    }
    maps
}

fn get_seeds_initial(input: &str) -> Vec<Seed> {
    input
        .split_whitespace()
        .map(|s| Seed {
            seed_type: "seed",
            value: s.parse::<isize>().unwrap(),
        })
        .collect()
}

fn get_seed_ranges(seeds: Vec<Seed>) -> Vec<SeedRange> {
    let mut seed_ranges: Vec<SeedRange> = Vec::new();
    for i in 0..seeds.len() / 2 {
        let start_pos = seeds[i * 2].value;
        let range_length = seeds[(i * 2) + 1].value;
        seed_ranges.push(SeedRange {
            seed_type: "seed",
            start: start_pos,
            end: start_pos + range_length,
        });
    }
    seed_ranges
}

fn day5_part1(input: Vec<&str>) -> isize {
    let split_input = input[0].split(": ").nth(1).unwrap();
    let mut seeds = get_seeds_initial(split_input);

    let maps: Vec<Map> = get_maps(input);
    for map in maps.iter() {
        for converter in map.converters.iter() {
            for seed in seeds.iter_mut() {
                // Restore the first solution
                if converter.length > 0
                    && seed.value >= converter.source
                    && seed.value <= converter.source + converter.length
                    && seed.seed_type == map.from
                {
                    seed.value = converter.destination + (seed.value - converter.source);
                    seed.seed_type = &map.to;
                }
            }
        }
        for seed in seeds.iter_mut().filter(|s| s.seed_type == map.from) {
            seed.seed_type = &map.to;
        }
    }

    println!("Seeds: {:?}", seeds);

    seeds.iter().map(|s| s.value).min().unwrap()
}

fn day5_part2(input: Vec<&str>) -> isize {
    let split_input = input[0].split(": ").nth(1).unwrap();
    let initial_seeds = get_seeds_initial(split_input);
    let mut seeds = get_seed_ranges(initial_seeds);

    let maps: Vec<Map> = get_maps(input);
    for map in maps.iter() {
        let mut new_ranges: Vec<SeedRange> = Vec::new();
        for converter in map.converters.iter() {
            for seed in seeds.iter_mut() {
                if converter.length > 0
                    && seed.end >= converter.source
                    && seed.end <= converter.source + converter.length
                    && seed.seed_type == map.from
                {
                    let intersecting_range_start = max(seed.start, converter.source);
                    let intersecting_range_end = seed.end;
                    let movement = converter.destination - converter.source;

                    if intersecting_range_start == seed.start && intersecting_range_end == seed.end
                    {
                        seed.start = seed.start + movement;
                        seed.end = seed.end + movement;
                    } else {
                        seed.end = intersecting_range_start;
                        new_ranges.push(SeedRange {
                            seed_type: &map.to,
                            start: intersecting_range_start + movement,
                            end: intersecting_range_end + movement,
                        });
                    }
                } else if converter.length > 0
                    && seed.start <= converter.source + converter.length
                    && seed.start >= converter.source
                    && seed.seed_type == map.from
                {
                    let intersecting_range_start = seed.start;
                    let intersecting_range_end = min(seed.end, converter.source + converter.length);
                    let movement = converter.destination - converter.source;

                    if intersecting_range_start == seed.start && intersecting_range_end == seed.end
                    {
                        seed.start = seed.start + movement;
                        seed.end = seed.end + movement;
                    } else {
                        seed.start = intersecting_range_end;
                        new_ranges.push(SeedRange {
                            seed_type: &map.to,
                            start: intersecting_range_start + movement,
                            end: intersecting_range_end + movement,
                        });
                    }
                }
            }
        }
        for seed in seeds.iter_mut().filter(|s| s.seed_type == map.from) {
            seed.seed_type = &map.to;
        }
        seeds.append(&mut new_ranges);
    }

    println!("Seeds: {:?}", seeds);

    seeds.iter().map(|s| s.start).min().unwrap()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "seeds: 79 14 55 13

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
        56 93 4",
        35
    )]
    #[case(include_str!("../input"), 510109797)]
    fn test_lines_part1(#[case] input: &str, #[case] possible: isize) {
        assert_eq!(
            day5_part1(input.lines().map(|line| line.trim()).collect()),
            possible
        );
    }

    #[rstest]
    #[case(
        "seeds: 79 14 55 13

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
        56 93 4",
        35
    )]
    #[case(include_str!("../input"), 0)]
    fn test_lines_part2(#[case] input: &str, #[case] possible: isize) {
        assert_eq!(
            day5_part2(input.lines().map(|line| line.trim()).collect()),
            possible
        );
    }
}
