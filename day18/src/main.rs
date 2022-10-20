//#![allow(dead_code)]

fn main() {
    let homework = Homework::parse(include_str!("input.txt"));
    println!("Part 1 : {}", homework.sum().magnitude());
    println!("Part 2 : {}", homework.largest_magnitude_sum());
}

struct Homework {
    nums: Vec<SNum>,
}

impl Homework {
    fn parse(text: &str) -> Self {
        Homework {
            nums: text.trim().lines().map(SNum::parse).collect(),
        }
    }

    fn sum(&self) -> SNum {
        let mut result = self.nums.first().unwrap().clone();
        for other in self.nums[1..].iter() {
            result = result.reduced_sum(other);
        }
        result
    }

    fn largest_magnitude_sum(&self) -> u64 {
        let mut largest = 0;
        for (i, snum) in self.nums.iter().enumerate() {
            for (j, other) in self.nums.iter().enumerate() {
                if i != j {
                    largest = largest.max(snum.reduced_sum(other).magnitude());
                }
            }
        }
        largest
    }
}

#[derive(Clone)]
struct SNum {
    values: Vec<Value>,
}

impl SNum {
    fn parse(text: &str) -> Self {
        let mut curr_depth = 0;
        let mut values = vec![];
        for c in text.trim().chars() {
            match c {
                '[' => curr_depth += 1,
                ']' => curr_depth -= 1,
                ',' => {}
                c => values.push(Value {
                    num: c.to_digit(10).unwrap().into(),
                    depth: curr_depth,
                }),
            }
        }
        assert!(curr_depth == 0);
        SNum { values }
    }

    fn sum(&self, other: &Self) -> Self {
        SNum {
            values: self
                .values
                .iter()
                .copied()
                .map(Value::inc_depth)
                .chain(other.values.iter().copied().map(Value::inc_depth))
                .collect(),
        }
    }

    fn explode(&mut self) -> bool {
        if let Some((explode_at, _)) = self
            .values
            .iter()
            .enumerate()
            .find(|(_, val)| val.depth > 4)
        {
            let left = self.values[explode_at];
            let right = self.values[explode_at + 1];
            if explode_at > 0 {
                self.values[explode_at - 1].num += left.num;
            }
            let next_index = explode_at + 2;
            if next_index < self.values.len() {
                self.values[next_index].num += right.num;
            }
            self.values[explode_at] = Value {
                num: 0,
                depth: left.depth - 1,
            };
            self.values.remove(explode_at + 1);
            true
        } else {
            false
        }
    }

    fn split(&mut self) -> bool {
        if let Some((split_at, value)) = self.values.iter().enumerate().find(|(_, val)| val.num > 9)
        {
            let left = Value {
                depth: value.depth + 1,
                num: value.num / 2,
            };
            let right = Value {
                depth: value.depth + 1,
                num: (value.num + 1) / 2,
            };
            self.values[split_at] = left;
            self.values.insert(split_at + 1, right);
            true
        } else {
            false
        }
    }

    fn reduce(mut self) -> Self {
        while self.explode() || self.split() {}
        self
    }

    fn reduced_sum(&self, other: &Self) -> Self {
        self.sum(other).reduce()
    }

    fn magnitude(&self) -> u64 {
        fn recurse(vals: &[Value], depth: u8) -> (u64, &[Value]) {
            if let Some((val, rest)) = vals.split_first() {
                if depth < val.depth {
                    let (left, rest) = recurse(vals, depth + 1);
                    let (right, rest) = recurse(rest, depth + 1);
                    (3 * left + 2 * right, rest)
                } else {
                    (val.num, rest)
                }
            } else {
                unreachable!()
            }
        }
        recurse(&self.values, 0).0
    }
}

#[derive(Debug, Clone, Copy)]
struct Value {
    num: u64,
    depth: u8,
}

impl Value {
    fn inc_depth(mut self) -> Self {
        self.depth += 1;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl SNum {
        fn as_string(&self) -> String {
            fn recurse<'a, 'b>(
                vals: &'a [Value],
                depth: u8,
                string: &'b mut String,
            ) -> &'a [Value] {
                if let Some((val, rest)) = vals.split_first() {
                    if depth < val.depth {
                        string.push('[');
                        let result = recurse(vals, depth + 1, string);
                        string.push(',');
                        let result = recurse(result, depth + 1, string);
                        string.push(']');
                        result
                    } else {
                        string.push_str(&format!("{}", val.num));
                        rest
                    }
                } else {
                    unreachable!()
                }
            }
            let mut result = String::with_capacity(100);
            recurse(&self.values, 0, &mut result);
            result
        }
    }

    #[test]
    fn test_parse() {
        assert_eq!(SNum::parse("[1,2]").as_string(), "[1,2]");
        assert_eq!(
            SNum::parse("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]")
                .as_string(),
            "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]"
        );
    }
    #[test]
    fn test_sum() {
        assert_eq!(
            SNum::parse("[1,2]")
                .sum(&SNum::parse("[[3,4],5]"))
                .as_string(),
            "[[1,2],[[3,4],5]]"
        );
    }

    #[test]
    fn test_identity_reduction() {
        assert!(
            !SNum::parse("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]").explode()
        );
    }

    #[test]
    fn test_explode() {
        let mut snum = SNum::parse("[[[[[9,8],1],2],3],4]");
        assert!(snum.explode());
        assert_eq!(
            snum.as_string(),
            SNum::parse("[[[[0,9],2],3],4]").as_string()
        );
        let mut snum = SNum::parse("[7,[6,[5,[4,[3,2]]]]]");
        assert!(snum.explode());
        assert_eq!(
            snum.as_string(),
            SNum::parse("[7,[6,[5,[7,0]]]]").as_string()
        );
        let mut snum = SNum::parse("[[6,[5,[4,[3,2]]]],1]");
        assert!(snum.explode());
        assert_eq!(
            snum.as_string(),
            SNum::parse("[[6,[5,[7,0]]],3]").as_string()
        );
        let mut snum = SNum::parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        assert!(snum.explode());
        assert_eq!(
            snum.as_string(),
            SNum::parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").as_string()
        );
        let mut snum = SNum::parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        assert!(snum.explode());
        assert_eq!(
            snum.as_string(),
            SNum::parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]").as_string()
        );
    }

    #[test]
    fn test_split() {
        let mut snum = SNum::parse("[1,1]");
        snum.values[1].num = 10;
        assert!(snum.split());
        assert_eq!(snum.as_string(), SNum::parse("[1,[5,5]]").as_string());
        let mut snum = SNum::parse("[1,1]");
        snum.values[0].num = 11;
        assert!(snum.split());
        assert_eq!(snum.as_string(), SNum::parse("[[5,6],1]").as_string());
    }

    #[test]
    fn test_reduce() {
        assert_eq!(
            SNum::parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")
                .reduce()
                .as_string(),
            SNum::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").as_string()
        )
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(SNum::parse("[9,1]").magnitude(), 29);
        assert_eq!(SNum::parse("[[9,1],[1,9]]").magnitude(), 129);
    }

    #[test]
    fn test_example() {
        let homework = Homework::parse(EXAMPLE);
        assert_eq!(homework.sum().magnitude(), 4140);
    }

    #[test]
    fn test_largest_magnitude() {
        let homework = Homework::parse(EXAMPLE);
        assert_eq!(homework.largest_magnitude_sum(), 3993);
    }

    const EXAMPLE: &str = include_str!("example.txt");
}
