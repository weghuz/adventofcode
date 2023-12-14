fn main() {
    let input = include_str!("../input").lines().collect::<Vec<_>>();

    day_1_part_2(input);
}

fn day_1_part_2(input: Vec<&str>) -> i32 {
    let array_strings = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut sum = 0;

    for line in input {
        let mut num = String::new();
        let mut first = (0, 0);
        let mut last = (0, 0);
        let mutable_line = line.to_string();

        let mut first_word = (0, 0);
        let mut last_word = (0, 0);

        for (index, word) in array_strings.iter().enumerate() {
            if mutable_line.contains(word) {
                let word_index = mutable_line.find(word).unwrap();
                if word_index < first_word.1 || first_word.0 == 0 {
                    first_word = (index + 1, word_index as usize);
                }
                let word_index = mutable_line.rfind(word).unwrap();
                if word_index > last_word.1 || last_word.0 == 0 {
                    last_word = (index + 1, word_index as usize);
                }
            }
        }

        for (index, character) in mutable_line.chars().enumerate() {
            if character.is_numeric() {
                if first.0 == 0 {
                    first = (character.to_digit(10).unwrap(), index);
                } else {
                    last = (character.to_digit(10).unwrap(), index);
                }
            }
        }

        if last.0 == 0 {
            last = first;
        }
        if (first.1 > first_word.1 && first_word.0 != 0) || first.0 == 0 {
            first = (first_word.0 as u32, first_word.1);
        }
        if last.1 < last_word.1 || (last.0 == 0 && first.0 != 0) {
            last = (last_word.0 as u32, last_word.1);
        }
        num.push(first.0.to_string().chars().next().unwrap());
        num.push(last.0.to_string().chars().next().unwrap());

        println!("{}", line);
        println!("answer: {}", num);

        sum += num.parse::<i32>().unwrap();
    }

    println!("Sum of numbers in the vector is {}", sum);

    return sum;
}

#[cfg(test)]
mod tests {

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    fn test_part_2_cases(#[case] line: &str, #[case] expected: i32) {
        assert_eq!(day_1_part_2(vec![line]), expected);
    }
}
