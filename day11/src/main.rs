use std::fs::read_to_string;

const FILENAME: &str = "input.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_data = read_to_string(FILENAME)?;
    let grid = Grid::from_string(&file_data);
    let steps = grid.step_iter();
    let flashes: i32 = steps.skip(1).take(100).map(|g| g.flash_count()).sum();
    println!("Step 1 : {}", flashes);
    let steps = grid.step_iter();
    let (index, _) = steps
        .enumerate()
        .find(|(_, grid)| grid.is_flashing_synchronized())
        .unwrap();
    println!("Step 2 : {}", index);
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Grid {
    w: i32,
    h: i32,
    levels: Vec<i32>,
}

impl Grid {
    pub fn from_string(string: &str) -> Grid {
        let levels = string
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| {
                        c.to_digit(10)
                            .unwrap_or_else(|| panic!("digit required {}", c))
                            as i32
                    })
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();
        let w = levels.iter().map(Vec::len).max().unwrap() as i32;
        let h = levels.len() as i32;
        Grid {
            w,
            h,
            levels: levels.into_iter().flatten().collect(),
        }
    }

    pub fn sub_step1(&mut self) {
        self.levels.iter_mut().for_each(|level| *level += 1);
    }

    pub fn sub_step2(&mut self) {
        // we did dijkstra yesterday, let do it quick and dirty this time
        loop {
            let flashables = self.flashables();
            if flashables.is_empty() {
                break;
            } else {
                for pos in flashables {
                    *pos.level_mut(self) = 0;
                    let neighbors = pos
                        .neighbors(self)
                        .filter(|n| n.level(self) != 0)
                        .collect::<Vec<Pos>>();
                    for pos in neighbors.iter() {
                        *pos.level_mut(self) = pos.level(self) + 1;
                    }
                }
            }
        }
    }

    pub fn flash_count(&self) -> i32 {
        self.iter().filter(|p| p.level(self) == 0).count() as i32
    }

    pub fn is_flashing_synchronized(&self) -> bool {
        self.iter().all(|p| p.level(self) == 0)
    }

    pub fn flashables(&self) -> Vec<Pos> {
        self.iter().filter(|pos| pos.level(self) > 9).collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = Pos> + '_ {
        (0..self.levels.len()).filter_map(|index| Pos::from_index(self, index))
    }

    pub fn step_iter(&self) -> impl Iterator<Item = Grid> + '_ {
        GridStepIterator {
            current_grid: self.clone(),
        }
    }
}

#[derive(Debug)]
struct GridStepIterator {
    current_grid: Grid,
}

impl Iterator for GridStepIterator {
    type Item = Grid;

    fn next(&mut self) -> Option<Self::Item> {
        let mut new_grid = self.current_grid.clone();
        new_grid.sub_step1();
        new_grid.sub_step2();
        Some(std::mem::replace(&mut self.current_grid, new_grid))
    }
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(grid: &Grid, x: i32, y: i32) -> Option<Self> {
        if x >= 0 && y >= 0 && x < grid.w && y < grid.h {
            Some(Self { x, y })
        } else {
            None
        }
    }

    pub fn from_index(grid: &Grid, index: usize) -> Option<Self> {
        let x = index as i32 % grid.w;
        let y = index as i32 / grid.w;
        Self::new(grid, x, y)
    }

    pub fn as_index(&self, grid: &Grid) -> usize {
        (self.x + self.y * grid.w) as usize
    }

    pub fn level(&self, grid: &Grid) -> i32 {
        grid.levels[self.as_index(grid)]
    }

    pub fn level_mut<'a>(&'a self, grid: &'a mut Grid) -> &'a mut i32 {
        let index = self.as_index(grid);
        &mut grid.levels[index]
    }

    pub fn neighbors<'a>(&'a self, grid: &'a Grid) -> impl Iterator<Item = Pos> + '_ {
        const OFFSETS: [(i32, i32); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        OFFSETS
            .iter()
            .filter_map(move |(dx, dy)| Pos::new(grid, self.x + dx, self.y + dy))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_from_string() {
        let grid = Grid::from_string(TEXT);
        assert_eq!(grid.w, 10);
        assert_eq!(grid.h, 10);
        assert_eq!(grid.levels.len(), 100);
    }

    #[test]
    fn test_grid_eq() {
        let grid1 = Grid::from_string(TEXT);
        let grid2 = Grid::from_string(TEXT);
        let mut grid3 = Grid::from_string(TEXT);
        grid3.levels[0] = 0;
        assert_eq!(grid1, grid2);
        assert_ne!(grid1, grid3);
    }

    #[test]
    fn test_small_steps() {
        const SMALL_GRID_TEXTS: [&str; 3] = [
            "11111
19991
19191
19991
11111",
            "34543
40004
50005
40004
34543",
            "45654
51115
61116
51115
45654",
        ];
        let grids = SMALL_GRID_TEXTS
            .into_iter()
            .map(Grid::from_string)
            .collect::<Vec<Grid>>();
        assert_eq!(grids.len(), 3);
        grids.iter().for_each(|g| {
            assert_eq!(g.w, 5);
            assert_eq!(g.h, 5);
        });
        let iterated = grids[0].step_iter().take(3).collect::<Vec<Grid>>();
        assert_eq!(grids, iterated);
    }

    #[test]
    fn test_big_steps() {
        let grid = Grid::from_string(TEXT);
        let steps = grid.step_iter();
        let flashes: i32 = steps.skip(1).take(100).map(|g| g.flash_count()).sum();
        assert_eq!(flashes, 1656);
    }

    #[test]
    fn test_all_synchronized() {
        let grid = Grid::from_string(TEXT);
        let steps = grid.step_iter();
        let (index, _) = steps
            .enumerate()
            .find(|(_, grid)| grid.is_flashing_synchronized())
            .unwrap();
        assert_eq!(index, 195);
    }

    const TEXT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
}
