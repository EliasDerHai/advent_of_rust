use crate::util::read_lines;

#[derive(Debug, PartialEq)]
struct SeedMap {
    seeds: Vec<u64>,
    mappings: Vec<Vec<SourceTargetMapping>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct SourceTargetMapping {
    source_start: u64,
    destination_start: u64,
    length: u64,
}

pub fn solve_day_05_part_1() -> u64 {
    inner_solve_day_05_part_1(read_lines("./src/five/input.txt").unwrap())
}

pub fn solve_day_05_part_2() -> u64 {
    inner_solve_day_05_part_2(read_lines("./src/five/input.txt").unwrap())
}

fn inner_solve_day_05_part_1(lines: Vec<String>) -> u64 {
    let seed_map: SeedMap = parse(lines.into_iter().collect());
    let mut cache = seed_map.seeds.clone();

    seed_map.mappings.iter().for_each(|mapping| {
        for val in cache.iter_mut() {
            *val = map_to_next_id(*val, mapping);
        }
    });

    *cache.iter().min().unwrap()
}

fn inner_solve_day_05_part_2(lines: Vec<String>) -> u64 {
    let seed_map: SeedMap = parse(lines.into_iter().collect());

    let mut cache: Vec<u64> = seed_map.seeds.chunks(2)
        .flat_map(|chunk| {
            if let [start_value, range_value] = chunk {
                (*start_value..*start_value + *range_value).collect::<Vec<_>>()
            } else {
                vec![]
            }
        }).collect();

    seed_map.mappings.iter().for_each(|mapping| {
        for val in cache.iter_mut() {
            *val = map_to_next_id(*val, mapping);
        }
    });

    *cache.iter().min().unwrap()
}

fn parse(lines: Vec<String>) -> SeedMap {
    let mut non_empty_lines = lines.into_iter().filter(|line| !line.is_empty());

    let first_line = non_empty_lines.next().unwrap();
    let seeds: Vec<u64> = first_line[first_line.find(':').unwrap()..]
        .split(' ')
        .filter_map(|number| number.parse().ok())
        .collect();

    let mut mappings: Vec<Vec<SourceTargetMapping>> = vec![];
    let mut current: Vec<SourceTargetMapping> = vec![];

    while let Some(line) = non_empty_lines.next() {
        if line.ends_with(" map:") {
            if !current.is_empty() {
                flush_mapping_cache(&mut mappings, &mut current);
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
    // flush last mapping (no more line.ends_with(" map:")
    flush_mapping_cache(&mut mappings, &mut current);

    SeedMap {
        seeds,
        mappings,
    }
}

fn flush_mapping_cache(mappings: &mut Vec<Vec<SourceTargetMapping>>, current: &mut Vec<SourceTargetMapping>) {
    mappings.push(current.clone());
    current.clear();
}

fn map_to_next_id(val: u64, mapping: &Vec<SourceTargetMapping>) -> u64 {
    mapping.iter()
        .filter(|&m| m.source_start <= val && val < m.source_start + m.length)
        .next()
        .map(|mapping| {
            let offset = val - mapping.source_start;
            mapping.destination_start + offset
        }).unwrap_or_else(|| val)
}

#[cfg(test)]
mod test {
    use crate::five::{inner_solve_day_05_part_1, inner_solve_day_05_part_2, parse, solve_day_05_part_1, solve_day_05_part_2, SourceTargetMapping};

    // destination range start, source range start, range length
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
        let lines = TEST_INPUT.to_string().lines().map(|line| line.to_string()).collect();
        let parsed = parse(lines);

        assert_eq!(vec![79, 14, 55, 13], parsed.seeds);
        assert_eq!(7, parsed.mappings.len());

        let expected_first_mappings = vec![
            SourceTargetMapping { source_start: 98, destination_start: 50, length: 2 },
            SourceTargetMapping { source_start: 50, destination_start: 52, length: 48 },
        ];
        assert_eq!(expected_first_mappings, *parsed.mappings.get(0).unwrap());

        let expected_second_mappings = vec![
            SourceTargetMapping { source_start: 15, destination_start: 0, length: 37 },
            SourceTargetMapping { source_start: 52, destination_start: 37, length: 2 },
            SourceTargetMapping { source_start: 0, destination_start: 39, length: 15 },
        ];
        assert_eq!(expected_second_mappings, *parsed.mappings.get(1).unwrap());
    }

    #[test]
    fn should_solve_day_05_part_1_example() {
        let lines = TEST_INPUT.to_string().lines().map(|line| line.to_string()).collect();
        let result = inner_solve_day_05_part_1(lines);

        assert_eq!(35, result);
    }

    #[test]
    fn should_solve_day_05_part_1() {
        let result = solve_day_05_part_1();

        assert_eq!(510109797, result);
    }

    #[test]
    fn should_solve_day_05_part_2_example() {
        let lines = TEST_INPUT.to_string().lines().map(|line| line.to_string()).collect();
        let result = inner_solve_day_05_part_2(lines);

        assert_eq!(46, result);
    }

    #[test]
    fn should_solve_day_05_part_2() {
        let result = solve_day_05_part_2();

        assert_eq!(46, result);
    }

}

