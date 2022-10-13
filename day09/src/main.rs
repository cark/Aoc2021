#![feature(assert_matches)]
#![feature(iter_collect_into)]

use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

const FILENAME: &str = "input.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_data = read_to_string(FILENAME)?;
    println!(
        "part1: {}",
        HeightMap::from_string(&file_data).risk_level_sum()
    );
    println!(
        "part2: {}",
        HeightMap::from_string(&file_data).top_basins_product()
    );
    Ok(())
}

#[derive(Debug)]
struct HeightMap {
    heights: Vec<u8>,
    w: i32,
    h: i32,
}

impl HeightMap {
    pub fn from_string(string: &str) -> HeightMap {
        let lines = string
            .split("\r\n")
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("digit expected") as u8)
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();
        let w = lines.iter().max_by_key(|a| a.len()).unwrap().len();
        let h = lines.len();
        HeightMap {
            heights: lines.into_iter().flatten().collect(),
            h: h as i32,
            w: w as i32,
        }
    }

    pub fn at(&self, x: i32, y: i32) -> Option<Pos> {
        if x >= 0 && x < self.w && y >= 0 && y < self.h {
            Some(Pos { x, y })
        } else {
            None
        }
    }

    pub fn at_index(&self, i: usize) -> Option<Pos> {
        if i < self.heights.len() {
            Some(Pos {
                x: (i as i32) % self.w,
                y: (i as i32) / self.w,
            })
        } else {
            None
        }
    }

    pub fn positions(&self) -> HeightMapIterator {
        HeightMapIterator {
            height_map: self,
            i: 0,
        }
    }

    pub fn low_points(&self) -> impl Iterator<Item = LowPoint> + '_ {
        self.positions()
            .filter_map(move |p| LowPoint::try_new(p, self))
    }

    pub fn risk_level_sum(&self) -> i32 {
        self.low_points().map(|lp| lp.risk_level(self)).sum()
    }

    pub fn top_basins_product(&self) -> i32 {
        let mut result = self
            .low_points()
            .map(|p| p.get_basin(self))
            .collect::<Vec<i32>>();
        result.sort();
        result.into_iter().rev().take(3).product()
    }
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn value(&self, hm: &HeightMap) -> u8 {
        hm.heights[(self.x + self.y * hm.w) as usize]
    }
    pub fn neighbors<'a>(&'a self, hm: &'a HeightMap) -> impl Iterator<Item = Pos> + 'a {
        const OFFSETS: [(i32, i32); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];
        OFFSETS
            .iter()
            .filter_map(|(dx, dy)| hm.at(self.x + dx, self.y + dy))
    }
    pub fn as_pair(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

#[derive(Debug)]
struct HeightMapIterator<'a> {
    height_map: &'a HeightMap,
    i: usize,
}

impl<'a> Iterator for HeightMapIterator<'a> {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.height_map.at_index(self.i);
        self.i += 1;
        result
    }
}

#[derive(Debug)]
struct LowPoint(Pos);

impl LowPoint {
    fn try_new(pos: Pos, hm: &HeightMap) -> Option<LowPoint> {
        let value = pos.value(hm);
        if pos.neighbors(hm).all(|neighbor| neighbor.value(hm) > value) {
            Some(LowPoint(pos))
        } else {
            None
        }
    }

    fn risk_level(self, hm: &HeightMap) -> i32 {
        self.0.value(hm) as i32 + 1
    }

    fn get_basin(&self, hm: &HeightMap) -> i32 {
        let mut found = HashSet::from([self.0.as_pair()]);
        let mut queue = VecDeque::from([BasinPoint::try_new(self.0, hm).unwrap()]);
        let mut result = 1;
        while let Some(current) = queue.pop_front() {
            for neighbor in current.0.neighbors(hm) {
                if let Some(bp) = BasinPoint::try_new(neighbor, hm) {
                    if !found.contains(&bp.0.as_pair()) {
                        result += 1;
                        found.insert(bp.0.as_pair());
                        queue.push_back(bp);
                    }
                }
            }
        }
        result
    }
}

#[derive(Debug, Clone, Copy)]
struct BasinPoint(Pos);

impl BasinPoint {
    fn try_new(pos: Pos, hm: &HeightMap) -> Option<BasinPoint> {
        if pos.value(hm) < 9 {
            Some(BasinPoint(pos))
        } else {
            None
        }
    }
}

