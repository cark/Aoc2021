use std::{error::Error, fs, io::Error as IOError};

use itertools::Itertools;

const FILENAME: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let file_data = load_file(FILENAME)?;
    let data: Vec<i32> = parse(&file_data).collect();
    let count = increase_count(pairs(data.iter().copied()));
    println!("Part1 = {}", count);
    let count = increase_count(pairs(triplets(data.into_iter()).map(|(a, b, c)| a+b+c)));
    println!("Part2 = {}", count);
    Ok(())
}

fn parse(data: &str) -> impl Iterator<Item = i32> + '_ {
    data.lines()
        .map(|string| string.parse().expect("every value must be an integer"))
}

fn load_file(filename: &str) -> Result<String, IOError> {
    fs::read_to_string(filename)
}

fn pairs(iter : impl Iterator<Item = i32>) -> impl Iterator<Item = (i32, i32)> {
    iter.tuple_windows()
}

fn triplets(iter : impl Iterator<Item = i32>) -> impl Iterator<Item = (i32, i32, i32)> {
    iter.tuple_windows()
}

fn increase_count(iter : impl Iterator<Item = (i32, i32)>) -> usize {
    iter.filter(|(a, b)| b > a).count()
}

#[cfg(test)]
mod testing {
    use super::*;
    const DATA1: &str = "199
200
208
210
200
207
240
269
260
263";
    #[test]
    fn read_data() {
        assert_eq!(parse(DATA1).count(), 10);
    }
    #[test]
    fn read_file() {
        parse(&load_file(FILENAME).unwrap()).count();            
    }
    #[test]
    fn tuples() {
        let data = vec![1,2,3,4];
        let mut pairs = pairs(data.into_iter());
        assert_eq!(pairs.next(), Some((1,2)));
        assert_eq!(pairs.next(), Some((2,3)));
        assert_eq!(pairs.next(), Some((3,4)));
        assert_eq!(pairs.next(), None);
    }
    #[test]
    fn test_solution() {
        let count = increase_count(pairs(parse(DATA1)));
        assert_eq!(count, 7);        
    }
    #[test]
    fn triplets_test() {
        let data = vec![1,2,3,4];
        let mut triples = triplets(data.into_iter());
        assert_eq!(triples.next(), Some((1,2,3)));
        assert_eq!(triples.next(), Some((2,3,4)));
        assert_eq!(triples.next(), None);
    }
    #[test]
    fn part2_test() {
        let count = increase_count(pairs(triplets(parse(DATA1)).map(|(a, b, c)| a+b+c)));
        assert_eq!(count, 5);
    }
}
