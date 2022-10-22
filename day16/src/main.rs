//#![allow(dead_code)]

// No allocation! ... WEEE !

use std::marker::PhantomData;

fn main() {
    let input = include_str!("input.txt");
    println!("Part1 : {}", Reader::new(input).version_sum());
    println!("Part2 : {}", Reader::new(input).eval());
}

#[derive(Clone, Copy)]
struct Reader<'a> {
    text: &'a str,
    bit_index: usize,
    hex_index: usize,
    hex: Option<u8>,
}

impl<'a> Reader<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            text,
            bit_index: 0,
            hex: None,
            hex_index: usize::MAX,
        }
    }

    #[inline(always)]
    fn hex_at(&mut self, string_index: usize) -> u8 {
        if self.hex_index == string_index {
            self.hex
        } else {
            self.hex_index = string_index;
            self.hex = self
                .text
                .as_bytes()
                .get(string_index)
                .and_then(|b| char::from_u32(*b as u32))
                .and_then(|c| c.to_digit(16))
                .map(|c| c as u8);
            self.hex
        }
        .unwrap()
    }

    fn next_bits(&mut self, bit_count: u8) -> u64 {
        let mut result: u64 = 0;
        let mut bits_left = bit_count;
        while bits_left > 0 {
            result <<= 1;
            let (hex_index, bit_index) = (self.bit_index >> 2, self.bit_index & 0b11);
            let bit = ((self.hex_at(hex_index) >> (3 - bit_index)) & 1) as u64;
            result |= bit;
            self.bit_index += 1;
            bits_left -= 1
        }
        result
    }

    fn version_sum(self) -> u64 {
        Walker::<VersionSum>::new(self).walk()
    }

    fn eval(self) -> u64 {
        Walker::<Evaluator>::new(self).walk()
    }
}

struct Walker<'a, W: WalkReducer + ?Sized> {
    reader: Reader<'a>,
    reducer: PhantomData<W>,
}

impl<'a, W: WalkReducer> Walker<'a, W> {
    fn new(reader: Reader<'a>) -> Self {
        Walker {
            reader,
            reducer: PhantomData::<W>,
        }
    }

    fn walk(&mut self) -> u64 {
        self.walk_packet()
    }

    fn walk_packet(&mut self) -> u64 {
        let version = self.reader.next_bits(3);
        let type_id = self.reader.next_bits(3);

        match type_id {
            4 => W::process_packet(self, version, 4, None),
            _ => {
                let state = OpPacketsState::new(self);
                W::process_packet(self, version, type_id, Some(state))
            }
        }
    }

    fn literal_groups_val(&mut self) -> u64 {
        let mut result = 0;
        loop {
            result <<= 4;
            let group = self.reader.next_bits(5);
            result += group & 0b1111;
            if group & 0b10000 == 0 {
                break result;
            }
        }
    }
}

trait WalkReducer {
    fn process_packet(
        walker: &mut Walker<'_, Self>,
        version: u64,
        type_id: u64,
        op_packet_state: Option<OpPacketsState>,
    ) -> u64;
}

struct VersionSum;

impl WalkReducer for VersionSum {
    fn process_packet(
        walker: &mut Walker<'_, Self>,
        version: u64,
        type_id: u64,
        op_packet_state: Option<OpPacketsState>,
    ) -> u64 {
        match type_id {
            4 => {
                walker.literal_groups_val();
                version
            }
            _ => op_packet_state
                .unwrap()
                .fold(walker, version, u64::wrapping_add),
        }
    }
}

struct Evaluator;

impl WalkReducer for Evaluator {
    fn process_packet(
        walker: &mut Walker<'_, Self>,
        _version: u64,
        type_id: u64,
        op_packet_state: Option<OpPacketsState>,
    ) -> u64 {
        match type_id {
            0 => op_packet_state.unwrap().fold(walker, 0, u64::wrapping_add),
            1 => op_packet_state.unwrap().fold(walker, 1, u64::wrapping_mul),
            2 => op_packet_state.unwrap().fold(walker, u64::MAX, u64::min),
            3 => op_packet_state.unwrap().fold(walker, 0, u64::max),
            4 => walker.literal_groups_val(),
            5 => u64::from(walker.walk_packet() > walker.walk_packet()),
            6 => u64::from(walker.walk_packet() < walker.walk_packet()),
            7 => u64::from(walker.walk_packet() == walker.walk_packet()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum OpPacketsState {
    Type0 { bit_count: usize, bits: usize },
    Type1 { packet_count: usize, packets: usize },
}

impl OpPacketsState {
    fn new<W: WalkReducer>(walker: &'_ mut Walker<W>) -> Self {
        if walker.reader.next_bits(1) == 0 {
            OpPacketsState::Type0 {
                bit_count: walker.reader.next_bits(15) as usize,
                bits: 0,
            }
        } else {
            OpPacketsState::Type1 {
                packet_count: walker.reader.next_bits(11) as usize,
                packets: 0,
            }
        }
    }

    fn next<W: WalkReducer>(&mut self, walker: &'_ mut Walker<W>) -> Option<u64> {
        match self {
            OpPacketsState::Type0 { bits, bit_count } => {
                if bits < bit_count {
                    let current_bit = walker.reader.bit_index;
                    let result = walker.walk_packet();
                    *bits += walker.reader.bit_index - current_bit;
                    Some(result)
                } else {
                    None
                }
            }
            OpPacketsState::Type1 {
                packets,
                packet_count,
            } => {
                if packets < packet_count {
                    *packets += 1;
                    Some(walker.walk_packet())
                } else {
                    None
                }
            }
        }
    }

    fn fold<F: Fn(u64, u64) -> u64, W: WalkReducer>(
        &mut self,
        walker: &'_ mut Walker<W>,
        mut init: u64,
        f: F,
    ) -> u64 {
        while let Some(val) = self.next(walker) {
            init = f(init, val);
        }
        init
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_at() {
        let mut r = Reader::new("D2FE28");
        assert_eq!(r.hex_at(0), 13);
        assert_eq!(r.hex_at(1), 2);
        assert_eq!(r.hex_at(2), 15);
        assert_eq!(r.hex_at(5), 8);
    }

    #[test]
    fn test_next_bits() {
        let mut r = Reader::new("D2FE28");
        assert_eq!(r.next_bits(3), 6);
        assert_eq!(r.next_bits(3), 4);
        assert_eq!(r.next_bits(5), 0b10111);
        assert_eq!(r.next_bits(5), 0b11110);
        assert_eq!(r.next_bits(5), 0b00101);
        assert_eq!(r.next_bits(3), 0);
    }

    #[test]
    fn test_version_sum() {
        assert_eq!(Reader::new("D2FE28").version_sum(), 6);
        assert_eq!(Reader::new("8A004A801A8002F478").version_sum(), 16);
        assert_eq!(Reader::new("620080001611562C8802118E34").version_sum(), 12);
        assert_eq!(
            Reader::new("C0015000016115A2E0802F182340").version_sum(),
            23
        );
        assert_eq!(
            Reader::new("A0016C880162017C3686B18A3D4780").version_sum(),
            31
        );
    }

    #[test]
    fn test_eval() {
        assert_eq!(Reader::new("C200B40A82").eval(), 3);
        assert_eq!(Reader::new("04005AC33890").eval(), 54);
        assert_eq!(Reader::new("9C0141080250320F1802104A08").eval(), 1);
    }
}
