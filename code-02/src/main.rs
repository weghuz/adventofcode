fn main() {
    let lines = include_str!("../input").lines().collect::<Vec<_>>();

    println!("Total power: {}", day2_part2(lines));
}

struct Config {
    red: i32,
    green: i32,
    blue: i32,
}

fn day2_part1(input: Vec<&str>, config: Config) -> i32 {
    // split input line on
    let mut sum = 0;
    for game in input {
        let game_name = game.split(":").next().unwrap();
        let game_id = game_name
            .split(" ")
            .skip(1)
            .next()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let mut possible = true;

        let split = game.split(":").skip(1).next().unwrap().trim();
        for set in split.split("; ") {
            for roll in set.split(", ").into_iter() {
                let split_roll = roll.split(" ").collect::<Vec<&str>>();
                let num = split_roll[0].parse::<i32>().unwrap();
                let color = split_roll[1];

                possible = match color {
                    "red" => config.red >= num,
                    "green" => config.green >= num,
                    "blue" => config.blue >= num,
                    _ => false,
                };
                if !possible {
                    break;
                }
            }
            if !possible {
                break;
            }
        }
        if possible {
            println!("{}: possible", game_id);
            println!("{}", game);
            sum += game_id;
            println!("sum: {}", sum);
        } else {
            println!("{}: impossible", game_id);
            println!("{}", game);
            println!("sum: {}", sum);
        }
    }
    sum
}

fn day2_part2(input: Vec<&str>) -> i32 {
    let mut total_power = 0;
    for game in input {
        let game_name = game.split(":").next().unwrap();
        let game_id = game_name
            .split(" ")
            .skip(1)
            .next()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let mut lowest_possible = Config {
            red: 0,
            green: 0,
            blue: 0,
        };

        let split = game.split(":").skip(1).next().unwrap().trim();
        for set in split.split("; ") {
            for roll in set.split(", ").into_iter() {
                let split_roll = roll.split(" ").collect::<Vec<&str>>();
                let num = split_roll[0].parse::<i32>().unwrap();
                let color = split_roll[1];
                match color {
                    "red" => {
                        if lowest_possible.red < num {
                            lowest_possible.red = num;
                        }
                    }
                    "green" => {
                        if lowest_possible.green < num {
                            lowest_possible.green = num;
                        }
                    }
                    "blue" => {
                        if lowest_possible.blue < num {
                            lowest_possible.blue = num;
                        }
                    }
                    _ => {}
                }
            }
        }
        total_power += lowest_possible.red * lowest_possible.green * lowest_possible.blue;
    }
    total_power
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 1)]
    #[case("game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 2)]
    #[case(
        "game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        0
    )]
    #[case(
        "game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        0
    )]
    #[case("game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 5)]
    fn test_lines(#[case] line: &str, #[case] possible: i32) {
        let config = Config {
            red: 12,
            green: 13,
            blue: 14,
        };

        assert_eq!(day2_part1(vec![line], config), possible);
    }

    #[test]
    fn sum() {
        let config = Config {
            red: 12,
            green: 13,
            blue: 14,
        };

        let lines = vec![
            "game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        assert_eq!(day2_part1(lines, config), 8);
    }

    #[test]
    fn real_input_sum() {
        let config = Config {
            red: 12,
            green: 13,
            blue: 14,
        };

        let lines = include_str!("../input").lines().collect::<Vec<_>>();

        println!(
            "Sum of numbers in the vector is {}",
            day2_part1(lines, config)
        );
    }
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    #[rstest]
    #[case("game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    #[case("game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12)]
    #[case(
        "game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        1560
    )]
    #[case(
        "game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        630
    )]
    #[case("game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36)]
    fn part_2_test_lines(#[case] line: &str, #[case] expected: i32) {
        let tot = day2_part2(vec![line]);
        println!("{}: {}", line, tot);
        assert_eq!(tot, expected);
    }

    #[test]
    fn part_2_test_input() {
        let lines = vec![
            "game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        assert_eq!(day2_part2(lines), 2286);
    }

    #[test]
    fn part_2_real_input() {
        let lines = include_str!("../input").lines().collect::<Vec<_>>();

        println!("Total power: {}", day2_part2(lines));
    }
}
