use crate::util::read_lines;

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq)]
struct GameData {
    id: u32,
    rounds: Vec<(u32, Color)>,
}

pub fn solve_day_02_part_1() -> u32 {
    let games: Vec<GameData> = read_lines("./src/two/input.txt")
        .unwrap()
        .iter()
        .filter(|line| line.starts_with("Game"))
        .map(|line| get_game_data_from_line(line.trim()))
        .collect();

    let check_sum: u32 = games
        .iter()
        .filter(|game|
            game.rounds.iter().all(|round| match *round {
                (amount, Color::Red) => amount <= 12,
                (amount, Color::Green) => amount <= 13,
                (amount, Color::Blue) => amount <= 14,
            }))
        .map(|game| game.id)
        .sum();

    check_sum
}

pub fn solve_day_02_part_2() -> u32 {
    let games: Vec<GameData> = read_lines("./src/two/input.txt")
        .unwrap()
        .iter()
        .filter(|line| line.starts_with("Game"))
        .map(|line| get_game_data_from_line(line.trim()))
        .collect();

    let check_sum: u32 = games
        .iter()
        .map(|game| {
            vec![Color::Red, Color::Green, Color::Blue]
                .iter()
                .map(|curr_color|
                    game.rounds.iter()
                        .filter(|(_, c)| c == curr_color)
                        .map(|(a, _)| *a)
                        .max()
                        .unwrap()
                ).product::<u32>()
        })
        .sum();

    check_sum
}

fn get_game_data_from_line(line: &str) -> GameData {
    let end_of_header = line.find(':').unwrap();
    let game_id: u32 = line[5..end_of_header].parse().unwrap();

    let tail = &line[end_of_header + 1..];
    let rounds: Vec<(u32, Color)> = tail
        .trim()
        .split(';')
        .flat_map(|round| round
            .trim()
            .split(',')
            .map(deconstruct_result)
        ).collect();

    return GameData {
        id: game_id,
        rounds,
    };
}

fn deconstruct_result(result: &str) -> (u32, Color) {
    let result: Vec<&str> = result
        .trim()
        .split(' ')
        .collect();
    let amount: u32 = result.get(0).unwrap().parse().unwrap();
    let color = match *result.get(1).unwrap() {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        other => panic!("Unknown color: {}", other)
    };

    (amount, color)
}


#[cfg(test)]
mod tests {
    use crate::two::{Color, GameData, get_game_data_from_line, solve_day_02_part_1, solve_day_02_part_2};

    #[test]
    fn should_parse_correctly() {
        // Arrange
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected: GameData = GameData {
            id: 1,
            rounds: vec![
                (3, Color::Blue),
                (4, Color::Red),
                (1, Color::Red),
                (2, Color::Green),
                (6, Color::Blue),
                (2, Color::Green),
            ],
        };

        // Act
        let actual = get_game_data_from_line(line);

        // Assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_solve_part_1() {
        // Arrange
        let expected = 2632;

        // Act
        let actual = solve_day_02_part_1();

        // Assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_solve_part_2() {
        // Arrange
        let expected = 69629;

        // Act
        let actual = solve_day_02_part_2();

        // Assert
        assert_eq!(expected, actual);
    }
}