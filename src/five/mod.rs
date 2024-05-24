use crate::util::read_lines;

#[derive(Debug, PartialEq)]
struct SeedMap {
    seeds: Vec<u32>,
    mappings: Vec<Vec<SourceTargetMapping>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct SourceTargetMapping {
    source_start: u32,
    destination_start: u32,
    length: u32,
}


pub fn solve_day_05_part_1() -> u32 {
    inner_solve_day_05_part_1(read_lines("./src/five/input.txt").unwrap())
}

fn inner_solve_day_05_part_1(_lines: Vec<String>) -> u32 {
    0
}

pub fn solve_day_05_part_2() -> u32 {
    inner_solve_day_05_part_2(read_lines("./src/five/input.txt").unwrap())
}

fn inner_solve_day_05_part_2(_lines: Vec<String>) -> u32 {
    0
}

fn parse(s: String) -> SeedMap {
    let mut non_empty_lines = s.lines().filter(|&line| !line.is_empty());

    let seeds: Vec<u32> = non_empty_lines
        .next().unwrap()[11..]
        .split(' ')
        .filter_map(|number| number.parse().ok())
        .collect();

    let mut mappings: Vec<Vec<SourceTargetMapping>> = vec![];
    let mut current: Vec<SourceTargetMapping> = vec![];

    while let Some(line) = non_empty_lines.next() {
        if line.ends_with(" map:") {
            if !current.is_empty() {
                mappings.push(current.clone());
            }
        } else {
            let mut mapping_chunks = line.split(' ')
                .filter_map(|number| number.parse().ok());
            let destination_range_start = mapping_chunks.next().unwrap();
            let source_range_start = mapping_chunks.next().unwrap();
            let range_length = mapping_chunks.next().unwrap();
            current.push(SourceTargetMapping {
                source_start: source_range_start,
                destination_start: destination_range_start,
                length: range_length,
            });
        }
    }

    SeedMap {
        seeds,
        mappings,
    }
}

#[cfg(test)]
mod test_part1 {
    use crate::five::{parse, SourceTargetMapping};

    // destination range start, source range start, range length.
    const TEST_INPUT: &str = "
    seeds: 79 14 55 13

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
    fn should_parse_test_input() {
        let parsed = parse(TEST_INPUT.to_string());

        println!("{:?}", parsed);

        assert_eq!(vec![79, 14, 55, 13], parsed.seeds);

        let expected_first_mappings = vec![
            SourceTargetMapping { source_start: 98, destination_start: 50, length: 2 },
            SourceTargetMapping { source_start: 50, destination_start: 52, length: 48 }
        ];
        assert_eq!(expected_first_mappings, *parsed.mappings.get(0).unwrap());
    }
}