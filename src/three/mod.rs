use crate::util::read_lines;

#[derive(Debug)]
struct Number {
    digits: Vec<Position>,
    value: u32,
}

#[derive(Debug)]
struct Position {
    row: u32,
    col: u32,
}

pub fn solve_day_03_part_1() -> u32 {
    inner_solve_day_03_part_1(read_lines("./src/three/input.txt").unwrap())
}

fn inner_solve_day_03_part_1(lines: Vec<String>) -> u32 {
    let (numbers, specials): (Vec<Number>, Vec<Position>) = lines
        .iter()
        .enumerate()
        .map(|(index, line)| parse_line(line.as_str(), index as u32))
        .fold((Vec::new(), Vec::new()), |mut acc, (numbers, pos)| {
            acc.0.extend(numbers);
            acc.1.extend(pos);
            acc
        });

    let filtered: Vec<u32> = numbers.iter()
        .filter(|&number| {
            number.digits.iter()
                .find(|&digit| specials.iter()
                    .find(|&special|
                        (special.col == digit.col || special.col + 1 == digit.col || special.col == digit.col + 1)
                            && (special.row == digit.row || special.row + 1 == digit.row || special.row == digit.row + 1)
                    ).is_some()
                ).is_some()
        })
        .map(|number| number.value)
        .collect();


    println!("raw Numbers: {:?}", numbers);
    println!("Numbers: {:?}", numbers.iter().map(|n| n.value).collect::<Vec<u32>>());
    println!("Filtered: {:?}", filtered);

    let sum = filtered.iter().sum();
    println!("Final sum: {sum}");
    return sum;
}


pub fn solve_day_03_part_2() -> u32 {
    inner_solve_day_03_part_2(read_lines("./src/three/input.txt").unwrap())
}

fn inner_solve_day_03_part_2(lines: Vec<String>) -> u32 {
    let (numbers, specials): (Vec<Number>, Vec<Position>) = lines
        .iter()
        .enumerate()
        .map(|(index, line)| parse_line_part2(line.as_str(), index as u32))
        .fold((Vec::new(), Vec::new()), |mut acc, (numbers, pos)| {
            acc.0.extend(numbers);
            acc.1.extend(pos);
            acc
        });

    println!("{:?}", numbers);
    println!("{:?}", specials);

    let sum = specials.iter()
        .map(|special| {
            let partial_numbers: Vec<&Number> = numbers.iter().filter(|&number| number.digits.iter()
                .find(|&digit|
                        (special.col == digit.col || special.col + 1 == digit.col || special.col == digit.col + 1)
                            && (special.row == digit.row || special.row + 1 == digit.row || special.row == digit.row + 1)
                    ).is_some()
                ).collect();

            return if partial_numbers.len() != 2 {
                0
            } else {
                partial_numbers.iter().map(|n| n.value).product()
            }

        })
        .sum();

    // println!("raw Numbers: {:?}", numbers);
    // println!("Numbers: {:?}", numbers.iter().map(|n| n.value).collect::<Vec<u32>>());
    // println!("Filtered: {:?}", filtered);

    println!("Final sum: {sum}");
    return sum;
}


fn parse_line(line: &str, row: u32) -> (Vec<Number>, Vec<Position>) {
    let mut numbers: Vec<Number> = Vec::new();
    let mut specials: Vec<Position> = Vec::new();

    let mut digits_of_number: Vec<char> = Vec::new();
    for (index, char) in line.char_indices() {
        if char.is_numeric() {
            digits_of_number.push(char)
        } else {
            if !digits_of_number.is_empty() {
                let value = digits_of_number.iter()
                    .map(|&c| c.to_digit(10).unwrap())
                    .fold(0, |acc, digit| acc * 10 + digit);

                numbers.push(Number {
                    digits: digits_of_number.iter()
                        .enumerate()
                        .map(|(digit_index, &_c)| {
                            Position { row, col: (index - digit_index - 1) as u32 }
                        })
                        .collect(),
                    value,
                });

                digits_of_number.clear();
            }

            if !char.is_alphabetic() && char != '.' {
                specials.push(Position {
                    row,
                    col: index as u32,
                });
            }
        }
    }

    if !digits_of_number.is_empty() {
        let value = digits_of_number.iter()
            .map(|&c| c.to_digit(10).unwrap())
            .fold(0, |acc, digit| acc * 10 + digit);

        numbers.push(Number {
            digits: digits_of_number.iter()
                .enumerate()
                .map(|(digit_index, &_c)| {
                    Position { row, col: (line.len() - 1 - digit_index ) as u32 }
                })
                .collect(),
            value,
        });

        digits_of_number.clear();
    }

    // println!("Numbers: {:?}", numbers);
    // println!("Specials: {:?}", specials);
    return (numbers, specials);
}

