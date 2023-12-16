fn main() {
    let lines = include_str!("../input").lines().collect::<Vec<_>>();
    println!("Total part1: {}", day3_part1(lines.clone()));
    println!("Total part2: {}", day3_part2(lines));
}

fn get_game_and_cards(input: &str) -> (usize, &str) {
    let mut split = input.split(":");
    let game: usize = split
        .next()
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as usize;
    let cards = split.next().unwrap();
    (game, cards)
}

fn get_winning_numbers_and_cards(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut split = input.split("|");
    let trimmed_winners = split.next().unwrap().trim();
    let winning_numbers = trimmed_winners
        .split(" ")
        .filter(|n| !n.is_empty())
        .map(|n| n.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let trimmed_cards = split.next().unwrap().trim().split(" ");

    let cards = trimmed_cards
        .filter(|n| !n.is_empty())
        .map(|n| {
            return n.trim().parse::<usize>().unwrap();
        })
        .collect::<Vec<_>>();
    (winning_numbers, cards)
}

fn solve_game_part1(input: &str) -> i32 {
    let (game, cards) = get_game_and_cards(input);
    let (winning_numbers, cards) = get_winning_numbers_and_cards(cards);
    let mut wins = 0;
    for (_i, card) in cards.iter().enumerate() {
        if winning_numbers.contains(card) {
            wins += 1;
        }
    }
    let base: i32 = 2;
    let result = if wins > 0 { base.pow(wins - 1) } else { 0 };

    result
}

fn solve_row_game_part2(input: &str) -> usize {
    let (game, cards) = get_game_and_cards(input);
    let (winning_numbers, cards) = get_winning_numbers_and_cards(cards);
    let mut wins = 0;
    for card in cards {
        if winning_numbers.contains(&card) {
            wins += 1;
        }
    }

    println!("Game: {}, Wins: {}", game, wins);

    wins
}

fn day3_part1(input: Vec<&str>) -> i32 {
    let mut result = 0;
    for line in input {
        result += solve_game_part1(line);
    }
    result
}

fn day3_part2(input: Vec<&str>) -> usize {
    let mut wins = vec![1; input.len()];
    for (index, line) in input.iter().enumerate() {
        let runs = wins[index];
        println!("Index: {}, Runs: {}", index, runs);
        let current_wins = solve_row_game_part2(line);
        for i in index..current_wins + index {
            println!("i: {}, index: {}, runs: {}", i, index, runs);
            wins[i + 1] += runs;
        }
    }
    println!("Wins: {:?}", wins);
    wins.iter().sum()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
         Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
         Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
         Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
         Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
         Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        13
    )]
    #[case(include_str!("../input"), 0)]
    fn test_lines_part1(#[case] input: &str, #[case] possible: i32) {
        assert_eq!(
            day3_part1(input.lines().map(|line| line.trim()).collect()),
            possible
        );
    }

    #[rstest]
    #[case(
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        30
    )]
    #[case(include_str!("../input"), 0)]
    fn test_lines_part2(#[case] input: &str, #[case] possible: usize) {
        assert_eq!(
            day3_part2(input.lines().map(|line| line.trim()).collect()),
            possible
        );
    }
}
