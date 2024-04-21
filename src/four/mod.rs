use std::collections::VecDeque;
use std::io::BufRead;
use std::ops::Index;
use crate::util::read_lines;

static DEBUG: bool = false;

pub fn solve_day_04_part_1() -> u32 {
    let sum = inner_solve_day_04_part_1(read_lines("./src/four/input.txt").unwrap());
    println!("Final sum: {}", sum);
    sum
}

fn inner_solve_day_04_part_1(lines: Vec<String>) -> u32 {
    lines
        .iter()
        .map(|line| parse_line(line))
        .map(|(wins, guesses)| {
            let hits: usize = wins.iter().filter(|&win| guesses.contains(win)).count();
            if DEBUG {
                println!("hits {hits}");
            }

            if hits <= 0 {
                0
            } else {
                2_u32.pow(hits as u32 - 1)
            }
        })
        .sum()
}

pub fn solve_day_04_part_2() -> u32 {
    let sum = inner_solve_day_04_part_2(read_lines("./src/four/input.txt").unwrap());
    println!("Final sum: {}", sum);
    sum
}

fn inner_solve_day_04_part_2(lines: Vec<String>) -> u32 {
    let mut duplicates: VecDeque<u32> = VecDeque::new();

    lines
        .iter()
        .map(|line| parse_line(line))
        .map(|(wins, guesses)| {
            let hits: usize = wins.iter().filter(|&win| guesses.contains(win)).count();
            if DEBUG {
                println!("hits {hits}");
            }

            let cards = duplicates.pop_front().unwrap_or(0) + 1;

            for i in 0..hits {
                if let Some(elem) = duplicates.get_mut(i) {
                    *elem += cards;
                } else {
                    duplicates.push_back(cards)
                }
            }

            cards
        })
        .sum()
}

fn parse_line(line: &str) -> (Vec<u32>, Vec<u32>) {
    // "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53" -> "41 48 83 86 17 | 83 86  6 31 17  9 48 53"
    let number_groups: Vec<Vec<u32>> = line[line.find(':').unwrap() + 2..]
        .split("|")
        .into_iter()
        .map(|numbers| numbers.trim()
            .split(" ")
            .into_iter()
            .filter(|&split| !split.is_empty())
            .map(|number| number.parse::<u32>().unwrap())
            .collect())
        .collect();

    let wins = number_groups.get(0).unwrap().clone();
    let guesses = number_groups.get(1).unwrap().clone();

    if DEBUG {
        println!("wins: {:?} - guesses: {:?}", wins, guesses);
    }

    (wins, guesses)
}

#[cfg(test)]
mod tests_part1 {
    use crate::four::{inner_solve_day_04_part_1, parse_line};
    use crate::util::read_lines;

    #[test]
    fn parse() {
        let (wins, guesses) = parse_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");

        assert_eq!(vec![41, 48, 83, 86, 17], wins);
        assert_eq!(vec![83, 86, 6, 31, 17, 9, 48, 53], guesses);
    }

    #[test]
    fn single_line_example() {
        let actual = inner_solve_day_04_part_1(vec!["Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string()]);

        assert_eq!(8, actual);
    }

    #[test]
    fn full_example() {
        let lines: Vec<String> = "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .split("\n")
            .filter(|&line| !line.is_empty())
            .map(|line| line.trim().to_string())
            .collect();

        assert_eq!(13, super::inner_solve_day_04_part_1(lines));
    }
}

#[cfg(test)]
mod tests_part2 {
    use crate::four::{inner_solve_day_04_part_2, parse_line};
    use crate::util::read_lines;

    #[test]
    fn full_example() {
        let lines: Vec<String> = "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .split("\n")
            .filter(|&line| !line.is_empty())
            .map(|line| line.trim().to_string())
            .collect();

        assert_eq!(30, super::inner_solve_day_04_part_2(lines));
    }
}