fn parse_line_part2(line: &str, row: u32) -> (Vec<Number>, Vec<Position>) {
    let mut numbers: Vec<Number> = Vec::new();
    let mut specials: Vec<Position> = Vec::new();

    let mut digits_of_number: Vec<char> = Vec::new();
    for (index, char) in line.char_indices() {
        if char.is_numeric() {
            digits_of_number.push(char)
        } else {
            if !digits_of_number.is_empty() {
                let value = digits_of_number.iter()
                    .map(|&c| c.to_digit(10).unwrap())
                    .fold(0, |acc, digit| acc * 10 + digit);

                numbers.push(Number {
                    digits: digits_of_number.iter()
                        .enumerate()
                        .map(|(digit_index, &_c)| {
                            Position { row, col: (index - digit_index - 1) as u32 }
                        })
                        .collect(),
                    value,
                });

                digits_of_number.clear();
            }

            if char == '*' {
                specials.push(Position {
                    row,
                    col: index as u32,
                });
            }
        }
    }

    if !digits_of_number.is_empty() {
        let value = digits_of_number.iter()
            .map(|&c| c.to_digit(10).unwrap())
            .fold(0, |acc, digit| acc * 10 + digit);

        numbers.push(Number {
            digits: digits_of_number.iter()
                .enumerate()
                .map(|(digit_index, &_c)| {
                    Position { row, col: (line.len() - 1 - digit_index ) as u32 }
                })
                .collect(),
            value,
        });

        digits_of_number.clear();
    }

    // println!("Numbers: {:?}", numbers);
    // println!("Specials: {:?}", specials);
    return (numbers, specials);
}

#[cfg(test)]
mod tests {
    use crate::three::{inner_solve_day_03_part_1, inner_solve_day_03_part_2, parse_line, solve_day_03_part_1};
    use crate::util::read_lines;

    #[test]
    fn check_all_ascii() {
        let lines = read_lines("./src/three/input.txt").unwrap();

        for line in lines {
            for c in line.chars() {
                if !c.is_ascii() {
                    panic!("Contains other than ascii '{c}'");
                }
            }
        }
    }

    #[test]
    fn should_parse_line_with_numbers() {
        let (a, b) = parse_line("467..114..", 0);
        assert_eq!(2, a.len());
    }

    #[test]
    fn should_parse_line_with_special() {
        let (a, b) = parse_line("...*......", 0);
        assert_eq!(1, b.len());
    }

    #[test]
    fn should_parse_line_with_special_and_numbers() {
        let (a, b) = parse_line("...*...111", 0);
        assert_eq!(1, b.len());
    }

    #[test]
    fn should_parse_line_with_special_and_numbers_2() {
        let (a, b) = parse_line("617*......", 0);
        assert_eq!(1, b.len());
        assert_eq!(1, a.len());
    }

    #[test]
    fn should_parse_line_0() {
        let (a, b) = parse_line("......755.", 0);
        assert_eq!(0, b.len());
        assert_eq!(1, a.len());

        dbg!(a);
    }

    #[test]
    fn should_parse_line_1() {
        let (a, b) = parse_line("+.........34", 0);
        assert_eq!(1, b.len());
        assert_eq!(1, a.len());

        let x = "+.........34".find("34");
        println!("{:?}", x);
    }

    #[test]
    fn should_work_with_test_001() {
        let lines: Vec<String> = "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598.."
            .split("\n")
            .filter(|&line| !line.is_empty())
            .map(|line| line.trim().to_string())
            .collect();

        assert_eq!(4361, inner_solve_day_03_part_1(lines));
    }

    #[test]
    fn should_work_with_test_002() {
        let lines: Vec<String> = "
            12.......*..
            +.........34
            .......-12..
            ..78........
            ..*....60...
            78.........9
            .5.....23..$
            8...90*12...
            ............
            2.2......12.
            .*.........*
            1.1..503+.56"
            .split("\n")
            .filter(|&line| !line.is_empty())
            .map(|line| line.trim().to_string())
            .collect();

        assert_eq!(925, inner_solve_day_03_part_1(lines));
    }



    #[test]
    fn should_work_part2() {
        let lines: Vec<String> = "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598.."
            .split("\n")
            .filter(|&line| !line.is_empty())
            .map(|line| line.trim().to_string())
            .collect();

        assert_eq!(467835, inner_solve_day_03_part_2(lines));
    }
}

