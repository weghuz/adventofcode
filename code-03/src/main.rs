fn main() {
    let lines = include_str!("../input").lines().collect::<Vec<_>>();
    println!("Total: {}", day3_part1(lines));
}

#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Number {
    number: String,
    closed: bool,
    valid: bool,
    indexes: Vec<Position>,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Number {
    fn new(number: String, index: Position) -> Self {
        Self {
            number,
            valid: false,
            closed: false,
            indexes: vec![index],
        }
    }

    fn add_number(&mut self, number: String, index: Position) {
        self.number.push_str(&number);
        self.indexes.push(index);
    }

    fn close(&mut self) {
        self.closed = true;
    }

    fn validate(&mut self) {
        self.valid = true;
    }

    fn is_valid(&self) -> bool {
        self.valid
    }
}

#[derive(Debug, Clone)]
struct Gear {
    first: Number,
    second: Number,
}

impl Gear {
    fn new(first: Number, second: Number) -> Self {
        Self { first, second }
    }
}

fn is_special_character(character: char) -> bool {
    character != '.' && !character.is_digit(10)
}

fn check_line_number(line: &str, index: usize, include_index: bool) -> bool {
    let line_1 = if index > 0 {
        match line.chars().nth(index - 1) {
            Some(character) => {
                if is_special_character(character) {
                    return true;
                }
                false
            }
            None => false,
        }
    } else {
        false
    };
    let line_2 = if include_index {
        match line.chars().nth(index) {
            Some(character) => {
                if is_special_character(character) {
                    return true;
                }
                false
            }
            None => false,
        }
    } else {
        false
    };
    let line_3 = if index < line.len() - 1 {
        match line.chars().nth(index + 1) {
            Some(character) => {
                if is_special_character(character) {
                    return true;
                }
                false
            }
            None => false,
        }
    } else {
        false
    };
    line_1 || line_2 || line_3
}

fn check_line_gear(
    line: &str,
    line_index: usize,
    char_index: usize,
    include_index: bool,
) -> Vec<Position> {
    let mut matched_positions = Vec::new();
    let mut skipping = false;
    if char_index > 0 {
        if let Some(character) = line.chars().nth(char_index - 1) {
            if character.is_digit(10) {
                skipping = true;
                matched_positions.push(Position {
                    x: line_index,
                    y: char_index - 1,
                });
            }
        }
    }
    if include_index {
        if let Some(character) = line.chars().nth(char_index) {
            if character.is_digit(10) && !skipping {
                matched_positions.push(Position {
                    x: line_index,
                    y: char_index,
                });
                skipping = true;
            } else if !character.is_digit(10) {
                skipping = false;
            }
        }
    } else {
        skipping = false;
    }
    if char_index < line.len() - 1 {
        if let Some(character) = line.chars().nth(char_index + 1) {
            // If there is a matched position from the previous one, ignore this one
            if character.is_digit(10) && !skipping {
                matched_positions.push(Position {
                    x: line_index,
                    y: char_index + 1,
                });
            }
        }
    }
    matched_positions
}

fn input_to_numbers(input: Vec<&str>) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();
    for (line_index, line) in input.iter().enumerate() {
        for (char_index, character) in line.chars().enumerate() {
            if character.is_digit(10) {
                if numbers.last().is_none() || numbers.last().unwrap().closed || char_index == 0 {
                    numbers.push(Number::new(
                        character.to_string(),
                        Position {
                            x: line_index,
                            y: char_index,
                        },
                    ));
                } else {
                    numbers.last_mut().unwrap().add_number(
                        character.to_string(),
                        Position {
                            x: line_index,
                            y: char_index,
                        },
                    );
                }

                if check_line_number(line, char_index, false) {
                    numbers.last_mut().unwrap().validate();
                }

                if line_index > 0 {
                    if check_line_number(
                        input.iter().nth(line_index - 1).unwrap(),
                        char_index,
                        true,
                    ) {
                        numbers.last_mut().unwrap().validate();
                    }
                }
                if line_index < input.len() - 1 {
                    if check_line_number(
                        input.iter().nth(line_index + 1).unwrap(),
                        char_index,
                        true,
                    ) {
                        numbers.last_mut().unwrap().validate();
                    }
                }
            } else {
                if let Some(number) = numbers.last_mut() {
                    number.close();
                }
            }
        }
    }
    numbers
}

