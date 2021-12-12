use std::{error::Error, fs};

const FILENAME: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(FILENAME)?;
    let positions = parse_crab_positions(&input);
    println!(
        "Part 1 : {}",
        best_consumption(&positions, part1_consumption)
    );
    println!(
        "Part 2 : {}",
        best_consumption(&positions, part2_consumption)
    );
    Ok(())
}

fn parse_crab_positions(data: &str) -> Vec<i32> {
    data.split(',')
        .map(str::trim)
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn part1_consumption(pos1: i32, pos2: i32) -> i32 {
    (pos1 - pos2).abs()
}

fn part2_consumption(pos1: i32, pos2: i32) -> i32 {
    let mut acc = 0;
    let target = (pos1 - pos2).abs();
    for i in 0..=target {
        acc += i;
    }
    acc
}

fn fuel_consumption<F>(position: &[i32], pos: i32, func: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    position.iter().map(|&curr| func(pos, curr)).sum()
}

fn best_consumption<F>(positions: &[i32], func: F) -> i32
where
    F: Fn(i32, i32) -> i32 + Copy,
{    
    let (min, max) = positions
        .iter()
        .fold((i32::MAX, i32::MIN), |(min, max), &v| {
            (v.min(min), v.max(max))
        });
    // This whole thing is rather slow for part2,
    // could probably be optimized by expanding search around median
    let mut best_consumption: i32 = i32::MAX;
    for pos in min..=max {
        let consumption = fuel_consumption(positions, pos, func);
        if best_consumption > consumption {
            best_consumption = consumption;
        } else {
            return best_consumption;
        }
    }
    best_consumption
}

#[cfg(test)]
mod testing {
    use super::*;
    const DATA: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_parsing() {
        let result = parse_crab_positions(DATA);
        assert_eq!(result.len(), 10);
        assert_eq!(result[0], 16);
        assert_eq!(result[result.len() - 1], 14);
    }

    #[test]
    fn test_fueld_consumption() {
        let data = parse_crab_positions(DATA);
        assert_eq!(fuel_consumption(&data, 2, part1_consumption), 37);
        assert_eq!(fuel_consumption(&data, 1, part1_consumption), 41);
        assert_eq!(fuel_consumption(&data, 3, part1_consumption), 39);
        assert_eq!(fuel_consumption(&data, 10, part1_consumption), 71);
    }

    #[test]
    fn test_best_consumption() {
        let data = parse_crab_positions(DATA);
        assert_eq!(best_consumption(&data, part1_consumption), 37);
    }

    #[test]
    fn test_part2() {
        let data = parse_crab_positions(DATA);
        assert_eq!(fuel_consumption(&data, 2, part2_consumption), 206);
        assert_eq!(best_consumption(&data, part2_consumption), 168);
    }

    #[test]
    fn test_part2_consumption() {
        assert_eq!(part2_consumption(0, 0), 0);
        assert_eq!(part2_consumption(0, 1), 1);
        assert_eq!(part2_consumption(0, 2), 3);
        assert_eq!(part2_consumption(0, 3), 6);
        assert_eq!(part2_consumption(0, 4), 10);
        assert_eq!(part2_consumption(0, 5), 15);
    }
}
