// #![allow(dead_code)]

const INPUT: &str = "target area: x=288..330, y=-96..-50";

fn main() {
    let target = Target::from_string(INPUT);
    println!("Part1 : {}", find_highest_y(&target));
    println!("Part2 : {}", solution_count(&target));
}

#[derive(Debug, Eq, PartialEq)]
pub struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Target {
    x_range: (i32, i32),
    y_range: (i32, i32),
}

impl Target {
    pub fn from_string(text: &str) -> Target {
        let mut coords = text.trim().split(' ').skip(2).map(|str| {
            let mut coord_els = str
                .split('=')
                .nth(1)
                .unwrap()
                .split(',')
                .next()
                .unwrap()
                .split("..");
            (
                coord_els.next().unwrap().parse::<i32>().unwrap(),
                coord_els.next().unwrap().parse::<i32>().unwrap(),
            )
        });
        Target {
            x_range: coords.next().unwrap(),
            y_range: coords.next().unwrap(),
        }
    }
    #[inline(always)]
    fn in_y_range(&self, y: i32) -> bool {
        y >= self.y_range.0 && y <= self.y_range.1
    }
    #[inline(always)]
    fn in_x_range(&self, x: i32) -> bool {
        x >= self.x_range.0 && x <= self.x_range.1
    }
}

fn max_dy(target: &Target) -> i32 {
    // - For any positive dy we eventually get back to y = 0.
    // - At that point our y speed would -dy.
    // - The next step will take us to: -dy - 1
    // - The biggest step we can take at that moment is one that will
    //   reach the bottom of the Target.
    // - That step will have: bottom_y = -dy - 1
    // - So we can solve for dy:
    //    -dy = bottom_y + 1
    //     dy = -(bottom_y + 1) // abstracted in max_dy
    // - In our example that value would be -(-10 + 1) = 9
    -(target.y_range.0 + 1)
}

fn find_highest_y(target: &Target) -> i32 {
    // - max = sum(range(max_dy))
    // - in our example that's 9 + 8 + 7 + 6 + 5 + 4 + 3 + 2 + 1 = 45
    // - That's an arithmetic serie so the result is  (n + 1) * (v(n) + v(1)) / 2
    let n = max_dy(target);
    (n + 1) * n / 2 // better than my first idea : (1..=max_dy(target)).sum()
}

fn min_dy(target: &Target) -> i32 {
    // Like in max_dy, we need to reach lowest point in a single step
    target.y_range.0
}

fn min_dx(target: &Target) -> i32 {
    //   |
    // v |\
    //   | \
    //   |  \
    //   | e \
    //   +------
    //        t
    // e = vt/2
    // In ou problem v = t because for each step we lose one speed
    // e = v²/2
    // v = sqrt(2e)
    // Looks like truncated is good enough for discrete movement ?
    ((target.x_range.0 * 2) as f32).sqrt() as i32
}

fn max_dx(target: &Target) -> i32 {
    target.x_range.1
}

fn simulate(target: &Target, mut dx: i32, mut dy: i32) -> bool {
    let mut pos_x = 0;
    let mut pos_y = 0;
    while pos_x <= target.x_range.1 && pos_y >= target.y_range.0 {
        pos_x += dx;
        pos_y += dy;
        if target.in_x_range(pos_x) && target.in_y_range(pos_y) {
            return true;
        }
        dx = (dx - 1).max(0);
        dy -= 1;
    }
    false
}

fn solution_count(target: &Target) -> i32 {
    let mut result = 0;
    for dx in min_dx(target)..=max_dx(target) {
        for dy in min_dy(target)..=max_dy(target) {
            if simulate(target, dx, dy) {
                result += 1;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let target = Target::from_string(SAMPLE);
        assert_eq!(
            target,
            Target {
                x_range: (20, 30),
                y_range: (-10, -5),
            }
        );
    }

    #[test]
    fn test_find_highest_y() {
        let target = Target::from_string(SAMPLE);
        assert_eq!(find_highest_y(&target), 45);
    }

    #[test]
    fn test_min_dx() {
        let target = Target::from_string(SAMPLE);
        assert_eq!(min_dx(&target), 6);
    }

    #[test]
    fn test_max_dx() {
        let target = Target::from_string(SAMPLE);
        assert_eq!(max_dx(&target), 30);
    }

    #[test]
    fn test_min_dy() {
        let target = Target::from_string(SAMPLE);
        assert_eq!(min_dy(&target), -10);
    }

    #[test]
    fn test_max_dy() {
        let target = Target::from_string(SAMPLE);
        assert_eq!(max_dy(&target), 9);
    }

    #[test]
    fn test_part2() {
        let target = Target::from_string(SAMPLE);
        assert_eq!(solution_count(&target), 112);
    }

    #[test]
    fn test_simulate() {
        let target = Target::from_string(SAMPLE);
        assert!(simulate(&target, 30, -7));
        assert!(simulate(&target, 9, -2));
        assert!(simulate(&target, 25, -9));
    }

    #[test]
    fn test_in_x_range() {
        let target = Target::from_string(SAMPLE);
        assert!(target.in_x_range(20));
        assert!(target.in_x_range(30));
        assert!(!target.in_x_range(19));
        assert!(!target.in_x_range(31));
    }
    #[test]
    fn test_in_y_range() {
        let target = Target::from_string(SAMPLE);
        assert!(target.in_y_range(-10));
        assert!(target.in_y_range(-5));
        assert!(!target.in_y_range(-11));
        assert!(!target.in_y_range(-4));
    }
    const SAMPLE: &str = "target area: x=20..30, y=-10..-5";
}
