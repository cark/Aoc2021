//#![allow(dead_code)]

use std::collections::{BinaryHeap, HashMap};

fn main() {
    let board = Board::from_string(include_str!("input.txt"));
    println!("Part 1 : {}", board.a_star());
    println!("Part 2 : {}", BigBoard::from_board(board).a_star());
}

type Level = i32;
type Coord = (i32, i32);

trait TBoard {
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn level(&self, coord: Coord) -> i32;
    fn top_left_coord(&self) -> Coord {
        (0, 0)
    }
    fn bottom_right_coord(&self) -> Coord;

    fn a_star(&self) -> i32 {
        let dest = self.bottom_right_coord();
        let start = self.top_left_coord();

        let mut gscore: HashMap<Coord, i32> = HashMap::default();
        gscore.insert(start, 0);

        let mut open_list = BinaryHeap::new();
        open_list.push(CoordCost::new(start, 0));

        while let Some(CoordCost {
            data: current,
            cost: _curr_cost,
        }) = open_list.pop()
        {
            if current == dest {
                return gscore[&current];
            }
            [(0, -1), (-1, 0), (1, 0), (0, 1)]
                .iter()
                .filter_map(|(dx, dy)| {
                    let x = current.0 + dx;
                    let y = current.1 + dy;
                    if x >= 0 && x < self.width() && y >= 0 && y < self.height() {
                        Some((x, y))
                    } else {
                        None
                    }
                })
                .for_each(|neighbour| {
                    let new_score =
                        gscore.get(&current).unwrap_or(&i32::MAX) + self.level(neighbour);
                    if new_score < *gscore.get(&neighbour).unwrap_or(&i32::MAX) {
                        gscore.insert(neighbour, new_score);
                        if !open_list.iter().any(|mc| mc.data == neighbour) {
                            open_list.push(CoordCost::new(
                                neighbour,
                                new_score + manathan_distance(neighbour, dest),
                            ));
                        }
                    }
                })
        }
        unreachable!()
    }
}

#[inline]
fn manathan_distance(from: Coord, to: Coord) -> i32 {
    (from.0 - to.0).abs() + (from.1 - to.1).abs()
}

#[derive(Debug, Default)]
struct Board {
    levels: Vec<Level>,
    width: i32,
    height: i32,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct CoordCost {
    data: Coord,
    cost: i32,
}

impl PartialOrd for CoordCost {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for CoordCost {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl CoordCost {
    pub fn new(data: Coord, cost: i32) -> Self {
        Self { data, cost }
    }
}

impl TBoard for Board {
    #[inline]
    fn width(&self) -> i32 {
        self.width
    }

    #[inline]
    fn height(&self) -> i32 {
        self.height
    }

    #[inline]
    fn level(&self, coord: Coord) -> i32 {
        self.levels[(coord.0 + coord.1 * self.width()) as usize]
    }

    #[inline]
    fn bottom_right_coord(&self) -> Coord {
        (self.width as i32 - 1, self.height as i32 - 1)
    }
}

impl Board {
    fn from_string(text: &str) -> Board {
        text.trim()
            .lines()
            .map(str::trim)
            .map(|line| line.trim().chars().map(|c| c.to_digit(10).unwrap()))
            .enumerate()
            .fold(Board::default(), |mut board, (y, lines)| {
                if board.height <= y as i32 {
                    board.height = y as i32 + 1;
                }
                for (x, value) in lines.enumerate() {
                    if board.width <= x as i32 {
                        board.width = x as i32 + 1;
                    }
                    board.levels.push(value as i32);
                }
                board
            })
    }
}

struct BigBoard {
    board: Board,
}

impl BigBoard {
    fn from_board(board: Board) -> Self {
        BigBoard { board }
    }
    #[cfg(test)]
    fn from_string(text: &str) -> Self {
        Self::from_board(Board::from_string(text))
    }
}

impl TBoard for BigBoard {
    #[inline]
    fn width(&self) -> i32 {
        self.board.width() * 5
    }

    #[inline]
    fn height(&self) -> i32 {
        self.board.height() * 5
    }

    #[inline]
    fn level(&self, coord: Coord) -> i32 {
        let x_chunk = coord.0 / self.board.width();
        let y_chunk = coord.1 / self.board.height();
        let x = coord.0 % self.board.width();
        let y = coord.1 % self.board.height();
        (self.board.level((x, y)) - 1 + x_chunk + y_chunk) % 9 + 1
    }

    #[inline]
    fn bottom_right_coord(&self) -> Coord {
        (self.width() - 1, self.height() - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let board = Board::from_string(SAMPLE);
        assert_eq!(board.width(), 10);
        assert_eq!(board.height(), 10);
        assert_eq!(board.levels.len(), 100);
    }

    #[test]
    fn test_manathan_cmp() {
        assert!(CoordCost::new((5, 6), 1) < CoordCost::new((0, 0), 0));
        assert!(CoordCost::new((5, 6), 0) > CoordCost::new((1, 1), 1));
    }

    #[test]
    fn test_astar() {
        let board = Board::from_string(SAMPLE);
        assert_eq!(board.a_star(), 40);
    }

    #[test]
    fn test_big_board_from_string() {
        let board = BigBoard::from_string(SAMPLE);
        assert_eq!(board.width(), 50);
        assert_eq!(board.height(), 50);
        assert_eq!(board.bottom_right_coord(), (49, 49));
        assert_eq!(board.level((0, 0)), 1);
        assert_eq!(board.level((10, 0)), 2);
        assert_eq!(board.level((10, 10)), 3);
        assert_eq!(board.level((20, 10)), 4);
        assert_eq!(board.level((20, 20)), 5);
        assert_eq!(board.level((1, 1)), 3);
        assert_eq!(board.level((11, 1)), 4);
        assert_eq!(board.level((11, 11)), 5);
        assert_eq!(board.level((21, 11)), 6);
        assert_eq!(board.level((21, 21)), 7);
        assert_eq!(board.level((49, 49)), 9);
    }

    #[test]
    fn test_big_board_astar() {
        let board = BigBoard::from_string(SAMPLE);
        assert_eq!(board.a_star(), 315);
    }

    const SAMPLE: &str = include_str!("sample.txt");
}
