use std::{error::Error, fs};

const FILENAME: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(FILENAME)?;
    let first_day = parse_fish(&input);

    println!("Part1 : {}", sum(&n_turns(first_day, 80)));
    println!("Part2 : {}", sum(&n_turns(first_day, 256)));

    Ok(())
}

type FreqArray = [usize; 9];

fn parse_fish(str: &str) -> FreqArray {
    let mut result = [0; 9];
    str.split(',')
        .filter_map(|v| match v.parse() {
            Ok(v) => Some(v),
            Err(_) => None,
        })
        .for_each(|v: usize| result[v] += 1);
    result
}

fn turn(data: FreqArray) -> FreqArray {
    let mut new_data = [0; 9];
    for (i, count) in data.iter().enumerate() {
        if i == 0 {
            new_data[8] = *count;
            new_data[6] = *count;
        } else {
            new_data[i - 1] += count;
        }
    }
    new_data
}

fn n_turns(data: FreqArray, n: usize) -> FreqArray {
    let mut data = data.to_owned();
    for _ in 0..n {
        data = turn(data);
    }
    data
}

fn sum(data: &FreqArray) -> usize {
    data.iter().sum()
}

#[cfg(test)]
mod testing {
    use super::*;
    const DATA: &str = "3,4,3,1,2";

    #[test]
    fn test_n_turns() {
        let first_day = parse_fish(DATA);
        let today = n_turns(first_day, 18);
        assert_eq!(sum(&today), 26);

        let today = n_turns(first_day, 80);
        assert_eq!(sum(&today), 5934);
    }

    #[test]
    fn test_parse_fish() {
        let first_day = parse_fish(DATA);
        assert_eq!(first_day, [0, 1, 1, 2, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn test_part2_turns() {
        let first_day = parse_fish(DATA);
        
        let day1_counts = parse_fish("2,3,2,0,1");
        let day1 = turn(first_day);
        assert_eq!(day1_counts, day1);

        let day2_counts = parse_fish("1,2,1,6,0,8");
        let day2 = turn(day1);
        assert_eq!(day2_counts, day2);

        let day3_counts = parse_fish("0,1,0,5,6,7,8");
        let day3 = turn(day2);
        assert_eq!(day3_counts, day3);
    }

    #[test]
    fn test_part2() {
        let first_day = parse_fish(DATA);
        let today = n_turns(first_day, 18);
        assert_eq!(sum(&today), 26);
        let today = n_turns(first_day, 80);
        assert_eq!(sum(&today), 5934);
        let today = n_turns(first_day, 256);
        assert_eq!(sum(&today), 26984457539);
    }
}
