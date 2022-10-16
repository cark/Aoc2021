//#![allow(dead_code)]

use std::collections::HashSet;
use std::fmt::Display;
use std::fs::read_to_string;
use std::rc::Rc;

const FILENAME: &str = "input.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_data = read_to_string(FILENAME)?;
    let problem = Problem::from_string(&file_data);
    println!(
        "Part 1 : {}",
        problem.apply_instruction().unwrap().points.len()
    );
    println!("Part 2 :\n{}", problem.iter().last().unwrap());
    Ok(())
}

#[derive(Clone)]
struct Problem {
    points: HashSet<(i32, i32)>,
    instructions: Vec<Instruction>,
}

#[derive(Clone, Copy)]
enum Instruction {
    Vertical(i32),
    Horizontal(i32),
}

impl Problem {
    pub fn from_string(text: &str) -> Self {
        let text = text.trim();
        let points = text
            .lines()
            .map(str::trim)
            .take_while(|&line| !line.is_empty())
            .map(|line| {
                let mut parts = line.split(',');
                (
                    parts.next().unwrap().parse::<i32>().unwrap(),
                    parts.next().unwrap().parse::<i32>().unwrap(),
                )
            })
            .collect::<HashSet<(i32, i32)>>();
        let instructions = text
            .lines()
            .map(str::trim)
            .skip_while(|&line| !line.is_empty())
            .skip(1)
            .map(Instruction::from_string)
            .collect::<Vec<Instruction>>();
        Problem {
            points,
            instructions,
        }
    }

    pub fn apply_instruction(&self) -> Option<Self> {
        if let Some(&instruction) = self.instructions.first() {
            let points = match instruction {
                Instruction::Horizontal(y) => fold(&self.points, y, |pair| &mut pair.1),
                Instruction::Vertical(x) => fold(&self.points, x, |pair| &mut pair.0),
            };
            Some(Problem {
                points,
                instructions: self.instructions[1..].to_vec(),
            })
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Rc<Self>> + '_ {
        ProblemIterator {
            prob: Some(Rc::new(self.clone())),
        }
    }
}

struct ProblemIterator {
    prob: Option<Rc<Problem>>,
}

impl Iterator for ProblemIterator {
    type Item = Rc<Problem>;

    fn next(&mut self) -> Option<Self::Item> {
        self.prob = self
            .prob
            .take()
            .and_then(|prob| prob.apply_instruction())
            .map(Rc::new);
        self.prob.clone()
    }
}

fn fold_around(fold_pos: i32, pos: i32) -> i32 {
    if pos > fold_pos {
        fold_pos - (pos - fold_pos)
    } else {
        pos
    }
}

fn fold(
    points: &HashSet<(i32, i32)>,
    fold_pos: i32,
    val: impl Fn(&mut (i32, i32)) -> &mut i32,
) -> HashSet<(i32, i32)> {
    let mut result = HashSet::default();
    for point in points {
        let mut new_point = *point;
        let pos = val(&mut new_point);
        *pos = fold_around(fold_pos, *pos);
        result.insert(new_point);
    }
    result
}

impl Instruction {
    pub fn from_string(text: &str) -> Self {
        let mut relevant = text.split(' ').last().unwrap().split('=');
        let direction = relevant.next().unwrap();
        let value = relevant.next().unwrap().parse::<i32>().unwrap();
        match direction {
            "x" => Instruction::Vertical(value),
            "y" => Instruction::Horizontal(value),
            _ => panic!("invalid instruction"),
        }
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (max_x, max_y) = self
            .points
            .iter()
            .fold((0, 0), |(mx, my), (x, y)| (mx.max(*x), my.max(*y)));
        let mut lines = vec![vec!['.'; max_x as usize + 1]; max_y as usize + 1];
        for (x, y) in self.points.iter() {
            lines[*y as usize][*x as usize] = '#';
        }
        f.write_str(
            &lines
                .into_iter()
                .map(|v| v.into_iter().collect())
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_from_string() {
        let prob = Problem::from_string(SAMPLE);
        assert_eq!(prob.points.len(), 18);
        assert_eq!(prob.instructions.len(), 2);
    }

    #[test]
    fn test_fold_pos() {
        assert_eq!(fold_around(7, 10), 4);
    }

    #[test]
    fn test_fold_once() {
        let prob = Problem::from_string(SAMPLE);
        let new_prob = prob.apply_instruction();
        assert!(matches!(new_prob, Some(_)));
        assert_eq!(new_prob.as_ref().unwrap().points.len(), 17);
        let new_prob = new_prob.unwrap().apply_instruction();
        assert!(matches!(new_prob, Some(_)));
        assert_eq!(new_prob.unwrap().points.len(), 16);
    }

    #[test]
    fn test_iter_and_display() {
        let result = Problem::from_string(SAMPLE).iter().last().unwrap();
        assert_eq!(
            format!("{}", result),
            "#####
#...#
#...#
#...#
#####"
        );
    }

    const SAMPLE: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
}
