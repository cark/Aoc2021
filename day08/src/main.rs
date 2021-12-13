use std::{collections::HashMap, error::Error, fs};

const FILENAME: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(FILENAME)?;
    let samples = parse_samples(&input);
    println!("Part1 : {}", part1(&samples));
    println!("Part2 : {}", part2(&samples));
    Ok(())
}

fn part1(samples: &[Sample]) -> usize {
    samples
        .iter()
        .map(|s| {
            s.display
                .iter()
                .filter(|&&d| is_unique_count_digit(d))
                .count()
        })
        .sum()
}

fn part2(samples: &[Sample]) -> u32 {
    samples
        .iter()
        .map(|s| {
            let segments_to_digit = s.infer_segments_to_digit();
            s.decode_display(segments_to_digit)
        })
        .sum()
}

const MASK: u8 = 0b01111111;

fn bit_count(mut value: u8) -> u8 {
    let mut count = 0;
    for _ in 0..8 {
        count += value & 1;
        value >>= 1;
    }
    count
}

fn unique_count_to_segments(count: u8) -> Option<u8> {
    match count {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None,
    }
}

fn is_unique_count_digit(digit: u8) -> bool {
    unique_count_to_segments(bit_count(digit)).is_some()
}

fn char_to_shift(char: char) -> u8 {
    ((char as u32) - ('a' as u32)) as u8
}

fn parse_segments(string: &str) -> u8 {
    let mut result = 0;
    string
        .trim()
        .chars()
        .for_each(|char| result += 1 << char_to_shift(char));
    result
}

fn parse_all_segments(string: &str) -> Vec<u8> {
    let result: Vec<u8> = string
        .trim()
        .split_whitespace()
        .map(parse_segments)
        .collect();
    debug_assert!(result.len() == 10);
    result
}

fn parse_display_segments(string: &str) -> Vec<u8> {
    let result: Vec<u8> = string
        .trim()
        .split_whitespace()
        .map(parse_segments)
        .collect();
    debug_assert!(result.len() == 4);
    result
}

fn parse_samples(string: &str) -> Vec<Sample> {
    string.trim().lines().map(Sample::parse).collect()
}

pub struct Sample {
    //    segments: Vec<u8>,
    display: Vec<u8>,
    digits: Vec<Digit>,
}

impl Sample {
    fn parse(string: &str) -> Self {
        let mut itr = string.trim().split('|');
        let segments = parse_all_segments(itr.next().unwrap());
        let display = parse_display_segments(itr.next().unwrap());
        let digits = segments
            .iter()
            .copied()
            .enumerate()
            .map(|(index, segments)| Digit::new(index, segments))
            .collect();
        Self {
            //            segments,
            display,
            digits,
        }
    }

    fn digit_indexes_for_length(&self, length: u8) -> Vec<usize> {
        self.digits
            .iter()
            .enumerate()
            .filter_map(|(index, digit)| {
                if bit_count(digit.segments) == length {
                    Some(index as usize)
                } else {
                    None
                }
            })
            .collect()
    }

    fn decode_display(&self, segments_to_digit: HashMap<u8, u8>) -> u32 {
        let mut result: u32 = 0;
        for display_digit_segments in &self.display {
            result *= 10;
            result += *segments_to_digit.get(display_digit_segments).unwrap() as u32;
        }
        result
    }

