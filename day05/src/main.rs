use std::{collections::HashMap, error::Error, fs};

const FILENAME: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(FILENAME)?;
    let lines = Line::parse_lines(&input);
    let mut map = Map::new();
    println!("Part1 : {}", map.part1(&lines));
    let mut map = Map::new();
    println!("Part2 : {}", map.part2(&lines));
    Ok(())
}

#[derive(Debug, Clone)]
struct Map {
    cells: HashMap<Point, u32>,
}

impl Map {
    fn new() -> Self {
        Self {
            cells: HashMap::new(),
        }
    }

    fn add_line(&mut self, line: &Line) {
        let mut x = line.p1.x;
        let mut y = line.p1.y;
       
        let dx = delta(line.p1.x, line.p2.x);
        let dy = delta(line.p1.y, line.p2.y);
        loop {
            let point = Point::new(x, y);
            self.cells.insert(point, self.cells.get(&point).unwrap_or(&0) + 1);
            x = ((x as i32) + dx) as u32;
            y = ((y as i32) + dy) as u32;
            if point == line.p2 {
                break;
            }
        }
    }
    
    fn part1(&mut self, lines : &[Line]) -> u32 {
        for line in lines {
            if line.is_cartesian() {
                self.add_line(line)
            }
        }
        self.cells.values().filter(| &&v| v >= 2_u32).count() as u32
    }

    fn part2(&mut self, lines : &[Line]) -> u32 {
        for line in lines {
            self.add_line(line)
        }
        self.cells.values().filter(| &&v| v >= 2_u32).count() as u32
    }
}

fn delta(v1: u32, v2: u32) -> i32 {
    match v1.cmp(&v2) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => -1,
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn parse(str: &str) -> Self {
        let mut tokens = str.trim().split(',');
        Self {
            x: tokens.next().unwrap().parse().unwrap(),
            y: tokens.next().unwrap().parse().unwrap(),
        }
    }
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn parse(str: &str) -> Self {
        let mut tokens = str.trim().split_whitespace();
        Self {
            p1: Point::parse(tokens.next().unwrap()),
            p2: Point::parse(tokens.nth(1).unwrap()),
        }
    }

    #[cfg(test)]
    fn new(x1: u32, y1: u32, x2: u32, y2: u32) -> Self {
        Self {
            p1: Point { x: x1, y: y1 },
            p2: Point { x: x2, y: y2 },
        }
    }

    fn parse_lines(str: &str) -> Vec<Self> {
        str.lines().map(Self::parse).collect()
    }

    fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }

    fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }

    fn is_cartesian(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    const DATA: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn parse_point() {
        let str = "12,132";
        let p = Point::parse(str);
        assert_eq!(p.x, 12);
        assert_eq!(p.y, 132);
    }
    #[test]
    fn parse_line() {
        let str = "8,0 -> 0,8";
        let line = Line::parse(str);
        assert_eq!(line.p1.x, 8);
        assert_eq!(line.p1.y, 0);
        assert_eq!(line.p2.x, 0);
        assert_eq!(line.p2.y, 8);
    }
    #[test]
    fn parse_lines() {
        let lines = Line::parse_lines(DATA);
        let line = &lines[0];
        assert_eq!(line.p1.x, 0);
        assert_eq!(line.p1.y, 9);
        assert_eq!(line.p2.x, 5);
        assert_eq!(line.p2.y, 9);
        let line = &lines[lines.len() - 1];
        assert_eq!(line.p1.x, 5);
        assert_eq!(line.p1.y, 5);
        assert_eq!(line.p2.x, 8);
        assert_eq!(line.p2.y, 2);
    }
    #[test]
    fn test_cartesian_line() {
        let line = Line::new(1, 2, 2, 6);
        assert!(!line.is_cartesian());
        let line = Line::new(1, 0, 1, 6);
        assert!(line.is_cartesian());
        let line = Line::new(0, 2, 8, 2);
        assert!(line.is_cartesian());
    }
    #[test]
    fn test_part1() {
        let lines = Line::parse_lines(DATA);
        let mut map = Map::new();
        let result = map.part1(&lines);
        println!("result: {}", result);
        assert_eq!(result, 5);
    }
    #[test]
    fn test_part2() {
        let lines = Line::parse_lines(DATA);
        let mut map = Map::new();
        let result = map.part2(&lines);
        println!("result: {}", result);
        assert_eq!(result, 12);
    }
}
