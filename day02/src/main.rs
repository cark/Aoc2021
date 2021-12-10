use std::{error::Error, fs};

const FILENAME: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(FILENAME)?;
    let moves: Vec<Move> = input.lines().map(Move::parse).collect();
    
    let mut pos = Pos::new();
    moves.iter().for_each(|m| pos.apply_move(m));
    println!("Part1: {}", pos.product());

    let mut pos = Pos2::new();
    moves.iter().for_each(|m| pos.apply_move(m));
    println!("Part2: {}", pos.product());

    Ok(())
}

enum Move {
    Fwd(i32),
    Up(i32),
    Down(i32),
}

impl Move {
    fn parse(str: &str) -> Self {
        let mut split = str.split(' ');
        let first = split.next().expect("There should be a first token");
        let second = split
            .next()
            .expect("There should be a second token")
            .parse::<i32>()
            .expect("second token should be a i32");
        match first {
            "forward" => Move::Fwd(second),
            "up" => Move::Up(second),
            "down" => Move::Down(second),
            _ => unreachable!("Unrecognized move {}", first),
        }
    }
}

struct Pos {
    horizontal: i32,
    depth: i32,
}

impl Pos {
    fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
        }
    }
    fn apply_move(&mut self, it: &Move) {
        match it {
            Move::Fwd(x) => self.horizontal += x,
            Move::Up(x) => self.depth -= x,
            Move::Down(x) => self.depth += x,
        }
    }

    fn product(&self) -> i32 {
        self.horizontal * self.depth
    }
}

struct Pos2 {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Pos2 {
    fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }
    fn apply_move(&mut self, it: &Move) {
        match it {
            Move::Fwd(x) => {
                self.horizontal += x;
                self.depth += x * self.aim;
            }
            Move::Up(x) => self.aim -= x,
            Move::Down(x) => self.aim += x,
        }
    }
    fn product(&self) -> i32 {
        self.horizontal * self.depth
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    const DATA: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
    #[test]
    fn test_sample() {
        let moves = DATA.lines().map(Move::parse);
        let mut pos = Pos::new();
        moves.for_each(|m| pos.apply_move(&m));
        assert_eq!(pos.product(), 150);
    }
    #[test]
    fn test_part2() {
        let moves = DATA.lines().map(Move::parse);
        let mut pos = Pos2::new();
        moves.for_each(|m| pos.apply_move(&m));
        assert_eq!(pos.product(), 900);
    }
}