// #[derive(Debug)]
// struct Basin<'a> {
//     points: Vec<BasinPoint<'a>>,
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;

    const TEST_DATA: &str = "2199943210\r\n\
         3987894921\r\n\
         9856789892\r\n\
         8767896789\r\n\
         9899965678";

    #[test]
    fn test_parsing() {
        let hm = HeightMap::from_string(TEST_DATA);
        assert_eq!(hm.w, 10);
        assert_eq!(hm.h, 5);
    }

    #[test]
    fn test_at() {
        let hm = HeightMap::from_string(TEST_DATA);
        assert_matches!(hm.at(0, 0), Some(_));
        assert_matches!(hm.at(9, 4), Some(_));
        assert_matches!(hm.at(10, 4), None);
        assert_matches!(hm.at(9, 5), None);
    }

    #[test]
    fn test_pos_value() {
        let hm = HeightMap::from_string(TEST_DATA);
        assert_matches!(hm.at(0, 0).map(|p| p.value(&hm)), Some(2));
        assert_matches!(hm.at(9, 4).map(|p| p.value(&hm)), Some(8));
        assert_matches!(hm.at(10, 4), None);
        assert_matches!(hm.at(9, 5), None);
    }

    #[test]
    fn test_at_index() {
        let hm = HeightMap::from_string(TEST_DATA);
        assert_matches!(hm.at_index(0).map(|p| p.value(&hm)), Some(2));
        assert_matches!(hm.at_index(49).map(|p| p.value(&hm)), Some(8));
        assert_matches!(hm.at_index(50), None);
        assert_matches!(hm.at_index(59), None);
    }

    #[test]
    fn test_height_map_iterator() {
        let hm = HeightMap::from_string(TEST_DATA);
        assert_eq!(hm.positions().count(), 50);
    }

    #[test]
    fn test_neighbors_iterator() {
        let hm = HeightMap::from_string(TEST_DATA);
        assert_eq!(hm.at(0, 0).unwrap().neighbors(&hm).count(), 2);
        assert_eq!(hm.at(1, 0).unwrap().neighbors(&hm).count(), 3);
        assert_eq!(hm.at(0, 3).unwrap().neighbors(&hm).count(), 3);
        assert_eq!(hm.at(3, 3).unwrap().neighbors(&hm).count(), 4);
        assert_eq!(hm.at(9, 0).unwrap().neighbors(&hm).count(), 2);
        assert_eq!(hm.at(9, 1).unwrap().neighbors(&hm).count(), 3);
        assert_eq!(hm.at(0, 4).unwrap().neighbors(&hm).count(), 2);
        assert_eq!(hm.at(9, 4).unwrap().neighbors(&hm).count(), 2);
        assert_eq!(hm.at(5, 4).unwrap().neighbors(&hm).count(), 3);
    }

    #[test]
    fn test_low_points() {
        let hm = HeightMap::from_string(TEST_DATA);
        assert_eq!(hm.low_points().count(), 4);
    }

    #[test]
    fn test_risk_level() {
        let hm = HeightMap::from_string(TEST_DATA);
        assert_matches!(
            LowPoint::try_new(hm.at(1, 0).unwrap(), &hm).map(|lp| lp.risk_level(&hm)),
            Some(2)
        );
        assert_matches!(
            LowPoint::try_new(hm.at(9, 0).unwrap(), &hm).map(|lp| lp.risk_level(&hm)),
            Some(1)
        );
        assert_matches!(
            LowPoint::try_new(hm.at(2, 2).unwrap(), &hm).map(|lp| lp.risk_level(&hm)),
            Some(6)
        );
        assert_matches!(
            LowPoint::try_new(hm.at(6, 4).unwrap(), &hm).map(|lp| lp.risk_level(&hm)),
            Some(6)
        );
    }

    #[test]
    fn test_risk_level_sum() {
        let hm = HeightMap::from_string(TEST_DATA);
        assert_eq!(hm.risk_level_sum(), 15);
    }

    #[test]
    fn test_basin_point_new() {
        let hm = HeightMap::from_string(TEST_DATA);
        assert_matches!(
            hm.at(0, 0).and_then(|p| BasinPoint::try_new(p, &hm)),
            Some(_)
        );
        assert_matches!(hm.at(2, 0).and_then(|p| BasinPoint::try_new(p, &hm)), None);
    }

    #[test]
    fn test_get_basin() {
        let hm = HeightMap::from_string(TEST_DATA);
        assert_eq!(
            hm.at(1, 0)
                .and_then(|p| LowPoint::try_new(p, &hm))
                .map(|lp| lp.get_basin(&hm)),
            Some(3)
        );
        assert_eq!(
            hm.at(9, 0)
                .and_then(|p| LowPoint::try_new(p, &hm))
                .map(|lp| lp.get_basin(&hm)),
            Some(9)
        );
        assert_eq!(
            hm.at(2, 2)
                .and_then(|p| LowPoint::try_new(p, &hm))
                .map(|lp| lp.get_basin(&hm)),
            Some(14)
        );
        assert_eq!(
            hm.at(6, 4)
                .and_then(|p| LowPoint::try_new(p, &hm))
                .map(|lp| lp.get_basin(&hm)),
            Some(9)
        );
    }

    #[test]
    fn test_top_basins_product() {
        let hm = HeightMap::from_string(TEST_DATA);
        assert_eq!(hm.top_basins_product(), 1134);
    }
}
