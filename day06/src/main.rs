use std::{error::Error, fs, iter};

const FILENAME: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(FILENAME)?;
    let first_day = parse_fish(&input);

    println!("Part1 : {}", n_turns(&first_day, 80).len());
    
    let first_day = as_day_counts(first_day);
    println!("Part2 : {}", part2_n_turns(first_day, 256).iter().sum::<usize>());
    Ok(())
}

fn parse_fish(str: &str) -> Vec<usize> {
    str.split(',')
        .filter_map(|v| match v.parse() {
            Ok(v) => Some(v),
            Err(_) => None,
        })
        .collect()
}

fn turn(mut fish: Vec<usize>) -> Vec<usize> {
    let mut add_fish = 0;
    for days in fish.iter_mut() {
        *days = match *days {
            0 => {
                add_fish += 1;
                6
            }
            d => d - 1,
        }
    }
    fish.into_iter()
        .chain(iter::repeat(8).take(add_fish))
        .collect()
}

fn n_turns(fish: &[usize], n: usize) -> Vec<usize> {
    let mut fish = fish.to_vec();
    for _i in 0..n {
        fish = turn(fish);
    }
    fish
}

fn as_day_counts(fish: Vec<usize>) -> [usize;9] {
    let mut result = [0;9];
    for days in fish {
        result[days] += 1;
    }
    result
}

fn part2_turn(data: [usize;9]) -> [usize;9] {
    let mut new_data = [0; 9];
    for (i, count) in data.iter().enumerate() {
        if i == 0 {
            new_data[8] = *count;
            new_data[6] = *count;
        } else {
            new_data[i-1] += count;
        }
    }
    new_data
}

fn part2_n_turns(data: [usize;9], n: usize) -> [usize;9] {
    let mut data = data.to_owned();
    for _i in 0..n {
        data = part2_turn(data);
    }
    data
}

#[cfg(test)]
mod testing {
    use super::*;
    const DATA: &str = "3,4,3,1,2";

    #[test]
    fn test_parse_fish() {
        let result = parse_fish(DATA);
        assert_eq!(result, vec![3, 4, 3, 1, 2]);
    }

    #[test]
    fn test_few_turns() {
        let today = parse_fish(DATA);
        assert_eq!(today, vec![3, 4, 3, 1, 2]);
        let today = turn(today);
        assert_eq!(today, vec![2, 3, 2, 0, 1]);
        let today = turn(today);
        assert_eq!(today, vec![1, 2, 1, 6, 0, 8]);
        let today = turn(today);
        assert_eq!(today, vec![0, 1, 0, 5, 6, 7, 8]);
    }

    #[test]
    fn test_n_turns() {
        let first_day = parse_fish(DATA);
        let today = n_turns(&first_day, 18);
        assert_eq!(today.len(), 26);

        let today = n_turns(&first_day, 80);
        assert_eq!(today.len(), 5934);
    }
    
    #[test]
    fn test_as_days_count() {
        let first_day = parse_fish(DATA);
        let as_counts = as_day_counts(first_day);
        assert_eq!(as_counts, [0,1,1,2,1,0,0,0,0]);
    }
    
    #[test]
    fn test_part2_turns() {
        let first_day = parse_fish(DATA);
        let day1 = parse_fish("2,3,2,0,1");
        let as_counts = as_day_counts(first_day);
        let day1_counts = as_day_counts(day1);
        let day1 = part2_turn(as_counts);
        assert_eq!(day1_counts, day1);

        let day2_counts = as_day_counts(parse_fish("1,2,1,6,0,8"));
        let day2 = part2_turn(day1);
        assert_eq!(day2_counts, day2);

        let day3_counts = as_day_counts(parse_fish("0,1,0,5,6,7,8"));
        let day3 = part2_turn(day2);
        assert_eq!(day3_counts, day3);
    }
    
    #[test]
    fn test_part2() {
        let first_day = as_day_counts(parse_fish(DATA));
        let today = part2_n_turns(first_day, 18);
        assert_eq!(today.iter().sum::<usize>(), 26);
        let today = part2_n_turns(first_day, 80);
        assert_eq!(today.iter().sum::<usize>(), 5934);
        let today = part2_n_turns(first_day, 256);
        assert_eq!(today.iter().sum::<usize>(), 26984457539);        
    }
}