fn input_to_gears(input: Vec<&str>, numbers: Vec<Number>) -> Vec<Gear> {
    let mut gears: Vec<Gear> = Vec::new();

    for (line_index, line) in input.iter().enumerate() {
        for (index, character) in line.chars().enumerate() {
            if character == '*' {
                let mut matches = Vec::new();
                matches.push(check_line_gear(line, line_index, index, false));
                if line_index > 0 {
                    matches.push(check_line_gear(
                        input.iter().nth(line_index - 1).unwrap(),
                        line_index - 1,
                        index,
                        true,
                    ));
                }
                if line_index < input.len() - 1 {
                    matches.push(check_line_gear(
                        input.iter().nth(line_index + 1).unwrap(),
                        line_index + 1,
                        index,
                        true,
                    ));
                }
                let matches = matches.iter().flatten().collect::<Vec<_>>();

                if matches.len() != 2 {
                    continue;
                }
                let first_position = matches.first().unwrap();
                let last_position = matches.last().unwrap();

                let first_number = numbers
                    .iter()
                    .find(|number| number.indexes.contains(first_position))
                    .unwrap();

                let second_number = numbers
                    .iter()
                    .find(|number| number.indexes.contains(last_position))
                    .unwrap();

                gears.push(Gear::new(first_number.clone(), second_number.clone()));
            }
        }
    }

    gears
}

fn day3_part1(input: Vec<&str>) -> i32 {
    let numbers = input_to_numbers(input);

    let numbers: Vec<i32> = numbers
        .iter()
        .filter(|number| number.is_valid())
        .map(|num| num.number.parse::<i32>().unwrap())
        .collect();
    let sum = numbers.iter().sum();

    sum
}

fn day3_part2(input: Vec<&str>) -> i32 {
    let numbers = input_to_numbers(input.clone());

    let gears = input_to_gears(input, numbers.clone());

    let mut total_sum = 0;
    for gear in gears {
        total_sum +=
            gear.first.number.parse::<i32>().unwrap() * gear.second.number.parse::<i32>().unwrap();
    }
    total_sum
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..",
        4361
    )]
    #[case(
            ".......=.......137.....313.........=.............998......&....*..........*.....................559..313..825=.....353....405.........296...
            ....447...........#...........342....%.....%........*..938......238.....327..............*152......@...*...................%..472.153.......
            .............152#............*......792...334......741........................570*....335..............137..........338..........*......+...
            952.........................................................793......583..........623............11........730............50.116.........446"
            ,
        10436
    )]
    #[case(
        ".......*....=...........58.991..412.42.222......................*.......79..978.....*............%.*........*58..389......616.........686...
         ....131............67.......................995............926.561......*....*....406.273...........490..611...................634$.........
         ..........908.320........................................................725.533.......*...624.....................198*246.209.........#....
         ............*...@....594..298....743...601......123......@......@606...$..............439.$.....#....../175...386.............*......490...."
    , 9694)]
    fn test_lines(#[case] input: &str, #[case] possible: i32) {
        assert_eq!(
            day3_part1(input.lines().map(|line| line.trim()).collect()),
            possible
        );
    }

    #[rstest]
    #[case(
        "467..114..
         ...*......
         ..35..633.
         ......#...
         617*......
         .....+.58.
         ..592.....
         ......755.
         ...$.*....
         .664.598..",
        467835
    )]
    #[case(
        "355.23
         32*.44
         44..32",
        0
    )]
    #[case(
        "355.23
         32*.44
         ....32",
        11360
    )]
    #[case(
        "355.23
         *2*.44
         ....3*",
        1552
    )]
    #[case(
        "...*..
         *2**44
         ....3*",
        264
    )]
    #[case(
        "...*..
         *2**44
         ..3.3*",
        138
    )]
    #[case(
        "..1*.4
         *2*.1*
         1.3.**",
        7
    )]
    #[case(
        ".....1
         ..*111
         2.2343",
        260073
    )]
    #[case(
        ".....1
         ..**1*
         2.2343",
        2343
    )]
    #[case("32*1", 32)]
    #[case(
        "..*1
         ..32",
        32
    )]
    #[case(
        "..*1
         .332",
        332
    )]
    #[case(
        "11111
         ..*..
         12345",
        137165295
    )]
    #[case(include_str!("../input"), 77509019)]
    fn test_lines_part2(#[case] input: &str, #[case] possible: i32) {
        assert_eq!(
            day3_part2(input.lines().map(|line| line.trim()).collect()),
            possible
        );
    }
}
