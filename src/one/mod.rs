use crate::util::read_lines;

pub fn solve_day_01_part_1() {
    let filename = "./src/one/input.txt";
    match read_lines(filename) {
        Ok(lines) => {
            let mut sum: u32 = 0;
            for line in lines {
                let first = line
                    .find(char::is_numeric)
                    .unwrap_or(0);
                let dec: u32 = line
                    .chars()
                    .nth(first)
                    .unwrap()
                    .to_digit(10)
                    .unwrap_or(0) * 10;

                let last = line
                    .rfind(char::is_numeric)
                    .unwrap_or(0);
                let single: u32 = line
                    .chars()
                    .nth(last)
                    .unwrap()
                    .to_digit(10)
                    .unwrap_or(0);

                sum += dec + single;
            }
            println!("Final sum: {sum}");
        }
        Err(e) => println!("Error: {}", e),
    }
}

pub fn solve_day_01_part_2() {
    let filename = "./src/one/input.txt";

    match read_lines(filename) {
        Ok(lines) => {
            calculate(lines);
        }
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use crate::one::calculate;

    #[test]
    fn should_work() {
        let lines: Vec<String> = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            .split("\n")
            .map(|s| s.to_string())
            .collect();

        assert_eq!(281, calculate(lines));
    }
}

fn calculate(lines: Vec<String>) -> u32 {
    let numbers_as_words: Vec<(&str, u32)> = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let mut sum: u32 = 0;
    for line in lines {

        // 10 - 90
        let first_numeric = line
            .find(|c: char| c.is_numeric() && c != '0')
            .map(|index| {
                let val: u32 = line
                    .chars()
                    .nth(index)
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
                (index, val)
            });

        let first_word = numbers_as_words
            .iter()
            .flat_map(|(word, val)| line.find(word).map(|index| (index, *val)))
            .min_by(|(a, _), (b, _)| a.cmp(b));

        println!("{:?} {:?}", first_numeric, first_word);
        let dec: u32 = vec![first_numeric, first_word]
            .iter()
            .flat_map(|x| *x)
            .min_by(|(numeric_index, _), (word_index, _)| numeric_index.cmp(word_index))
            .map(|(_, val)| val)
            .unwrap();

        // 1 - 9
        let last_numeric = line
            .rfind(|c: char| c.is_numeric() && c != '0')
            .map(|index| {
                let val: u32 = line
                    .chars()
                    .nth(index)
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
                (index, val)
            });

        let last_word = numbers_as_words
            .iter()
            .flat_map(|(word, val)| line.rfind(word).map(|index| (index, *val)))
            .max_by(|(a, _), (b, _)| a.cmp(b));

        let single: u32 = vec![last_numeric, last_word]
            .iter()
            .flat_map(|x| *x)
            .max_by(|(numeric_index, _), (word_index, _)| numeric_index.cmp(word_index))
            .map(|(_, val)| val)
            .unwrap();

        sum += (dec * 10) + single;
    }
    println!("Final sum: {sum}");
    return sum;
}
