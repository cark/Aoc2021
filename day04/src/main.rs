use std::{collections::HashSet, error::Error, fs};

const FILENAME: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(FILENAME)?;
    let lines: Vec<&str> = input.lines().collect();
    let mut game = Game::parse(&lines);
    println!("Part1: {}", game.part1());
    let mut game = Game::parse(&lines);
    println!("Part2: {}", game.part2());
    Ok(())
}

const BOARD_WIDTH: usize = 5;

#[derive(Debug, Clone)]
struct Board {
    cells: [u32; BOARD_WIDTH * BOARD_WIDTH],
}

impl Board {
    fn parse(lines: &[&str]) -> Self {
        let mut cells = [0; BOARD_WIDTH * BOARD_WIDTH];
        for (y, &line) in lines.iter().enumerate() {
            if !line.is_empty() {
                for (x, value) in line
                    .split_whitespace()
                    .map(|str| str.parse::<u32>().expect("board cells should be u32"))
                    .enumerate()
                {
                    cells[Point::new(x, y).to_index()] = value;
                }
            }
        }
        Self { cells }
    }

    fn value(&self, index: usize) -> u32 {
        self.cells[index]
    }

    fn column(&self, index: usize) -> impl Iterator<Item = u32> + '_ {
        (0..BOARD_WIDTH)
            .into_iter()
            .map(move |y| self.value(Point::new(index, y).to_index()))
    }

    fn row(&self, index: usize) -> impl Iterator<Item = u32> + '_ {
        (0..BOARD_WIDTH)
            .into_iter()
            .map(move |x| self.value(Point::new(x, index).to_index()))
    }

    fn unmarked_sum(&self, marked: &HashSet<u32>) -> u32 {
        self.cells
            .into_iter()
            .filter(|v| !marked.contains(v))
            .sum()
    }

    fn is_winning(&self, marked: &HashSet<u32>) -> bool {
        for index in 0..BOARD_WIDTH {
            if iterator_is_winning(self.column(index), marked) {
                return true;
            }
        }
        for index in 0..BOARD_WIDTH {
            if iterator_is_winning(self.row(index), marked) {
                return true;
            }
        }
        false
    }

    fn display(&self, _marked: &HashSet<u32>) {
        for index in 0..BOARD_WIDTH {
            self.row(index).for_each(|item| print!("{:2} ", item));
            println!();
        }
        println!();
    }
}

fn iterator_is_winning(mut iterator: impl Iterator<Item = u32>, marked: &HashSet<u32>) -> bool {
    iterator.all(|v| marked.contains(&v))
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    fn to_index(&self) -> usize {
        self.x + self.y * BOARD_WIDTH
    }
}

#[derive(Debug)]
struct Game {
    boards: Vec<Board>,
    numbers: Vec<u32>,
    marked: HashSet<u32>,
    number_index: usize,
}

impl Game {
    fn parse(lines: &[&str]) -> Self {
        let mut lines_iter = lines.iter();
        let numbers = lines_iter
            .next()
            .expect("Numbers not found")
            .split(',')
            .map(|str| str.parse::<u32>().expect("every number should be a u32."))
            .collect();
        let boards = lines[2..].chunks(6).map(Board::parse).collect();
        Self {
            boards,
            numbers,
            marked: HashSet::new(),
            number_index: 0,
        }
    }

    fn turn(&mut self) {
        self.marked.insert(self.numbers[self.number_index]);
        self.number_index += 1;
    }

    fn winning_board_index(&self, boards: &[Board]) -> Option<usize> {
        boards
            .iter()
            .enumerate()
            .find(|(_, board)| board.is_winning(&self.marked))
            .map(|(i, _)| i)
    }

    fn last_number(&self) -> u32 {
        // panics on first turn
        self.numbers[self.number_index - 1]
    }

    fn part1(&mut self) -> u32 {
        loop {
            self.turn();
            if let Some(index) = self.winning_board_index(&self.boards) {
                let board = &self.boards[index];
                board.display(&self.marked);
                return board.unmarked_sum(&self.marked) * self.last_number();
            }
        }
    }