    fn infer_segments_to_digit(&self) -> HashMap<u8, u8> {
        // first we find digits with a unique number of segments
        let index = self.digit_indexes_for_length(2)[0];
        let mut digit1 = self.digits[index];
        digit1.digit = Some(1);

        let index = self.digit_indexes_for_length(3)[0];
        let mut digit7 = self.digits[index];
        digit7.digit = Some(7);

        let index = self.digit_indexes_for_length(4)[0];
        let mut digit4 = self.digits[index];
        digit4.digit = Some(4);

        let index = self.digit_indexes_for_length(7)[0];
        let mut digit8 = self.digits[index];
        digit8.digit = Some(8);

        // we now know digits 1 7 4 8
        // the segment in 7 and not in 1 is a
        let a = digit7.segments & !digit1.segments & MASK;
        debug_assert!(bit_count(a) == 1);

        // 8 ^ (7 | 4) is either e or g => the one missing in any digit with 5 segments is e, the other is g.
        let e_or_g = digit8.segments ^ (digit7.segments | digit4.segments);
        debug_assert!(bit_count(e_or_g) == 2);
        let e = {
            let mut result = 0;
            for index in self.digit_indexes_for_length(5) {
                let segments = self.digits[index].segments;
                let bit_left = e_or_g & !segments;
                if bit_count(bit_left) == 1 {
                    result = bit_left;
                    break;
                }
            }
            result
        };
        debug_assert!(bit_count(e) == 1);
        let g = e_or_g & !e;
        debug_assert!(bit_count(e) == 1);

        // if any of a e g is missing from a 6 segment digit, the digit is 9
        let indexes = self.digit_indexes_for_length(6);
        let &index_of9 = indexes
            .iter()
            .find(|&&index| {
                let not_segments = !self.digits[index].segments;
                (a & not_segments != 0) || (e & not_segments != 0) || (g & not_segments != 0)
            })
            .unwrap();
        let mut digit9 = self.digits[index_of9];
        digit9.digit = Some(9);
        debug_assert!(digit9.segment_count == 6);

        // we know digits 1 4 7 8 9 and segments a e g
        // considering 6 segments (besides 9) , when they're or'ed with 7, and a segment is missing,
        // the missing segment is d, and the digit is 0. It follows that the missing segment in the
        // other one is c, digit 6
        let indexes = indexes
            .into_iter()
            .filter(|&index| index != index_of9)
            .collect::<Vec<usize>>();
        debug_assert!(indexes.len() == 2);
        let mut found_index = 0;
        let mut missing_segment = 0;
        for (i, _) in indexes.iter().enumerate() {
            found_index = i;
            missing_segment = !(self.digits[indexes[i]].segments | digit7.segments) & MASK;
            if missing_segment != 0 {
                break;
            }
        }
        let d = missing_segment;
        let mut digit0 = self.digits[indexes[found_index]];
        digit0.digit = Some(0);
        debug_assert!(digit0.segment_count == 6);

        found_index = (found_index as i32 - 1).abs() as usize;
        debug_assert!(found_index == 0 || found_index == 1);

        let mut digit6 = self.digits[indexes[found_index]];
        digit6.digit = Some(6);
        let c = !digit6.segments & MASK;

        // we can find 2 with acdeg ... we now have 0 1 2 4 6 7 8 9
        let index_of2 = self
            .digits
            .iter()
            .enumerate()
            .find(|(_, digit)| digit.segments == a + c + d + e + g)
            .unwrap()
            .0;

        let mut digit2 = self.digits[index_of2];
        digit2.digit = Some(2);
        debug_assert!(digit2.segment_count == 5);

        // of the two left 3 has a c and 5 is the last
        let found_indexes = [
            digit0, digit1, digit2, digit4, digit6, digit7, digit8, digit9,
        ]
        .iter()
        .map(|digit| digit.index)
        .collect::<Vec<usize>>();
        let missing_indexes = (0..10)
            .filter(|i| !found_indexes.contains(i))
            .collect::<Vec<usize>>();
        debug_assert!(missing_indexes.len() == 2);
        let &index_of3 = missing_indexes
            .iter()
            .find(|&&i| self.digits[i].segments & c != 0)
            .unwrap();
        let mut digit3 = self.digits[index_of3];
        digit3.digit = Some(3);
        debug_assert!(digit3.segment_count == 5);
        let index_of5 = missing_indexes
            .into_iter()
            .find(|&i| i != index_of3)
            .unwrap();
        let mut digit5 = self.digits[index_of5];
        digit5.digit = Some(5);
        debug_assert!(digit5.segment_count == 5);

        let segments_to_digit: HashMap<u8, u8> = HashMap::from([
            (digit0.segments, 0),
            (digit1.segments, 1),
            (digit2.segments, 2),
            (digit3.segments, 3),
            (digit4.segments, 4),
            (digit5.segments, 5),
            (digit6.segments, 6),
            (digit7.segments, 7),
            (digit8.segments, 8),
            (digit9.segments, 9),
        ]);

        segments_to_digit
    }
}

#[derive(Clone, Copy)]
pub struct Digit {
    index: usize,
    digit: Option<u8>,
    segment_count: u8,
    segments: u8,
}

impl Digit {
    fn new(index: usize, segments: u8) -> Self {
        Self {
            index,
            segments,
            digit: None,
            segment_count: bit_count(segments),
        }
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    const DATA: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    const LINE: &str =
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part2() {
        let input = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
cdfeb fcadb cdfeb cdbaf";
        let sample = Sample::parse(input);
        let segments_to_digit = sample.infer_segments_to_digit();
        let result = sample.decode_display(segments_to_digit);
        assert_eq!(result, 5353);

        let samples = parse_samples(DATA);
        let result = part2(&samples);
        assert_eq!(result, 61229);
    }

    #[test]
    fn test_part1() {
        let samples = parse_samples(DATA);
        let count: usize = samples
            .iter()
            .map(|s| {
                s.display
                    .iter()
                    .filter(|&&d| is_unique_count_digit(d))
                    .count()
            })
            .sum();
        assert_eq!(count, 26);
    }

    #[test]
    fn test_parse_samples() {
        let samples = parse_samples(DATA);
        assert_eq!(samples.len(), 10);
    }

    #[test]
    fn test_parse_sample() {
        let _sample = Sample::parse(LINE);
    }

    const DIGIT_SEGMENTS: [u8; 10] = [
        0b01110111, 0b00100100, 0b01011101, 0b01101101, 0b00101110, 0b01101011, 0b01111011,
        0b00100101, 0b01111111, 0b01101111,
    ];

    #[test]
    fn test_parse_segments() {
        let segments = [
            "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
        ];
        for i in 0..10 {
            assert_eq!(parse_segments(segments[i]), DIGIT_SEGMENTS[i]);
        }
    }

    #[test]
    fn test_bit_count() {
        let counts = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];
        for i in 0..10 {
            assert_eq!(bit_count(DIGIT_SEGMENTS[i]), counts[i]);
        }
    }
}
