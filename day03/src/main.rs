use std::{error::Error, fs};

const FILENAME: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(FILENAME)?;
    let data: Vec<&str> = input.lines().collect();
    let bit_stats = BitStats::parse(&data);
    let (gamma_rate, epsilon_rate) = part1(&bit_stats);

    println!("part1 : {}", gamma_rate * epsilon_rate);

    let (oxy, co2) = part2(&bit_stats);
    println!("part2 : {}", oxy * co2);
    
    Ok(())
}

struct BitStats {
    bits: Vec<u32>,
    values: Vec<u32>,
}

impl BitStats {
    fn parse(data: &[&str]) -> Self {
        let mut result = Self {
            bits: Vec::new(),
            values: Vec::new(),
        };
        for &line in data {
            let mut value = 0;
            for (order, char) in line.chars().enumerate() {
                if result.bits.len() < order + 1 {
                    result.bits.resize(order + 1, 0);
                }
                value <<= 1;
                if char == '1' {
                    result.bits[order] += 1;
                    value += 1 ;
                }                
            }
            result.values.push(value);
        }
        result
    }

    fn mask(&self) -> u32 {
        let mut result = 0;
        for i in 0..self.bits.len() {
            result += 1_u32 << i;
        }
        result
    }

    // order is 0 for left-most bit
    fn most_common_bit(&self, order: usize, prefer_bit: bool) -> u32 {
        match self.bits[order].cmp(&(self.values.len() as u32 / 2)) {
            std::cmp::Ordering::Less => 0,
            std::cmp::Ordering::Equal => match prefer_bit {
                true => 1,
                false => unreachable!("Nothing was said for equal values"),
            },
            std::cmp::Ordering::Greater => 1,
        }
    }
}

fn part1(bit_stats: &BitStats) -> (u32, u32) {
    let mut gamma_rate: u32 = 0;
    for order in 0..bit_stats.bits.len() {
        gamma_rate += bit_stats.most_common_bit(bit_stats.bits.len() - order - 1, false) << order as u32;
    }
    let epsilon_rate: u32 = !gamma_rate & bit_stats.mask();
    (gamma_rate, epsilon_rate)
}


fn match_shift(value: u32, shift: usize) -> bool {
    ((value >> shift) & 1) == 1
}

fn count_bits(in_vec: &[u32], shift: usize) -> u32 {    
    in_vec.iter().filter(|&v| match_shift(*v, shift)).count() as u32
}

fn most_common(in_vec: &[u32], shift: usize) -> u32 {
    println!("yoh {} {}", count_bits(in_vec, shift), (in_vec.len() as u32 / 2));
    match (count_bits(in_vec, shift) as f32).partial_cmp(&(in_vec.len() as f32 / 2.0)) {
        Some(a) => match a {
            std::cmp::Ordering::Less => 0,
            std::cmp::Ordering::Equal => 1,
            std::cmp::Ordering::Greater => 1,
        },
        None => todo!(),
    }
}

fn invert_bit(bit: u32) -> u32 {
    !bit & 1
}

fn part2(bit_stats: &BitStats) -> (u32, u32) {
    // oxygen-rating
    let mut values = bit_stats.values.clone();
    let mut order = 0;
    while values.len() > 1 {
        let shift = bit_stats.bits.len() - order - 1;        
        let most_common = most_common(&values, shift);   //bit_stats.most_common_bit(order, true);
        values = values.into_iter().filter(|&v| ((v >> shift) & 1) == most_common).collect();            
        order += 1;
    }
    let oxy = values[0];
    
    // co2-rating
    let mut values = bit_stats.values.clone();
    let mut order = 0;
    while values.len() > 1 {
        let shift = bit_stats.bits.len() - order - 1;        
        let most_common = invert_bit(most_common(&values, shift));
        values = values.into_iter().filter(|&v| ((v >> shift) & 1) == most_common).collect();            
        order += 1;
    }
    let co2 = values[0];

    (oxy,co2)
}

#[cfg(test)]
mod testing {
    use super::*;
    const DATA: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn part1_test() {
        let data: Vec<&str> = DATA.lines().collect();
        let bit_stats = BitStats::parse(&data);
        let (gamma_rate, epsilon_rate) = part1(&bit_stats);
        assert_eq!(gamma_rate, 22);
        assert_eq!(epsilon_rate, 9);
        assert_eq!(gamma_rate * epsilon_rate, 198);
    }

    #[test]
    fn check_reversed() {
        let a = "11110";
        let mut reversed = a.chars().rev().enumerate();
        assert_eq!(reversed.next(), Some((0, '0')));
        assert_eq!(reversed.next(), Some((1, '1')));
        assert_eq!(reversed.next(), Some((2, '1')));
        assert_eq!(reversed.next(), Some((3, '1')));
        assert_eq!(reversed.next(), Some((4, '1')));
        assert_eq!(reversed.next(), None);
    }
    #[test]
    fn part2_test() {
        let data: Vec<&str> = DATA.lines().collect();
        let bit_stats = BitStats::parse(&data);
        let (oxy, co2) = part2(&bit_stats);
        
        // let (gamma_rate, epsilon_rate) = part1(&bit_stats);
        assert_eq!(oxy, 23);
        assert_eq!(co2, 10);
        // assert_eq!(epsilon_rate, 9);
        // assert_eq!(gamma_rate * epsilon_rate, 198);
    }
    #[test]
    fn test_match_shift() {
        assert_eq!(match_shift(0b1000,3), true);
        assert_eq!(match_shift(0b1000,4), false);
        assert_eq!(match_shift(0b1000,2), false);
        assert_eq!(match_shift(0b1000,1), false);
        assert_eq!(match_shift(0b1000,0), false);
        assert_eq!(match_shift(0b0100,3), false);
        assert_eq!(match_shift(0b0100,4), false);
        assert_eq!(match_shift(0b0100,2), true);
        assert_eq!(match_shift(0b0100,1), false);
        assert_eq!(match_shift(0b0100,0), false);
    }
    #[test]
    fn test_count_bits() {
        assert_eq!(count_bits(&[0b1000, 0b0111, 0b1001], 3), 2);
    }
    #[test]
    fn test_most_common() {
        assert_eq!(most_common(&[0b1000, 0b0111, 0b1001, 0b0011], 3), 1);
        assert_eq!(most_common(&[0b1000, 0b0111, 0b1001, 0b0011], 2), 0);
        assert_eq!(most_common(&[0b1000, 0b0111, 0b1001, 0b0011], 1), 1);
        assert_eq!(most_common(&[0b1000, 0b0111, 0b1001, 0b0011], 0), 1);
        assert_eq!(count_bits(&[0b11110,
                                 0b10110,
                                 0b10111,
                                 0b10101,
                                 0b11100,
                                 0b10000,
                                 0b11001,], 3), 3);
        assert_eq!(most_common(&[0b11110,
                                 0b10110,
                                 0b10111,
                                 0b10101,
                                 0b11100,
                                 0b10000,
                                 0b11001,]
                               ,3),0)
        // assert_eq!(most_common(&[0b1000, 0b0111, 0b1001, 0b0011], 3), 1);
    }
}