    fn part2(&mut self) -> u32 {
        let mut boards = self.boards.clone();
        let mut last_score = 0;
        while !boards.is_empty() {
            self.turn();
            while let Some(index) = self.winning_board_index(&boards) {
                let board = &boards[index];                
                last_score = board.unmarked_sum(&self.marked) * self.last_number();
                boards.remove(index);
            }            
        }
        last_score
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    const DATA: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    const NUMBERS: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1";
    const BOARD: &str = "22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
";
    #[test]
    fn test_parse_numbers() {
        let numbers: Vec<u32> = NUMBERS
            .split(',')
            .map(|str| str.parse::<u32>().expect("every number should be a u32."))
            .collect();
        assert_eq!(numbers[0], 7);
        assert_eq!(numbers[numbers.len() - 1], 1);
    }

    #[test]
    fn test_parse_board() {
        let lines: Vec<&str> = BOARD.lines().collect();
        let board = Board::parse(&lines);
        assert_eq!(board.value(Point::new(0, 0).to_index()), 22);
        assert_eq!(board.value(Point::new(4, 0).to_index()), 0);
        assert_eq!(board.value(Point::new(4, 4).to_index()), 19);
        assert_eq!(board.value(Point::new(0, 4).to_index()), 1);
    }

    #[test]
    fn test_parse_game() {
        let lines: Vec<&str> = DATA.lines().collect();
        let game = Game::parse(&lines);
        assert_eq!(game.boards.len(), 3);
        println!("{:?}", game);
        // Assert!(false);
    }

    #[test]
    fn test_board_row() {
        let lines: Vec<&str> = BOARD.lines().collect();
        let board = Board::parse(&lines);
        let row: Vec<u32> = board.row(0).collect();
        assert_eq!(vec![22, 13, 17, 11, 0], row);
        let row: Vec<u32> = board.row(4).collect();
        assert_eq!(vec![1, 12, 20, 15, 19], row);
    }

    #[test]
    fn test_board_col() {
        let lines: Vec<&str> = BOARD.lines().collect();
        let board = Board::parse(&lines);
        let row: Vec<u32> = board.column(0).collect();
        assert_eq!(vec![22, 8, 21, 6, 1], row);
        let row: Vec<u32> = board.column(4).collect();
        assert_eq!(vec![0, 24, 7, 5, 19], row);
    }

    #[test]
    fn test_board_winning() {
        let lines: Vec<&str> = BOARD.lines().collect();
        let board = Board::parse(&lines);
        let marked: HashSet<u32> = HashSet::from([22, 13, 17, 11, 0]);
        assert!(board.is_winning(&marked));
        let marked: HashSet<u32> = HashSet::from([1, 12, 20, 15, 19]);
        assert!(board.is_winning(&marked));
        let marked: HashSet<u32> = HashSet::from([22, 8, 21, 6, 1]);
        assert!(board.is_winning(&marked));
        let marked: HashSet<u32> = HashSet::from([0, 24, 7, 5, 19]);
        assert!(board.is_winning(&marked));
    }
    
    #[test]
    fn test_game() {
        let lines: Vec<&str> = DATA.lines().collect();
        let mut game = Game::parse(&lines);
        println!("{:#?}", game);
        loop {
            game.turn();
            if let Some(index) = game.winning_board_index(&game.boards) {
                let board = &game.boards[index];
                board.display(&game.marked);
                assert_eq!(
                    HashSet::from([7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24]),                    
                    game.marked
                );
                assert_eq!(board.unmarked_sum(&game.marked) * game.last_number(),4512);
                break;
            }
        }
    }
    #[test]
    fn test_part1() {
        let lines: Vec<&str> = DATA.lines().collect();
        let mut game = Game::parse(&lines);
        let result = game.part1();
        assert_eq!(result, 4512);
    }
    #[test]
    fn test_part2() {
        let lines: Vec<&str> = DATA.lines().collect();
        let mut game = Game::parse(&lines);
        let result = game.part2();
        assert_eq!(result, 1924);
    }
}
